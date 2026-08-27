import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-dialog';
import {
  copyEntry,
  copyEntryToSystemClipboard,
  createFile as createWorkspaceFile,
  deleteEntry,
  hasSystemClipboardFiles,
  listDirectory,
  openWorkspace,
  pasteSystemClipboardEntries,
} from '../api/file';
import type { FileInfo, WorkspaceInfo } from '../types/file';
import { useHistoryStore } from './history';

export const useWorkspaceStore = defineStore('workspace', {
  state: () => ({
    workspace: null as WorkspaceInfo | null,
    currentDirectory: '',
    entries: [] as FileInfo[],
    directoryEntries: {} as Record<string, FileInfo[]>,
    loadingDirectories: {} as Record<string, boolean>,
    loading: false,
    error: '',
  }),
  getters: {
    rootEntries: (state) =>
      state.workspace ? (state.directoryEntries[state.workspace.path] ?? []) : [],
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
        this.directoryEntries = {};
        this.loadingDirectories = {};
        await this.loadDirectory(this.currentDirectory);
        await useHistoryStore().loadWorkspaces();
        return this.workspace;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        return null;
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
        this.directoryEntries = { ...this.directoryEntries, [target]: this.entries };
        this.currentDirectory = target;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        this.loading = false;
      }
    },
    selectDirectory(path: string) {
      this.currentDirectory = path;
    },
    async ensureDirectoryLoaded(path: string) {
      if (this.directoryEntries[path]) return this.directoryEntries[path];
      this.loadingDirectories = { ...this.loadingDirectories, [path]: true };
      this.error = '';
      try {
        const entries = await listDirectory(path);
        this.directoryEntries = { ...this.directoryEntries, [path]: entries };
        return entries;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        throw error;
      } finally {
        this.loadingDirectories = { ...this.loadingDirectories, [path]: false };
      }
    },
    async refreshDirectory(path?: string) {
      const target = path ?? this.currentDirectory;
      if (!target) return;
      this.loading = true;
      this.error = '';
      try {
        const entries = await listDirectory(target);
        this.directoryEntries = { ...this.directoryEntries, [target]: entries };
        if (target === this.currentDirectory) this.entries = entries;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
      } finally {
        this.loading = false;
      }
    },
    async refreshLoadedDirectories() {
      if (!this.workspace) return;
      this.loading = true;
      this.error = '';
      try {
        const paths = Object.keys(this.directoryEntries);
        const refreshed = await Promise.all(
          paths.map(async (path) => [path, await listDirectory(path)] as const),
        );
        this.directoryEntries = Object.fromEntries(refreshed);
        this.entries = this.directoryEntries[this.currentDirectory] ?? [];
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
          this.directoryEntries = {
            ...this.directoryEntries,
            [this.currentDirectory]: this.entries,
          };
        }
        return copied;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        throw error;
      } finally {
        this.loading = false;
      }
    },
    async createFile(destinationDirectory: string, fileName: string) {
      this.loading = true;
      this.error = '';
      try {
        const created = await createWorkspaceFile(destinationDirectory, fileName);
        const entries = await listDirectory(destinationDirectory);
        this.directoryEntries = {
          ...this.directoryEntries,
          [destinationDirectory]: entries,
        };
        if (destinationDirectory === this.currentDirectory) this.entries = entries;
        return created;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        throw error;
      } finally {
        this.loading = false;
      }
    },
    async hasSystemClipboardFiles() {
      return hasSystemClipboardFiles();
    },
    async copyEntryToSystemClipboard(path: string) {
      await copyEntryToSystemClipboard(path);
    },
    async pasteSystemClipboardEntries(destinationDirectory: string) {
      this.loading = true;
      this.error = '';
      try {
        const copied = await pasteSystemClipboardEntries(destinationDirectory);
        if (destinationDirectory === this.currentDirectory) {
          this.entries = await listDirectory(this.currentDirectory);
          this.directoryEntries = {
            ...this.directoryEntries,
            [this.currentDirectory]: this.entries,
          };
        }
        return copied;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        throw error;
      } finally {
        this.loading = false;
      }
    },
    async deleteEntry(path: string) {
      this.loading = true;
      this.error = '';
      try {
        await deleteEntry(path);
        this.removeDeletedEntry(path);
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        throw error;
      } finally {
        this.loading = false;
      }
    },
    removeDeletedEntry(path: string) {
      const isDeletedPath = (candidate: string) =>
        candidate.toLowerCase() === path.toLowerCase() ||
        candidate.toLowerCase().startsWith(`${path.toLowerCase()}\\`) ||
        candidate.toLowerCase().startsWith(`${path.toLowerCase()}/`);
      const directoryEntries = Object.fromEntries(
        Object.entries(this.directoryEntries)
          .filter(([directory]) => !isDeletedPath(directory))
          .map(([directory, entries]) => [
            directory,
            entries.filter((entry) => !isDeletedPath(entry.path)),
          ]),
      );
      this.directoryEntries = directoryEntries;
      this.loadingDirectories = Object.fromEntries(
        Object.entries(this.loadingDirectories).filter(([directory]) => !isDeletedPath(directory)),
      );
      if (isDeletedPath(this.currentDirectory)) {
        this.currentDirectory = this.workspace?.path ?? '';
      }
      this.entries = this.directoryEntries[this.currentDirectory] ?? [];
    },
  },
});
