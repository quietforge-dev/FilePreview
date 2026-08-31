import DOMPurify from 'dompurify';
import { marked, Renderer, type Token, type Tokens } from 'marked';
import type { FileInfo, MarkdownHeading, PreviewContent } from '../../types/file';
import { readTextFile } from './helpers';
import type { PreviewRenderer } from './types';

const extensions = new Set(['md', 'markdown', 'mdx']);

export interface MarkdownDocument {
  html: string;
  headings: MarkdownHeading[];
}

const headingId = (text: string, index: number) => {
  const slug = text
    .toLowerCase()
    .trim()
    .replace(/[^\p{L}\p{N}\s-]/gu, '')
    .replace(/[\s-]+/g, '-');
  return `heading-${index}-${slug || 'section'}`;
};

const headingText = (tokens: Token[]): string =>
  tokens
    .map((token) => {
      if ('tokens' in token && Array.isArray(token.tokens)) return headingText(token.tokens);
      return 'text' in token && typeof token.text === 'string' ? token.text : '';
    })
    .join('');

export const renderMarkdownDocument = async (source: string): Promise<MarkdownDocument> => {
  const headings: MarkdownHeading[] = [];
  const renderer = new Renderer();
  renderer.heading = ({ tokens, depth }: Tokens.Heading) => {
    const text = headingText(tokens).trim();
    const id = headingId(text, headings.length + 1);
    headings.push({ id, depth, text: text || `标题 ${headings.length + 1}` });
    return `<h${depth} id="${id}">${renderer.parser.parseInline(tokens)}</h${depth}>\n`;
  };
  const html = await marked.parse(source, { async: true, gfm: true, breaks: false, renderer });
  return {
    headings,
    html: DOMPurify.sanitize(html, { USE_PROFILES: { html: true } }),
  };
};

export const renderMarkdownSource = async (source: string) =>
  (await renderMarkdownDocument(source)).html;

export class MarkdownRenderer implements PreviewRenderer {
  readonly id = 'markdown';

  canHandle(file: FileInfo) {
    return extensions.has(file.extension);
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const source = await readTextFile(file.path);
    const document = await renderMarkdownDocument(source);
    return {
      kind: 'markdown',
      source,
      ...document,
    };
  }
}
