import type { FileInfo, PreviewContent } from '../../types/file';
import { convertOfficeToPdf, getOfficeRuntimeStatus } from '../../api/office';
import type { PreviewRenderer } from './types';

const officeExtensions = new Set(['doc', 'docx', 'ppt', 'pptx', 'xls', 'xlsx']);

export class OfficeRenderer implements PreviewRenderer {
  readonly id = 'office';

  canHandle(file: FileInfo) {
    return officeExtensions.has(file.extension);
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const runtime = await getOfficeRuntimeStatus();
    if (!runtime.installed) {
      return {
        kind: 'office-unavailable',
        message: '预览 Office 文件需要 LibreOffice。本文件不会上传或修改。',
      };
    }
    return { kind: 'pdf', data: await convertOfficeToPdf(file.path) };
  }
}
