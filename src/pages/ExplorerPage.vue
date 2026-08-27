<template>
  <main class="explorer-page">
    <header class="toolbar">
      <div class="toolbar-left">
        <span class="brand"
          ><el-icon><Files /></el-icon>FilePreview</span
        ><el-button
          :icon="Refresh"
          :disabled="!workspace.workspace"
          circle
          aria-label="刷新文件夹"
          @click="refresh"
        />
        <el-tooltip content="最近文件夹" placement="bottom">
          <el-button
            :icon="FolderOpened"
            circle
            aria-label="最近文件夹"
            @click="showHistoryDialog('workspaces')"
          />
        </el-tooltip>
      </div>
      <div class="path">{{ workspace.currentDirectory || '选择一个本地文件夹' }}</div>
      <div class="toolbar-right">
        <span class="app-version" :title="`FilePreview v${appVersion}`">v{{ appVersion }}</span>
        <el-tooltip content="在 GitHub 查看项目" placement="bottom">
          <el-button :icon="Github" circle aria-label="打开 GitHub" @click="openGithub" />
        </el-tooltip>
        <el-tooltip content="检查新版本" placement="bottom">
          <el-button
            :icon="RefreshCw"
            :loading="updateChecking"
            circle
            aria-label="检查更新"
            @click="checkForUpdates"
          />
        </el-tooltip>
      </div>
    </header>
    <ExplorerTabBar
      @choose-workspace="chooseWorkspace"
      @activate="activateTab"
      @close="closeTab"
      @close-tabs="closeTabs"
    />
    <RecentHistoryDialog
      v-model="historyDialogVisible"
      :title="historyDialogTitle"
      :items="historyDialogItems"
      :loading="historyDialogLoading"
      :empty-text="historyDialogEmptyText"
      @select="openHistoryItem"
      @clear="clearHistoryDialog"
    />
    <div v-if="workspace.error" class="error-bar">
      <el-icon><WarningFilled /></el-icon>{{ workspace.error }}
    </div>
    <div class="workspace-layout" :style="layoutStyle">
      <aside class="folder-pane">
        <div class="pane-title">资源管理器</div>
        <WorkspaceSearchPanel
          ref="workspaceSearchPanel"
          :workspace-ready="!!workspace.workspace"
          :mode="search.mode"
          :query="search.query"
          :name-results="search.nameResults"
          :content-results="search.contentResults"
          :loading="search.loading"
          :error="search.error"
          :searched="search.searched"
          @update:mode="setSearchMode"
          @update:query="setSearchQuery"
          @search="runWorkspaceSearch"
          @open-file="openSearchResult"
          @open-directory="openSearchDirectory"
        />
        <FolderTree
          v-if="!showSearchResults"
          :workspace="workspace.workspace"
          :entries="workspace.rootEntries"
          :path="workspace.currentDirectory"
          :selected-path="selectedEntry?.path ?? preview.file?.path"
          @open="openDirectory"
          @select="selectEntry"
          @contextmenu="openContextMenu"
        />
      </aside>
      <div
        class="pane-resizer"
        role="separator"
        aria-label="调整资源管理器宽度"
        aria-orientation="vertical"
        @pointerdown="startResize($event)"
      />
      <PreviewPanel
        class="preview-pane"
        :file="preview.file"
        :content="preview.content"
        :loading="preview.loading"
        :error="preview.error"
        @retry="retryPreview"
        @reload-markdown="reloadMarkdown"
      />
    </div>
    <FileContextMenu
      v-if="contextMenu"
      :file="contextMenu.file"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :can-paste="!!workspace.currentDirectory && (!!copiedEntry || systemClipboardHasFiles)"
      @open="openContextEntry"
      @reveal="revealContextEntry"
      @system-open="openContextEntryWithSystem"
      @copy-path="copyContextEntryPath"
      @copy="copyContextEntry"
      @paste="pasteContextEntry"
      @refresh="refresh"
      @delete="deleteContextEntry"
      @close="contextMenu = null"
    />
  </main>
  <AppUpdateDialog
    v-model:visible="updateVisible"
    :current-version="appVersion"
    :version="updateVersion"
    :notes="updateNotes"
    :installing="updateInstalling"
    :progress-percentage="updateProgressPercentage"
    :progress-label="updateProgressLabel"
    @install="installUpdate"
    @manual-download="openReleasePage"
  />
