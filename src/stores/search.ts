import { defineStore } from 'pinia';
import { searchFileContents, searchWorkspaceEntries, type ContentSearchResult } from '../api/file';
import type { FileInfo } from '../types/file';

export type WorkspaceSearchMode = 'name' | 'content';

export const useSearchStore = defineStore('search', {
  state: () => ({
    mode: 'name' as WorkspaceSearchMode,
    query: '',
    nameResults: [] as FileInfo[],
    contentResults: [] as ContentSearchResult[],
    loading: false,
    error: '',
    searched: false,
    requestVersion: 0,
  }),
  actions: {
    setMode(mode: WorkspaceSearchMode) {
      this.requestVersion += 1;
      this.mode = mode;
      this.loading = false;
      this.error = '';
      this.searched = false;
      if (mode === 'name') this.contentResults = [];
      else this.nameResults = [];
    },
    reset() {
      this.requestVersion += 1;
      this.query = '';
      this.nameResults = [];
      this.contentResults = [];
      this.loading = false;
      this.error = '';
      this.searched = false;
    },
    clearNameResults() {
      this.requestVersion += 1;
      this.nameResults = [];
      this.error = '';
      this.searched = false;
    },
    setQuery(query: string) {
      this.requestVersion += 1;
      this.query = query;
      this.nameResults = [];
      this.contentResults = [];
      this.loading = false;
      this.error = '';
      this.searched = false;
    },
    async searchNames() {
      const query = this.query.trim();
      if (!query) {
        this.clearNameResults();
        return;
      }
      const requestVersion = ++this.requestVersion;
      this.loading = true;
      this.error = '';
      this.searched = true;
      try {
        const results = await searchWorkspaceEntries(query);
        if (requestVersion !== this.requestVersion || this.mode !== 'name') return;
        this.nameResults = results;
      } catch (error) {
        if (requestVersion !== this.requestVersion || this.mode !== 'name') return;
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        if (requestVersion === this.requestVersion) this.loading = false;
      }
    },
    async searchContents() {
      const query = this.query.trim();
      if (!query) return;
      const requestVersion = ++this.requestVersion;
      this.loading = true;
      this.error = '';
      this.searched = true;
      try {
        const results = await searchFileContents(query);
        if (requestVersion !== this.requestVersion || this.mode !== 'content') return;
        this.contentResults = results;
      } catch (error) {
        if (requestVersion !== this.requestVersion || this.mode !== 'content') return;
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        if (requestVersion === this.requestVersion) this.loading = false;
      }
    },
  },
});
