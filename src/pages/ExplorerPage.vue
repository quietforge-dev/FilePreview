<template>
  <main class="explorer-page">
    <header class="toolbar">
      <div class="toolbar-left">
        <span class="brand"
          ><el-icon><Files /></el-icon>FilePreview</span
        ><el-button :icon="FolderOpened" type="primary" @click="chooseWorkspace"
          >打开文件夹</el-button
        ><el-dropdown
          trigger="click"
          @visible-change="loadWorkspaceHistory"
          @command="openRecentWorkspace"
        >
          <el-button :icon="Clock">最近文件夹</el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item
                v-for="item in history.recentWorkspaces"
                :key="item.path"
                :command="item.path"
              >
                {{ item.name }}<span class="history-path">{{ item.path }}</span>
              </el-dropdown-item>
              <el-dropdown-item v-if="!history.recentWorkspaces.length" disabled>
                暂无记录
              </el-dropdown-item>
              <el-dropdown-item
                v-if="history.recentWorkspaces.length"
                divided
                command="clear-workspaces"
              >
                <el-icon><Delete /></el-icon>清空最近文件夹
              </el-dropdown-item>
            </el-dropdown-menu>
          </template> </el-dropdown
        ><el-dropdown trigger="click" @visible-change="loadFileHistory" @command="openRecentFile">
          <el-button :icon="Document">浏览记录</el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item
                v-for="item in history.recentFiles"
                :key="item.path"
                :command="item.path"
              >
                {{ item.name }}<span class="history-path">{{ item.path }}</span>
              </el-dropdown-item>
              <el-dropdown-item v-if="!history.recentFiles.length" disabled>
                暂无记录
              </el-dropdown-item>
              <el-dropdown-item v-if="history.recentFiles.length" divided command="clear-files">
                <el-icon><Delete /></el-icon>清空浏览记录
              </el-dropdown-item>
            </el-dropdown-menu>
          </template> </el-dropdown
        ><el-button
          :icon="Refresh"
          :disabled="!workspace.workspace"
          circle
          aria-label="刷新文件夹"
          @click="refresh"
        />
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
        <el-input
          v-model="workspace.filter"
          class="search"
          :prefix-icon="Search"
          placeholder="搜索当前目录"
          clearable
        />
      </div>
    </header>
    <div v-if="workspace.error" class="error-bar">
      <el-icon><WarningFilled /></el-icon>{{ workspace.error }}
    </div>
    <div class="workspace-layout" :style="layoutStyle">
      <aside class="folder-pane">
        <div class="pane-title">资源管理器</div>
        <FolderTree
          :workspace="workspace.workspace"
          :entries="workspace.rootEntries"
          :path="workspace.currentDirectory"
          :selected-path="preview.file?.path"
          :filter="workspace.filter"
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
      />
    </div>
    <div
      v-if="contextMenu"
      ref="contextMenuElement"
      class="file-context-menu"
      :style="{ left: `${contextMenu.x}px`, top: `${contextMenu.y}px` }"
      @contextmenu.prevent
    >
      <button type="button" @click="copySelectedEntry(contextMenu.file)">
        <Copy :size="16" />复制
      </button>
      <button
        type="button"
        :disabled="!copiedEntry || !workspace.currentDirectory"
        @click="
          pasteEntry(
            contextMenu.file.isDirectory ? contextMenu.file.path : workspace.currentDirectory,
          )
        "
      >
        <ClipboardPaste :size="16" />粘贴到此处
      </button>
    </div>
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
import {
  Clock,
  Delete,
  Document,
  Files,
  FolderOpened,
  Refresh,
  Search,
  WarningFilled,
} from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { ClipboardPaste, Copy, Github, RefreshCw } from 'lucide-vue-next';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { getAppVersion, openProjectUrl } from '../api/app';
import AppUpdateDialog from '../components/app/AppUpdateDialog.vue';
import { usePreviewStore } from '../stores/preview';
import { useWorkspaceStore } from '../stores/workspace';
import { useHistoryStore } from '../stores/history';
import type { FileInfo } from '../types/file';
import { useAppUpdater } from '../composables/useAppUpdater';
import FolderTree from '../components/explorer/FolderTree.vue';
import PreviewPanel from '../components/preview/PreviewPanel.vue';

const workspace = useWorkspaceStore();
const preview = usePreviewStore();
const history = useHistoryStore();
const appVersion = ref('0.0.3');
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

const folderWidth = ref(230);
const activeResize = ref(false);
const selectedEntry = ref<FileInfo | null>(null);
const copiedEntry = ref<FileInfo | null>(null);
const contextMenu = ref<FileContextMenu | null>(null);
const contextMenuElement = ref<HTMLElement>();
const layoutStyle = computed(() => ({
  '--folder-width': `${folderWidth.value}px`,
}));