</template>

<script setup lang="ts">
import { Files, FolderOpened, Refresh, WarningFilled } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Github, RefreshCw } from 'lucide-vue-next';
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getAppVersion, openProjectUrl } from '../api/app';
import { getFileInfo, type ContentSearchResult } from '../api/file';
import {
  copyPathToClipboard,
  openWithDefaultApplication,
  revealInFileManager,
} from '../api/system';
import AppUpdateDialog from '../components/app/AppUpdateDialog.vue';
import ExplorerTabBar from '../components/explorer/ExplorerTabBar.vue';
import FileContextMenu from '../components/explorer/FileContextMenu.vue';
import RecentHistoryDialog from '../components/explorer/RecentHistoryDialog.vue';
import WorkspaceSearchPanel from '../components/explorer/WorkspaceSearchPanel.vue';
import { usePreviewStore } from '../stores/preview';
import { useWorkspaceStore } from '../stores/workspace';
import { useHistoryStore } from '../stores/history';
import { useAppSettingsStore } from '../stores/appSettings';
import { useMarkdownEditorStore } from '../stores/markdownEditor';
import { useSearchStore, type WorkspaceSearchMode } from '../stores/search';
import { useTabsStore } from '../stores/tabs';
import type { FileInfo } from '../types/file';
import { useAppUpdater } from '../composables/useAppUpdater';
import FolderTree from '../components/explorer/FolderTree.vue';
import PreviewPanel from '../components/preview/PreviewPanel.vue';

const workspace = useWorkspaceStore();
const preview = usePreviewStore();
const history = useHistoryStore();
const appSettings = useAppSettingsStore();
const markdownEditor = useMarkdownEditorStore();
const search = useSearchStore();
const tabs = useTabsStore();
const appVersion = ref('0.0.5');
const updater = useAppUpdater();
const {
  checking: updateChecking,
  installing: updateInstalling,
  notes: updateNotes,
  progressLabel: updateProgressLabel,
  progressPercentage: updateProgressPercentage,
  version: updateVersion,
  visible: updateVisible,
} = updater;
const GITHUB_URL = 'https://github.com/quietforge-dev/FilePreview';
const AUTO_UPDATE_INITIAL_DELAY_MS = 5_000;
const AUTO_UPDATE_INTERVAL_MS = 6 * 60 * 60 * 1_000;
const MIN_FOLDER_WIDTH = 180;
const MIN_PREVIEW_WIDTH = 360;
const RESIZER_TOTAL_WIDTH = 8;

type FileContextMenu = { file: FileInfo; x: number; y: number };
type HistoryDialog = 'workspaces' | 'files';

const folderWidth = ref(230);
const activeResize = ref(false);
const selectedEntry = ref<FileInfo | null>(null);
const copiedEntry = ref<FileInfo | null>(null);
const systemClipboardHasFiles = ref(false);
const contextMenu = ref<FileContextMenu | null>(null);
const historyDialog = ref<HistoryDialog | null>(null);
const workspaceSearchPanel = ref<{ focus: () => void }>();
const layoutStyle = computed(() => ({
  '--folder-width': `${folderWidth.value}px`,
}));
const historyDialogVisible = computed({
  get: () => historyDialog.value !== null,
  set: (value: boolean) => {
    if (!value) historyDialog.value = null;
  },
});
const historyDialogTitle = computed(() =>
  historyDialog.value === 'files' ? '浏览记录' : '最近文件夹',
);
const historyDialogItems = computed(() =>
  historyDialog.value === 'files' ? history.recentFiles : history.recentWorkspaces,
);
const historyDialogLoading = computed(() =>
  historyDialog.value === 'files' ? history.loadingFiles : history.loadingWorkspaces,
);
const historyDialogEmptyText = computed(() =>
  historyDialog.value === 'files' ? '暂无浏览记录' : '暂无最近文件夹',
);
const showSearchResults = computed(
  () => Boolean(search.query.trim()) && (search.searched || search.loading),
);
const setSearchMode = (mode: WorkspaceSearchMode) => search.setMode(mode);
const setSearchQuery = (query: string) => search.setQuery(query);
const runWorkspaceSearch = () => {
  if (search.mode === 'name') {
    if (nameSearchTimer !== undefined) window.clearTimeout(nameSearchTimer);
    nameSearchTimer = undefined;
    void search.searchNames();
    return;
  }
  void search.searchContents();
};
const focusWorkspaceSearch = (mode: WorkspaceSearchMode = 'name') => {
  search.setMode(mode);
  void nextTick(() => workspaceSearchPanel.value?.focus());
};

