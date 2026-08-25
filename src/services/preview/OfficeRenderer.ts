import DOMPurify from 'dompurify';
import JSZip from 'jszip';
import mammoth from 'mammoth';
import type { FileInfo, PreviewContent } from '../../types/file';
import { readBinaryFile, toArrayBuffer } from './helpers';
import type { PreviewRenderer } from './types';

const wordExtensions = new Set(['docx']);
const powerpointExtensions = new Set(['pptx']);
const drawingNamespace = 'http://schemas.openxmlformats.org/drawingml/2006/main';

export class OfficeRenderer implements PreviewRenderer {
  readonly id = 'office';

  canHandle(file: FileInfo) {
    return wordExtensions.has(file.extension) || powerpointExtensions.has(file.extension);
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const bytes = await readBinaryFile(file.path);
    if (wordExtensions.has(file.extension)) return this.renderWord(bytes);
    return this.renderPowerPoint(bytes);
  }

  private async renderWord(bytes: Uint8Array): Promise<PreviewContent> {
    const result = await mammoth.convertToHtml({ arrayBuffer: toArrayBuffer(bytes) });
    return {
      kind: 'office',
      officeType: 'word',
      html: DOMPurify.sanitize(result.value, { USE_PROFILES: { html: true } }),
    };
  }

  private async renderPowerPoint(bytes: Uint8Array): Promise<PreviewContent> {
    const archive = await JSZip.loadAsync(bytes);
    const slidePaths = Object.keys(archive.files)
      .filter((path) => /^ppt\/slides\/slide\d+\.xml$/.test(path))
      .sort((left, right) => Number(left.match(/\d+/)?.[0]) - Number(right.match(/\d+/)?.[0]));
    const slides = await Promise.all(
      slidePaths.map(async (path) => {
        const xml = await archive.file(path)?.async('text');
        if (!xml) return [];
        const document = new DOMParser().parseFromString(xml, 'application/xml');
        return Array.from(document.getElementsByTagNameNS(drawingNamespace, 't'))
          .map((element) => element.textContent?.trim() ?? '')
          .filter(Boolean);
      }),
    );
    return { kind: 'office', officeType: 'powerpoint', slides };
  }
}
