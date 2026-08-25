import type { FileInfo, PreviewContent } from '../../types/file';

export interface PreviewRenderer {
  readonly id: string;
  canHandle(file: FileInfo): boolean;
  render(file: FileInfo): Promise<PreviewContent>;
}
