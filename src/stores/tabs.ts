import { open } from '@tauri-apps/plugin-dialog';
import { defineStore } from 'pinia';
import { getFileInfo } from '../api/file';
import { listSessionTabs, saveSessionTabs } from '../api/session';
import type { FileInfo } from '../types/file';
import type { SessionTab } from '../types/session';
import { useHistoryStore } from './history';
import { usePreviewStore } from './preview';
import { useWorkspaceStore } from './workspace';

const MAX_TABS = 20;

const createId = () =>
  globalThis.crypto?.randomUUID?.() ?? `tab-${Date.now()}-${Math.random().toString(16).slice(2)}`;

export const useTabsStore = defineStore('tabs', {
  state: () => ({
    tabs: [] as SessionTab[],
    activeId: '',
    restoring: false,
  }),
  getters: {
    activeTab: (state) => state.tabs.find((tab) => tab.id === state.activeId) ?? null,
  },
  actions: {
    async restore() {
      this.restoring = true;
      try {
        this.tabs = await listSessionTabs();
        this.activeId = this.tabs.find((tab) => tab.active)?.id ?? this.tabs[0]?.id ?? '';
        if (this.activeId && (await this.activate(this.activeId, false))) return;

        this.tabs = [];
        this.activeId = '';
        await this.persist();
        const history = useHistoryStore();
        await history.loadWorkspaces();
        const latest = history.recentWorkspaces[0];
        if (latest) await this.openWorkspace(latest.path);
      } finally {
        this.restoring = false;
      }
    },
    async chooseWorkspace() {
      const path = await open({ directory: true, multiple: false, title: '选择要预览的文件夹' });
      if (typeof path === 'string') await this.openWorkspace(path);
    },
    async openWorkspace(path: string) {
      const workspace = useWorkspaceStore();
      const opened = await workspace.openWorkspace(path);
      if (!opened) return false;
      const existing = this.tabs.find(
        (tab) => tab.kind === 'workspace' && tab.workspacePath === opened.path,
      );
      if (existing) {
        this.activeId = existing.id;
        existing.currentDirectory = workspace.currentDirectory;
      } else {
        this.tabs.push({
          id: createId(),
          kind: 'workspace',
          workspacePath: opened.path,
          workspaceName: opened.name,
          filePath: null,
          fileName: null,
          fileExtension: null,
          currentDirectory: workspace.currentDirectory,
          position: this.tabs.length,
          active: true,
        });
        this.tabs = this.tabs.slice(-MAX_TABS);
        this.activeId = this.tabs.at(-1)?.id ?? '';
      }
      usePreviewStore().clear();
      await this.persist();
      return true;
    },
    async openFile(file: FileInfo) {
      const workspace = useWorkspaceStore();
      if (!workspace.workspace) return;
      const existing = this.tabs.find((tab) => tab.kind === 'file' && tab.filePath === file.path);
      if (existing) {
        await this.activate(existing.id);
        return;
      }
      this.tabs.push({
        id: createId(),
        kind: 'file',
        workspacePath: workspace.workspace.path,
        workspaceName: workspace.workspace.name,
        filePath: file.path,
        fileName: file.name,
        fileExtension: file.extension,
        currentDirectory: workspace.currentDirectory,
        position: this.tabs.length,
        active: true,
      });
      this.tabs = this.tabs.slice(-MAX_TABS);
      this.activeId = this.tabs.at(-1)?.id ?? '';
      await usePreviewStore().preview(file);
      await this.persist();
    },
    async activate(id: string, persist = true) {
      const tab = this.tabs.find((candidate) => candidate.id === id);
      if (!tab) return false;
      const workspace = useWorkspaceStore();
      const preview = usePreviewStore();
      preview.clear();
      const opened = await workspace.openWorkspace(tab.workspacePath);
      if (!opened) return false;
      this.activeId = id;
      if (tab.currentDirectory && tab.currentDirectory !== opened.path) {
        await workspace.loadDirectory(tab.currentDirectory);
      }
      if (tab.kind === 'file' && tab.filePath) {
        try {
          await preview.preview(await getFileInfo(tab.filePath));
        } catch (error) {
          preview.file = {
            path: tab.filePath,
            name: tab.fileName ?? '已删除文件',
            extension: tab.fileExtension ?? '',
            size: 0,
            modifiedAt: null,
            isDirectory: false,
          };
          preview.error = `文件已不可用：${error instanceof Error ? error.message : String(error)}`;
        }
      }
      if (persist) await this.persist();
      return true;
    },
    updateCurrentDirectory(path: string) {
      const tab = this.activeTab;
      if (!tab) return;
      tab.currentDirectory = path;
      void this.persist();
    },
    async close(id?: string) {
      const targetId = id ?? this.activeId;
      const index = this.tabs.findIndex((tab) => tab.id === targetId);
      if (index < 0) return;
      const closingActive = targetId === this.activeId;
      this.tabs.splice(index, 1);
      if (!closingActive) {
        await this.persist();
        return;
      }
      usePreviewStore().clear();
      const next = this.tabs[index] ?? this.tabs[index - 1];
      this.activeId = next?.id ?? '';
      if (next) await this.activate(next.id, false);
      await this.persist();
    },
    async persist() {
      const tabs = this.tabs.map((tab, position) => ({
        ...tab,
        position,
        active: tab.id === this.activeId,
      }));
      this.tabs = tabs;
      try {
        await saveSessionTabs(tabs);
      } catch {
        // 会话恢复失败不应阻止文件浏览。
      }
    },
  },
});