const chooseWorkspace = async () => {
  if (!(await confirmMarkdownChanges(filePathsForTabs(tabs.tabs)))) return;
  const previousPath = workspace.workspace?.path;
  await tabs.chooseWorkspace();
  if (workspace.workspace?.path !== previousPath) markdownEditor.clear();
  if (workspace.workspace?.path !== previousPath) search.reset();
  selectedEntry.value = null;
};
const openDirectory = async (path: string) => {
  workspace.selectDirectory(path);
  tabs.updateCurrentDirectory(path);
  selectedEntry.value = null;
  preview.clear();
};
const selectEntry = async (entry: FileInfo) => {
  if (!entry.isDirectory && !(await openFile(entry))) return;
  selectedEntry.value = entry;
};
const refresh = () => void workspace.refreshLoadedDirectories();
const retryPreview = () => {
  if (preview.file) void preview.preview(preview.file);
};
const activeFilePath = () =>
  tabs.activeTab?.kind === 'file' && tabs.activeTab.filePath ? [tabs.activeTab.filePath] : [];
const filePathsForTabs = (tabList: typeof tabs.tabs) =>
  tabList.flatMap((tab) => (tab.kind === 'file' && tab.filePath ? [tab.filePath] : []));
const markdownPathsBeforeOpening = (file: FileInfo) => {
  const existing = tabs.tabs.find((tab) => tab.kind === 'file' && tab.filePath === file.path);
  const paths = existing?.id === tabs.activeId ? [] : activeFilePath();
  if (!existing && tabs.tabs.length >= 20) paths.push(...filePathsForTabs(tabs.tabs.slice(0, 1)));
  return paths;
};
const removeMarkdownSessions = (paths: string[]) =>
  paths.forEach((path) => markdownEditor.remove(path));
