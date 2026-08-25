import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-dialog';
import { copyEntry, listDirectory, openWorkspace } from '../api/file';
import type { FileInfo, WorkspaceInfo } from '../types/file';
import { useHistoryStore } from './history';

export const useWorkspaceStore = defineStore('workspace', {
  state: () => ({
    workspace: null as WorkspaceInfo | null,
    currentDirectory: '',
    entries: [] as FileInfo[],
    loading: false,
    error: '',
    filter: '',
  }),
  getters: {
    visibleEntries: (state) => {
      const keyword = state.filter.trim().toLowerCase();
      return keyword
        ? state.entries.filter((entry) => entry.name.toLowerCase().includes(keyword))
        : state.entries;
    },
  },
  actions: {
    async chooseWorkspace() {
      const path = await open({ directory: true, multiple: false, title: '选择要预览的文件夹' });
      if (typeof path === 'string') await this.openWorkspace(path);
    },
    async openWorkspace(path: string) {
      this.loading = true;
      this.error = '';
      try {
        this.workspace = await openWorkspace(path);
        this.currentDirectory = this.workspace.path;
        await this.loadDirectory(this.currentDirectory);
        await useHistoryStore().loadWorkspaces();
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        this.loading = false;
      }
    },
    async loadDirectory(path?: string) {
      if (!this.workspace) return;
      this.loading = true;
      this.error = '';
      try {
        const target = path ?? this.currentDirectory;
        this.entries = await listDirectory(target);
        this.currentDirectory = target;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        this.loading = false;
      }
    },
    async copyEntry(source: string, destinationDirectory: string) {
      this.loading = true;
      this.error = '';
      try {
        const copied = await copyEntry(source, destinationDirectory);
        if (destinationDirectory === this.currentDirectory) {
          this.entries = await listDirectory(this.currentDirectory);
        }
        return copied;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        throw error;
      } finally {
        this.loading = false;
      }
    },
  },
});
