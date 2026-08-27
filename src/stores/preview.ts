import { defineStore } from 'pinia';
import { recordBrowsedFile } from '../api/history';
import { previewManager } from '../services/preview/PreviewManager';
import { renderMarkdownSource } from '../services/preview/MarkdownRenderer';
import type { FileInfo, PreviewContent } from '../types/file';
import { useHistoryStore } from './history';

export const usePreviewStore = defineStore('preview', {
  state: () => ({
    file: null as FileInfo | null,
    content: null as PreviewContent | null,
    loading: false,
    error: '',
    renderVersion: 0,
  }),
  actions: {
    async preview(file: FileInfo) {
      const version = ++this.renderVersion;
      if (this.content?.kind === 'image') URL.revokeObjectURL(this.content.url);
      this.file = file;
      this.content = null;
      this.loading = true;
      this.error = '';
      try {
        try {
          await recordBrowsedFile(file.path);
          await useHistoryStore().loadFiles();
        } catch {
          // 浏览记录失败不应阻止当前文件的正常预览。
        }
        const content = await previewManager.render(file);
        if (version !== this.renderVersion) {
          if (content.kind === 'image') URL.revokeObjectURL(content.url);
          return;
        }
        this.content = content;
      } catch (error) {
        if (version === this.renderVersion) {
          this.error = error instanceof Error ? error.message : String(error);
        }
      } finally {
        if (version === this.renderVersion) this.loading = false;
      }
    },
    clear() {
      this.renderVersion += 1;
      if (this.content?.kind === 'image') URL.revokeObjectURL(this.content.url);
      this.file = null;
      this.content = null;
      this.loading = false;
      this.error = '';
    },
    async renderMarkdownSource(source: string) {
      const version = ++this.renderVersion;
      if (this.content?.kind !== 'markdown') return;
      const filePath = this.file?.path;
      const html = await renderMarkdownSource(source);
      if (version !== this.renderVersion || this.file?.path !== filePath) return;
      this.content = {
        kind: 'markdown',
        source,
        html,
      };
    },
    updateFileMetadata(file: FileInfo) {
      if (this.file?.path === file.path) this.file = file;
    },
  },
});
