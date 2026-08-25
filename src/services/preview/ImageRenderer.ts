import type { FileInfo, PreviewContent } from '../../types/file';
import { readBinaryFile, toArrayBuffer } from './helpers';
import type { PreviewRenderer } from './types';

const mimeTypes: Record<string, string> = {
  gif: 'image/gif',
  jpeg: 'image/jpeg',
  jpg: 'image/jpeg',
  png: 'image/png',
  svg: 'image/svg+xml',
  webp: 'image/webp',
};

export class ImageRenderer implements PreviewRenderer {
  readonly id = 'image';

  canHandle(file: FileInfo) {
    return file.extension in mimeTypes;
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const bytes = await readBinaryFile(file.path);
    return {
      kind: 'image',
      url: URL.createObjectURL(
        new Blob([toArrayBuffer(bytes)], { type: mimeTypes[file.extension] }),
      ),
    };
  }
}