const confirmMarkdownChanges = async (paths: string[]) => {
  const dirtyPaths = [...new Set(paths)].filter((path) => markdownEditor.sessions[path]?.dirty);
  if (!dirtyPaths.length) return true;
  const hasExternalChanges = dirtyPaths.some(
    (path) => markdownEditor.sessions[path]?.externalChanged,
  );

  try {
    await ElMessageBox.confirm(
      hasExternalChanges
        ? '文件已在外部修改，保存将覆盖外部内容。'
        : '存在未保存的 Markdown 修改。',
      hasExternalChanges ? '确认覆盖保存' : '未保存的修改',
      {
        confirmButtonText: hasExternalChanges ? '覆盖保存并继续' : '保存并继续',
        cancelButtonText: '放弃修改',
        distinguishCancelAndClose: true,
        type: 'warning',
      },
    );
  } catch (reason) {
    if (reason === 'cancel') {
      dirtyPaths.forEach((path) => markdownEditor.discardChanges(path));
      return true;
    }
    return false;
  }

  try {
    for (const path of dirtyPaths) {
      const saved = await markdownEditor.save(path);
      if (!saved) throw new Error('文件正在保存，请稍后重试');
      preview.updateFileMetadata(saved);
    }
    return true;
  } catch (error) {
    ElMessage.error(`保存失败：${error instanceof Error ? error.message : String(error)}`);
    return false;
  }
};
const activateTab = async (id: string) => {
  if (id === tabs.activeId || !(await confirmMarkdownChanges(activeFilePath()))) return;
  await tabs.activate(id);
};
const closeTab = async (id: string) => {
  const target = tabs.tabs.find((tab) => tab.id === id);
  const paths = filePathsForTabs(target ? [target] : []);
  if (!(await confirmMarkdownChanges(paths))) return;
  await tabs.close(id);
  removeMarkdownSessions(paths);
};
const closeTabs = async (scope: 'left' | 'right' | 'others' | 'all', id: string) => {
  const index = tabs.tabs.findIndex((tab) => tab.id === id);
  if (index < 0) return;
  const closingTabs =
    scope === 'left'
      ? tabs.tabs.slice(0, index)
      : scope === 'right'
        ? tabs.tabs.slice(index + 1)
        : scope === 'others'
          ? tabs.tabs.filter((tab) => tab.id !== id)
          : tabs.tabs;
  const paths = filePathsForTabs(closingTabs);
  if (!(await confirmMarkdownChanges(paths))) return;
  if (scope === 'left') await tabs.closeLeft(id);
  if (scope === 'right') await tabs.closeRight(id);
  if (scope === 'others') await tabs.closeOthers(id);
  if (scope === 'all') await tabs.closeAll();
  removeMarkdownSessions(paths);
};
const closeActiveTab = () => {
  if (tabs.activeId) void closeTab(tabs.activeId);
};
const openFile = async (file: FileInfo) => {
  const evictedPaths =
    !tabs.tabs.some((tab) => tab.kind === 'file' && tab.filePath === file.path) &&
    tabs.tabs.length >= 20
      ? filePathsForTabs(tabs.tabs.slice(0, 1))
      : [];
  if (!(await confirmMarkdownChanges(markdownPathsBeforeOpening(file)))) return false;
  await tabs.openFile(file);
  removeMarkdownSessions(evictedPaths);
  return true;
};
const copySelectedEntry = async (entry: FileInfo) => {
  copiedEntry.value = entry;
  selectedEntry.value = entry;
  contextMenu.value = null;
  try {
    await workspace.copyEntryToSystemClipboard(entry.path);
    systemClipboardHasFiles.value = true;
    ElMessage.success(`已复制 ${entry.name}`);
  } catch (error) {
    if (copiedEntry.value?.path === entry.path) copiedEntry.value = null;
    ElMessage.error(`复制失败：${error instanceof Error ? error.message : String(error)}`);
  }
};
const copyContextEntry = () => {
  const entry = contextEntry();
  if (entry) void copySelectedEntry(entry);
};
const refreshSystemClipboardAvailability = async () => {
  if (!workspace.currentDirectory) {
    systemClipboardHasFiles.value = false;
    return;
  }
  try {
    systemClipboardHasFiles.value = await workspace.hasSystemClipboardFiles();
  } catch {
    systemClipboardHasFiles.value = false;
  }
};
const pasteEntry = async (destinationDirectory = workspace.currentDirectory) => {
  contextMenu.value = null;
  if (!destinationDirectory) return;
  try {
    const hasSystemFiles = await workspace.hasSystemClipboardFiles();
    systemClipboardHasFiles.value = hasSystemFiles;
    if (hasSystemFiles) {
      const copied = await workspace.pasteSystemClipboardEntries(destinationDirectory);
      if (destinationDirectory === workspace.currentDirectory && copied.length) {
        selectedEntry.value = copied[copied.length - 1];
      }
      ElMessage.success(
        copied.length === 1 ? `已粘贴 ${copied[0].name}` : `已粘贴 ${copied.length} 个项目`,
      );
      return;
    }

    const source = copiedEntry.value;
    if (!source) return;
    const copied = await workspace.copyEntry(source.path, destinationDirectory);
    if (destinationDirectory === workspace.currentDirectory) selectedEntry.value = copied;
    ElMessage.success(`已粘贴 ${copied.name}`);
  } catch (error) {
    ElMessage.error(`粘贴失败：${error instanceof Error ? error.message : String(error)}`);
  }
};
const contextEntry = () => contextMenu.value?.file ?? selectedEntry.value;
const openContextEntry = async () => {
  const entry = contextEntry();
  if (!entry) return;
  if (entry.isDirectory) await openDirectory(entry.path);
  else await openFile(entry);
};
const revealContextEntry = async () => {
  const entry = contextEntry();
  if (!entry || entry.isDirectory) return;
  try {
    await revealInFileManager(entry.path);
  } catch (error) {
    ElMessage.error(
      `无法在文件夹中显示：${error instanceof Error ? error.message : String(error)}`,
    );
  }
};
const openContextEntryWithSystem = async () => {
  const entry = contextEntry();
  if (!entry) return;
  try {
    await openWithDefaultApplication(entry.path);
  } catch (error) {
    ElMessage.error(`无法使用系统打开：${error instanceof Error ? error.message : String(error)}`);
  }
};
const copyContextEntryPath = async () => {
  const entry = contextEntry();
  if (!entry) return;
  try {
    await copyPathToClipboard(entry.path);
    ElMessage.success('路径已复制到剪贴板');
  } catch (error) {
    ElMessage.error(`复制路径失败：${error instanceof Error ? error.message : String(error)}`);
  }
};
const pasteContextEntry = () => {
  const entry = contextEntry();
  void pasteEntry(entry?.isDirectory ? entry.path : workspace.currentDirectory);
};
const isPathAtOrBelow = (candidate: string, parent: string) => {
  const normalizedCandidate = candidate.toLowerCase();
  const normalizedParent = parent.toLowerCase();
  return (
    normalizedCandidate === normalizedParent ||
    normalizedCandidate.startsWith(`${normalizedParent}\\`) ||
    normalizedCandidate.startsWith(`${normalizedParent}/`)
  );
};
const deleteContextEntry = async () => {
  const entry = contextEntry();
  if (!entry) return;
  const deletedFilePaths = filePathsForTabs(tabs.tabs).filter((path) =>
    isPathAtOrBelow(path, entry.path),
  );
  if (!(await confirmMarkdownChanges(deletedFilePaths))) return;

  try {
    await ElMessageBox.confirm(
      entry.isDirectory
        ? `“${entry.name}”及其所有内容会移入系统回收站。`
        : `“${entry.name}”会移入系统回收站。`,
      entry.isDirectory ? '删除文件夹' : '删除文件',
      {
        confirmButtonText: '移入回收站',
        cancelButtonText: '取消',
        type: 'warning',
      },
    );
  } catch {
    return;
  }

  try {
    await workspace.deleteEntry(entry.path);
    await tabs.closeFilesAtPath(entry.path);
    removeMarkdownSessions(deletedFilePaths);
    if (preview.file && isPathAtOrBelow(preview.file.path, entry.path)) preview.clear();
    if (selectedEntry.value && isPathAtOrBelow(selectedEntry.value.path, entry.path)) {
      selectedEntry.value = null;
    }
    if (copiedEntry.value && isPathAtOrBelow(copiedEntry.value.path, entry.path)) {
      copiedEntry.value = null;
    }
    ElMessage.success(entry.isDirectory ? '文件夹已移入系统回收站' : '文件已移入系统回收站');
  } catch (error) {
    ElMessage.error(`删除失败：${error instanceof Error ? error.message : String(error)}`);
  }
};
const openContextMenu = (file: FileInfo, event: MouseEvent) => {
  selectedEntry.value = file;
  systemClipboardHasFiles.value = false;
  contextMenu.value = {
    file,
    x: event.clientX,
    y: event.clientY,
  };
  void refreshSystemClipboardAvailability();
};

