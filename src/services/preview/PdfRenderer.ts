import type { FileInfo, PreviewContent } from '../../types/file';
import { readBinaryFile } from './helpers';
import type { PreviewRenderer } from './types';

export class PdfRenderer implements PreviewRenderer {
  readonly id = 'pdf';

  canHandle(file: FileInfo) {
    return file.extension === 'pdf';
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    return { kind: 'pdf', data: await readBinaryFile(file.path) };
  }
}
