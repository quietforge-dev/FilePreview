import { defineStore } from 'pinia';
import { searchFileContents, type ContentSearchResult } from '../api/file';

export const useSearchStore = defineStore('search', {
  state: () => ({
    visible: false,
    query: '',
    results: [] as ContentSearchResult[],
    loading: false,
    error: '',
  }),
  actions: {
    open() {
      this.visible = true;
      this.error = '';
    },
    close() {
      this.visible = false;
    },
    async search() {
      this.loading = true;
      this.error = '';
      try {
        this.results = await searchFileContents(this.query);
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        this.loading = false;
      }
    },
  },
});
