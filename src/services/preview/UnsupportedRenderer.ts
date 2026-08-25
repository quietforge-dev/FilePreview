import type { FileInfo, PreviewContent } from '../../types/file';
import type { PreviewRenderer } from './types';

export class UnsupportedRenderer implements PreviewRenderer {
  readonly id = 'unsupported';

  canHandle() {
    return true;
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const extension = file.extension ? `.${file.extension}` : '此类';
    return { kind: 'unsupported', message: `${extension} 文件暂不支持预览` };
  }
}
