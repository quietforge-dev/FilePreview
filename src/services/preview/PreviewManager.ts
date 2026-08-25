import type { FileInfo, PreviewContent } from '../../types/file';
import { ImageRenderer } from './ImageRenderer';
import { MarkdownRenderer } from './MarkdownRenderer';
import { OfficeRenderer } from './OfficeRenderer';
import { TextRenderer } from './TextRenderer';
import type { PreviewRenderer } from './types';
import { UnsupportedRenderer } from './UnsupportedRenderer';

export class PreviewManager {
  private readonly renderers: PreviewRenderer[];

  constructor(renderers: PreviewRenderer[]) {
    this.renderers = renderers;
  }

  async render(file: FileInfo): Promise<PreviewContent> {
    const renderer = this.renderers.find((candidate) => candidate.canHandle(file));
    if (!renderer) throw new Error('没有可用的预览器');
    return renderer.render(file);
  }
}

export const previewManager = new PreviewManager([
  new MarkdownRenderer(),
  new OfficeRenderer(),
  new ImageRenderer(),
  new TextRenderer(),
  new UnsupportedRenderer(),
]);
