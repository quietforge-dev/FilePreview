import { defineStore } from 'pinia';
import { previewManager } from '../services/preview/PreviewManager';
import type { FileInfo, PreviewContent } from '../types/file';

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
