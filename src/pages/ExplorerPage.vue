<template>
  <main class="explorer-page">
    <header class="toolbar">
      <div class="toolbar-left">
        <span class="brand"
          ><el-icon><Files /></el-icon>FilePreview</span
        ><el-button :icon="FolderOpened" type="primary" @click="chooseWorkspace"
          >打开文件夹</el-button
        ><el-button
          :icon="Refresh"
          :disabled="!workspace.workspace"
          circle
          aria-label="刷新文件夹"
          @click="refresh"
        />
      </div>
      <div class="path">{{ workspace.currentDirectory || '选择一个本地文件夹' }}</div>
      <el-input
        v-model="workspace.filter"
        class="search"
        :prefix-icon="Search"
        placeholder="搜索当前目录"
        clearable
      />
    </header>
    <div v-if="workspace.error" class="error-bar">
      <el-icon><WarningFilled /></el-icon>{{ workspace.error }}
    </div>
    <div class="workspace-layout">
      <aside class="folder-pane">
        <div class="pane-title">资源管理器</div>
        <FolderTree
          :workspace="workspace.workspace"
          :path="workspace.currentDirectory"
          @open="openDirectory"
        />
      </aside>
      <section class="list-pane">
        <div class="pane-title">{{ workspace.workspace ? '文件' : '尚未打开工作区' }}</div>
        <FileList
          :entries="workspace.visibleEntries"
          :loading="workspace.loading"
          :selected-path="preview.file?.path"
          @select="selectFile"
          @open="openEntry"
        />
      </section>
      <PreviewPanel
        class="preview-pane"
        :file="preview.file"
        :content="preview.content"
        :loading="preview.loading"
        :error="preview.error"
      />
    </div>
  </main>
</template>

<script setup lang="ts">
import { Files, FolderOpened, Refresh, Search, WarningFilled } from '@element-plus/icons-vue';
import { usePreviewStore } from '../stores/preview';
import { useWorkspaceStore } from '../stores/workspace';
import type { FileInfo } from '../types/file';
import FileList from '../components/explorer/FileList.vue';
import FolderTree from '../components/explorer/FolderTree.vue';
import PreviewPanel from '../components/preview/PreviewPanel.vue';

const workspace = useWorkspaceStore();
const preview = usePreviewStore();

const chooseWorkspace = async () => {
  await workspace.chooseWorkspace();
  preview.clear();
};
const openDirectory = async (path: string) => {
  await workspace.loadDirectory(path);
  preview.clear();
};
const selectFile = (file: FileInfo) => {
  if (!file.isDirectory) void preview.preview(file);
};
const openEntry = (file: FileInfo) => {
  if (file.isDirectory) void openDirectory(file.path);
  else void preview.preview(file);
};
const refresh = () => void workspace.loadDirectory();
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
  grid-template-columns: auto minmax(180px, 1fr) 240px;
  height: 60px;
  padding: 0 18px;
}
.toolbar-left {
  align-items: center;
  display: flex;
  gap: 9px;
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
  grid-template-columns: 230px minmax(370px, 1fr) minmax(430px, 1.35fr);
  min-height: 0;
}
.folder-pane,
.list-pane {
  background: #fff;
  border-right: 1px solid #e1e6ed;
  min-width: 0;
  overflow: auto;
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
.preview-pane {
  background: #fff;
  min-width: 0;
}
</style>
