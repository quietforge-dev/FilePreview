import { defineStore } from 'pinia';
import {
  clearRecentFiles,
  clearRecentWorkspaces,
  listRecentFiles,
  listRecentWorkspaces,
} from '../api/history';
import type { RecentFile, RecentWorkspace } from '../types/history';

export const useHistoryStore = defineStore('history', {
  state: () => ({
    recentFiles: [] as RecentFile[],
    recentWorkspaces: [] as RecentWorkspace[],
    loadingFiles: false,
    loadingWorkspaces: false,
  }),
  actions: {
    async loadWorkspaces() {
      this.loadingWorkspaces = true;
      try {
        this.recentWorkspaces = await listRecentWorkspaces();
      } finally {
        this.loadingWorkspaces = false;
      }
    },
    async loadFiles() {
      this.loadingFiles = true;
      try {
        this.recentFiles = await listRecentFiles();
      } finally {
        this.loadingFiles = false;
      }
    },
    async clearWorkspaces() {
      await clearRecentWorkspaces();
      this.recentWorkspaces = [];
    },
    async clearFiles() {
      await clearRecentFiles();
      this.recentFiles = [];
    },
  },
});