const chooseWorkspace = async () => {
  await workspace.chooseWorkspace();
  selectedEntry.value = null;
  preview.clear();
};
const openDirectory = async (path: string) => {
  workspace.selectDirectory(path);
  selectedEntry.value = null;
  preview.clear();
};
const selectEntry = (entry: FileInfo) => {
  selectedEntry.value = entry;
  if (!entry.isDirectory) void preview.preview(entry);
};
const refresh = () => void workspace.refreshDirectory();
const retryPreview = () => {
  if (preview.file) void preview.preview(preview.file);
};
const copySelectedEntry = (entry: FileInfo) => {
  copiedEntry.value = entry;
  selectedEntry.value = entry;
  contextMenu.value = null;
  ElMessage.success(`已复制 ${entry.name}`);
};
const pasteEntry = async (destinationDirectory = workspace.currentDirectory) => {
  const source = copiedEntry.value;
  contextMenu.value = null;
  if (!source || !destinationDirectory) return;
  try {
    const copied = await workspace.copyEntry(source.path, destinationDirectory);
    if (destinationDirectory === workspace.currentDirectory) selectedEntry.value = copied;
    ElMessage.success(`已粘贴 ${copied.name}`);
  } catch (error) {
    ElMessage.error(`粘贴失败：${error instanceof Error ? error.message : String(error)}`);
  }
};
const openContextMenu = (file: FileInfo, event: MouseEvent) => {
  selectedEntry.value = file;
  contextMenu.value = {
    file,
    x: Math.min(event.clientX, window.innerWidth - 176),
    y: Math.min(event.clientY, window.innerHeight - 100),
  };
};

const clamp = (value: number, min: number, max: number) => Math.max(min, Math.min(max, value));
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
  const maxWidth = window.innerWidth - MIN_PREVIEW_WIDTH - RESIZER_TOTAL_WIDTH;
  folderWidth.value = clamp(event.clientX - 4, MIN_FOLDER_WIDTH, maxWidth);
};
const stopResize = () => {
  activeResize.value = false;
  document.documentElement.style.cursor = '';
  document.removeEventListener('pointermove', resizePane);
  document.removeEventListener('pointerup', stopResize);
};
const closeContextMenuOnOutsideClick = (event: PointerEvent) => {
  if (contextMenuElement.value?.contains(event.target as Node)) return;
  contextMenu.value = null;
};
const handleKeyboardShortcut = (event: KeyboardEvent) => {
  const target = event.target as HTMLElement | null;
  if (target?.matches('input, textarea, [contenteditable="true"]')) return;
  if (!event.ctrlKey && !event.metaKey) return;

  if (event.key.toLowerCase() === 'c' && selectedEntry.value) {
    event.preventDefault();
    copySelectedEntry(selectedEntry.value);
  }
  if (event.key.toLowerCase() === 'v' && copiedEntry.value && workspace.currentDirectory) {
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
const loadWorkspaceHistory = (visible: boolean) => {
  if (visible) void history.loadWorkspaces();
};
const loadFileHistory = (visible: boolean) => {
  if (visible) void history.loadFiles();
};
const recentFileByPath = computed(
  () => new Map(history.recentFiles.map((item) => [item.path, item])),
);

const openRecentWorkspace = async (command: string) => {
  if (command === 'clear-workspaces') {
    await clearHistory('最近文件夹', () => history.clearWorkspaces());
    return;
  }
  await workspace.openWorkspace(command);
  selectedEntry.value = null;
  preview.clear();
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
  await workspace.openWorkspace(item.path.slice(0, separator));
  const file = {
    path: item.path,
    name: item.name,
    extension: item.extension,
    size: 0,
    modifiedAt: null,
    isDirectory: false,
  };
  selectedEntry.value = file;
  await preview.preview(file);
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

onMounted(() => {
  document.addEventListener('pointerdown', closeContextMenuOnOutsideClick);
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
});

onUnmounted(() => {
  stopResize();
  document.removeEventListener('pointerdown', closeContextMenuOnOutsideClick);
  window.removeEventListener('keydown', handleKeyboardShortcut);
  if (initialUpdateTimer !== undefined) window.clearTimeout(initialUpdateTimer);
  if (periodicUpdateTimer !== undefined) window.clearInterval(periodicUpdateTimer);
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
  grid-template-columns: auto minmax(120px, 1fr) 310px;
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
.search {
  min-width: 0;
}
.history-path {
  color: #98a2b3;
  display: block;
  font-size: 11px;
  max-width: 260px;
  overflow: hidden;
  text-overflow: ellipsis;
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
.file-context-menu {
  background: #fff;
  border: 1px solid #d8dee8;
  box-shadow: 0 8px 20px rgb(15 23 42 / 16%);
  min-width: 168px;
  padding: 4px;
  position: fixed;
  z-index: 20;
}
.file-context-menu button {
  align-items: center;
  background: transparent;
  border: 0;
  color: #344054;
  cursor: pointer;
  display: flex;
  font: inherit;
  gap: 8px;
  min-height: 32px;
  padding: 0 9px;
  text-align: left;
  width: 100%;
}
.file-context-menu button:hover:not(:disabled) {
  background: #f1f5f9;
}
.file-context-menu button:disabled {
  color: #98a2b3;
  cursor: not-allowed;
}
</style>