const clamp = (value: number, min: number, max: number) => Math.max(min, Math.min(max, value));
const maxFolderWidth = () => window.innerWidth - MIN_PREVIEW_WIDTH - RESIZER_TOTAL_WIDTH;
const clampFolderWidth = (width: number) =>
  clamp(width, MIN_FOLDER_WIDTH, Math.max(MIN_FOLDER_WIDTH, maxFolderWidth()));
const startResize = (event: PointerEvent) => {
  if (event.button !== 0) return;
  event.preventDefault();
  activeResize.value = true;
  document.documentElement.style.cursor = 'col-resize';
  document.addEventListener('pointermove', resizePane);
  document.addEventListener('pointerup', stopResize);
};
const resizePane = (event: PointerEvent) => {
  if (!activeResize.value) return;
  folderWidth.value = clampFolderWidth(event.clientX - 4);
};
const stopResize = () => {
  const shouldPersistWidth = activeResize.value;
  activeResize.value = false;
  document.documentElement.style.cursor = '';
  document.removeEventListener('pointermove', resizePane);
  document.removeEventListener('pointerup', stopResize);
  if (shouldPersistWidth) void appSettings.saveFolderPaneWidth(folderWidth.value);
};
const handleKeyboardShortcut = (event: KeyboardEvent) => {
  const target = event.target as HTMLElement | null;
  if (target?.closest('input, textarea, [contenteditable="true"]')) return;
  if (event.key === 'F5') {
    event.preventDefault();
    refresh();
    return;
  }
  if (!event.ctrlKey && !event.metaKey) return;

  if (event.shiftKey && event.key.toLowerCase() === 'f') {
    event.preventDefault();
    focusWorkspaceSearch('content');
    return;
  }
  if (event.key.toLowerCase() === 'f') {
    event.preventDefault();
    focusWorkspaceSearch();
    return;
  }
  if (event.key.toLowerCase() === 'o' || event.key.toLowerCase() === 't') {
    event.preventDefault();
    void chooseWorkspace();
    return;
  }
  if (event.key.toLowerCase() === 'w') {
    event.preventDefault();
    closeActiveTab();
    return;
  }

  if (event.key.toLowerCase() === 'c' && selectedEntry.value) {
    event.preventDefault();
    void copySelectedEntry(selectedEntry.value);
  }
  if (event.key.toLowerCase() === 'v' && workspace.currentDirectory) {
    event.preventDefault();
    void pasteEntry();
  }
};
const openGithub = async () => {
  try {
    await openProjectUrl(GITHUB_URL);
  } catch {
    window.open(GITHUB_URL, '_blank', 'noopener,noreferrer');
  }
};
const checkForUpdates = async () => {
  try {
    const result = await updater.checkForUpdates();
    if (result && !result.available) ElMessage.success(`当前已是最新版本：v${appVersion.value}`);
  } catch (error) {
    ElMessage.error(`检查更新失败：${error instanceof Error ? error.message : String(error)}`);
  }
};
const checkForUpdatesSilently = async () => {
  try {
    await updater.checkForUpdates();
  } catch {
    // 自动检查失败时不打断本地文件浏览。
  }
};
const installUpdate = async () => {
  try {
    await updater.installAndRelaunch();
  } catch (error) {
    ElMessage.error(error instanceof Error ? error.message : String(error));
  }
};
const openReleasePage = () => void openProjectUrl(updater.releasePageUrl());
const recentFileByPath = computed(
  () => new Map(history.recentFiles.map((item) => [item.path, item])),
);

