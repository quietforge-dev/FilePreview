import type { FileInfo, PreviewContent } from '../../types/file';
import { readTextFile } from './helpers';
import type { PreviewRenderer } from './types';

const textExtensions = new Set([
  'txt',
  'json',
  'yaml',
  'yml',
  'xml',
  'toml',
  'ini',
  'env',
  'sql',
  'js',
  'ts',
  'tsx',
  'vue',
  'rs',
  'java',
  'kt',
  'go',
  'py',
  'sh',
  'css',
  'scss',
  'html',
  'csv',
  'log',
]);

export class TextRenderer implements PreviewRenderer {
  readonly id = 'text';

  canHandle(file: FileInfo) {
    return textExtensions.has(file.extension) || !file.extension;
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    return {
      kind: 'text',
      content: await readTextFile(file.path),
      language: file.extension || 'text',
    };
  }
}
