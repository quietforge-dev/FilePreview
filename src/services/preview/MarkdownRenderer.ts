import DOMPurify from 'dompurify';
import { marked } from 'marked';
import type { FileInfo, PreviewContent } from '../../types/file';
import { readTextFile } from './helpers';
import type { PreviewRenderer } from './types';

const extensions = new Set(['md', 'markdown', 'mdx']);

export const renderMarkdownSource = async (source: string) => {
  const html = await marked.parse(source, { async: true, gfm: true, breaks: false });
  return DOMPurify.sanitize(html, { USE_PROFILES: { html: true } });
};

export class MarkdownRenderer implements PreviewRenderer {
  readonly id = 'markdown';

  canHandle(file: FileInfo) {
    return extensions.has(file.extension);
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const source = await readTextFile(file.path);
    return {
      kind: 'markdown',
      source,
      html: await renderMarkdownSource(source),
    };
  }
}