const showHistoryDialog = (kind: HistoryDialog) => {
  historyDialog.value = kind;
  if (kind === 'files') void history.loadFiles();
  else void history.loadWorkspaces();
};

const openRecentWorkspace = async (command: string) => {
  if (command === 'clear-workspaces') {
    await clearHistory('最近文件夹', () => history.clearWorkspaces());
    return;
  }
  if (!(await confirmMarkdownChanges(filePathsForTabs(tabs.tabs)))) return;
  const previousPath = workspace.workspace?.path;
  if (!(await tabs.replaceWorkspace(command))) return;
  if (workspace.workspace?.path !== previousPath) markdownEditor.clear();
  if (workspace.workspace?.path !== previousPath) search.reset();
  selectedEntry.value = null;
};

const openRecentFile = async (command: string) => {
  if (command === 'clear-files') {
    await clearHistory('浏览记录', () => history.clearFiles());
    return;
  }
  const item = recentFileByPath.value.get(command);
  if (!item) return;
  const separator = Math.max(item.path.lastIndexOf('/'), item.path.lastIndexOf('\\'));
  if (separator < 1) return;
  if (!(await confirmMarkdownChanges(filePathsForTabs(tabs.tabs)))) return;
  const previousPath = workspace.workspace?.path;
  if (!(await tabs.replaceWorkspace(item.path.slice(0, separator)))) return;
  if (workspace.workspace?.path !== previousPath) markdownEditor.clear();
  if (workspace.workspace?.path !== previousPath) search.reset();
  try {
    const file = await getFileInfo(item.path);
    selectedEntry.value = file;
    await tabs.openFile(file);
  } catch (error) {
    ElMessage.error(`文件已不可用：${error instanceof Error ? error.message : String(error)}`);
  }
};

