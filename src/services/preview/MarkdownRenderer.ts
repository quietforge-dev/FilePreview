import DOMPurify from 'dompurify';
import { marked, Renderer, type Token, type Tokens } from 'marked';
import type { FileInfo, MarkdownHeading, PreviewContent } from '../../types/file';
import {
  fileExtension,
  imageMimeType,
  readBinaryFile,
  readTextFile,
  toArrayBuffer,
} from './helpers';
import { localResourcePath } from './localResourcePath';
import type { PreviewRenderer } from './types';

const extensions = new Set(['md', 'markdown', 'mdx']);

export interface MarkdownDocument {
  html: string;
  headings: MarkdownHeading[];
  objectUrls: string[];
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

const resolveLocalImages = async (html: string, documentPath: string, objectUrls: string[]) => {
  const template = document.createElement('template');
  template.innerHTML = html;
  const images = [...template.content.querySelectorAll<HTMLImageElement>('img[src]')];

  await Promise.all(
    images.map(async (image) => {
      const source = image.getAttribute('src') ?? '';
      const path = localResourcePath(documentPath, source);
      if (!path) return;
      const mimeType = imageMimeType(fileExtension(path));
      if (!mimeType) return;

      try {
        const bytes = await readBinaryFile(path);
        const objectUrl = URL.createObjectURL(new Blob([toArrayBuffer(bytes)], { type: mimeType }));
        objectUrls.push(objectUrl);
        image.src = objectUrl;
      } catch {
        // 单张本地图片不可用时保留原始地址和替代文本，不阻止 Markdown 正文预览。
      }
    }),
  );

  return template.innerHTML;
};

export const renderMarkdownDocument = async (
  source: string,
  documentPath?: string,
): Promise<MarkdownDocument> => {
  const headings: MarkdownHeading[] = [];
  const objectUrls: string[] = [];
  try {
    const renderer = new Renderer();
    renderer.heading = ({ tokens, depth }: Tokens.Heading) => {
      const text = headingText(tokens).trim();
      const id = headingId(text, headings.length + 1);
      headings.push({ id, depth, text: text || `标题 ${headings.length + 1}` });
      return `<h${depth} id="${id}">${renderer.parser.parseInline(tokens)}</h${depth}>\n`;
    };
    const rendered = await marked.parse(source, {
      async: true,
      gfm: true,
      breaks: false,
      renderer,
    });
    const sanitized = DOMPurify.sanitize(rendered, { USE_PROFILES: { html: true } });
    const html = documentPath
      ? await resolveLocalImages(sanitized, documentPath, objectUrls)
      : sanitized;
    return { headings, html, objectUrls };
  } catch (error) {
    objectUrls.forEach((url) => URL.revokeObjectURL(url));
    throw error;
  }
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
    const document = await renderMarkdownDocument(source, file.path);
    return {
      kind: 'markdown',
      source,
      ...document,
    };
  }
}
