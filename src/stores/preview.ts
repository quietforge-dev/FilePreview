import { defineStore } from 'pinia';
import { recordBrowsedFile } from '../api/history';
import { previewManager } from '../services/preview/PreviewManager';
import type { FileInfo, PreviewContent } from '../types/file';
import { useHistoryStore } from './history';

export const usePreviewStore = defineStore('preview', {
  state: () => ({
    file: null as FileInfo | null,
    content: null as PreviewContent | null,
    loading: false,
    error: '',
  }),
  actions: {
    async preview(file: FileInfo) {
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
        this.content = await previewManager.render(file);
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        this.loading = false;
      }
    },
    clear() {
      if (this.content?.kind === 'image') URL.revokeObjectURL(this.content.url);
      this.file = null;
      this.content = null;
      this.error = '';
    },
  },
});