const openHistoryItem = (path: string) => {
  const kind = historyDialog.value;
  historyDialog.value = null;
  if (kind === 'files') void openRecentFile(path);
  else void openRecentWorkspace(path);
};
const clearHistoryDialog = () => {
  if (historyDialog.value === 'files') {
    void clearHistory('浏览记录', () => history.clearFiles());
    return;
  }
  void clearHistory('最近文件夹', () => history.clearWorkspaces());
};

const openSearchResult = async (result: FileInfo | ContentSearchResult) => {
  try {
    const file = 'lineNumber' in result ? await getFileInfo(result.path) : result;
    if (!(await openFile(file))) return;
    selectedEntry.value = file;
  } catch (error) {
    ElMessage.error(`无法打开搜索结果：${error instanceof Error ? error.message : String(error)}`);
  }
};
const openSearchDirectory = async (entry: FileInfo) => {
  try {
    await workspace.loadDirectory(entry.path);
    await openDirectory(entry.path);
  } catch (error) {
    ElMessage.error(`无法打开文件夹：${error instanceof Error ? error.message : String(error)}`);
  }
};

const samePath = (left: string, right: string) => left.toLowerCase() === right.toLowerCase();
const refreshChangedWorkspace = async (workspacePath: string, changedPaths: string[]) => {
  if (workspace.workspace?.path !== workspacePath) return;
  const previous = preview.file;
  await workspace.refreshLoadedDirectories();
  if (search.mode === 'name' && search.query.trim()) void search.searchNames();
  if (!previous || !changedPaths.some((path) => samePath(path, previous.path))) return;
  if (markdownEditor.sessions[previous.path]?.dirty) {
    markdownEditor.markExternalChanged(previous.path);
    return;
  }
  try {
    const current = await getFileInfo(previous.path);
    if (current.size !== previous.size || current.modifiedAt !== previous.modifiedAt) {
      await preview.preview(current);
    }
  } catch (error) {
    preview.clear();
    preview.file = previous;
    preview.error = `正在预览的文件已不可用：${error instanceof Error ? error.message : String(error)}`;
  }
};

const reloadMarkdown = async () => {
  const previous = preview.file;
  if (!previous) return;
  try {
    markdownEditor.discardChanges(previous.path);
    await preview.preview(await getFileInfo(previous.path));
  } catch (error) {
    ElMessage.error(`重新加载失败：${error instanceof Error ? error.message : String(error)}`);
  }
};

const handleMenuAction = (action: string) => {
  switch (action) {
    case 'open-folder':
      void chooseWorkspace();
      break;
    case 'show-recent-workspaces':
      showHistoryDialog('workspaces');
      break;
    case 'show-recent-files':
      showHistoryDialog('files');
      break;
    case 'close-tab':
      closeActiveTab();
      break;
    case 'copy':
      if (selectedEntry.value) void copySelectedEntry(selectedEntry.value);
      break;
    case 'paste':
      void pasteEntry();
      break;
    case 'refresh':
      refresh();
      break;
    case 'search-content':
      focusWorkspaceSearch('content');
      break;
    case 'check-updates':
      void checkForUpdates();
      break;
    case 'project-home':
      void openGithub();
      break;
  }
};

const clearHistory = async (name: string, action: () => Promise<void>) => {
  try {
    await ElMessageBox.confirm(`确定清空${name}吗？此操作不会删除本地文件。`, '清空记录', {
      confirmButtonText: '清空',
      cancelButtonText: '取消',
      type: 'warning',
    });
    await action();
    ElMessage.success(`${name}已清空`);
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') ElMessage.error(`清空失败：${String(error)}`);
  }
};

let initialUpdateTimer: number | undefined;
let periodicUpdateTimer: number | undefined;
let unlistenMenu: UnlistenFn | undefined;
let unlistenFileWatch: UnlistenFn | undefined;
let nameSearchTimer: number | undefined;

