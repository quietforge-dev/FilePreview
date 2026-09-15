import type { FileInfo, PreviewContent } from '../../types/file';
import { imageMimeType, readBinaryFile, toArrayBuffer } from './helpers';
import type { PreviewRenderer } from './types';

export class ImageRenderer implements PreviewRenderer {
  readonly id = 'image';

  canHandle(file: FileInfo) {
    return Boolean(imageMimeType(file.extension));
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const bytes = await readBinaryFile(file.path);
    const mimeType = imageMimeType(file.extension);
    if (!mimeType) throw new Error('不支持的图片格式');
    return {
      kind: 'image',
      url: URL.createObjectURL(new Blob([toArrayBuffer(bytes)], { type: mimeType })),
    };
  }
}