watch([() => search.mode, () => search.query, () => workspace.workspace?.path], () => {
  if (nameSearchTimer !== undefined) window.clearTimeout(nameSearchTimer);
  if (search.mode !== 'name' || !workspace.workspace || !search.query.trim()) {
    if (search.mode === 'name') search.clearNameResults();
    return;
  }
  nameSearchTimer = window.setTimeout(() => {
    nameSearchTimer = undefined;
    void search.searchNames();
  }, 220);
});

onMounted(() => {
  window.addEventListener('keydown', handleKeyboardShortcut);
  void getAppVersion().then((value) => (appVersion.value = value));
  initialUpdateTimer = window.setTimeout(
    () => void checkForUpdatesSilently(),
    AUTO_UPDATE_INITIAL_DELAY_MS,
  );
  periodicUpdateTimer = window.setInterval(
    () => void checkForUpdatesSilently(),
    AUTO_UPDATE_INTERVAL_MS,
  );
  void tabs.restore();
  void appSettings.restoreFolderPaneWidth().then((width) => {
    if (width !== null) folderWidth.value = clampFolderWidth(width);
  });
  void listen<string>('menu-action', (event) => handleMenuAction(event.payload)).then(
    (unlisten) => (unlistenMenu = unlisten),
  );
  void listen<{ workspacePath: string; paths: string[] }>('workspace-files-changed', (event) => {
    void refreshChangedWorkspace(event.payload.workspacePath, event.payload.paths);
  }).then((unlisten) => (unlistenFileWatch = unlisten));
});

onUnmounted(() => {
  stopResize();
  window.removeEventListener('keydown', handleKeyboardShortcut);
  if (initialUpdateTimer !== undefined) window.clearTimeout(initialUpdateTimer);
  if (periodicUpdateTimer !== undefined) window.clearInterval(periodicUpdateTimer);
  if (nameSearchTimer !== undefined) window.clearTimeout(nameSearchTimer);
  unlistenMenu?.();
  unlistenFileWatch?.();
});
</script>

<style scoped lang="scss">
.explorer-page {
  background: #f8fafc;
  display: flex;
  flex-direction: column;
  height: 100vh;
  min-width: 1100px;
}
.toolbar {
  align-items: center;
  background: #fff;
  border-bottom: 1px solid #e1e6ed;
  display: grid;
  gap: 18px;
  grid-template-columns: auto minmax(120px, 1fr) auto;
  height: 60px;
  padding: 0 18px;
}
.toolbar-left {
  align-items: center;
  display: flex;
  gap: 9px;
  white-space: nowrap;
}
.brand {
  align-items: center;
  color: #1e3a5f;
  display: flex;
  font-size: 17px;
  font-weight: 700;
  gap: 7px;
  margin-right: 8px;
}
.brand .el-icon {
  color: #2563eb;
  font-size: 20px;
}
.path {
  color: #7b8797;
  font-family: Consolas, monospace;
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.toolbar-right {
  align-items: center;
  display: flex;
  gap: 8px;
}
.toolbar-right .el-button {
  flex: 0 0 auto;
}
.app-version {
  color: #7b8797;
  font-family: Consolas, monospace;
  font-size: 12px;
  white-space: nowrap;
}
.error-bar {
  align-items: center;
  background: #fff2f0;
  color: #b42318;
  display: flex;
  font-size: 13px;
  gap: 8px;
  padding: 8px 18px;
}
.workspace-layout {
  display: grid;
  flex: 1;
  grid-template-columns: var(--folder-width) 8px minmax(360px, 1fr);
  min-height: 0;
}
.folder-pane,
.preview-pane {
  background: #fff;
  min-width: 0;
  overflow: auto;
}
.pane-resizer {
  background: #fff;
  cursor: col-resize;
  position: relative;
  touch-action: none;
}
.pane-resizer::after {
  background: #e1e6ed;
  content: '';
  inset: 0 3px;
  position: absolute;
}
.pane-resizer:hover::after {
  background: #60a5fa;
}
.pane-title {
  align-items: center;
  border-bottom: 1px solid #edf0f4;
  color: #697586;
  display: flex;
  font-size: 11px;
  font-weight: 700;
  height: 42px;
  letter-spacing: 0.04em;
  padding: 0 14px;
  text-transform: uppercase;
}
.folder-pane :deep(.tree) {
  padding: 8px 5px;
}
</style>
