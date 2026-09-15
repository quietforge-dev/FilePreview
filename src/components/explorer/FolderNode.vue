<template>
  <div v-for="entry in entries" :key="entry.path">
    <el-tooltip v-if="!entry.isDirectory" placement="right" :show-after="350">
      <template #content>
        <div class="file-details">
          <strong>{{ entry.name }}</strong>
          <span>类型：{{ entry.extension.toUpperCase() || '文件' }}</span>
          <span>大小：{{ formatSize(entry.size) }}</span>
          <span>修改时间：{{ formatTime(entry.modifiedAt) }}</span>
        </div>
      </template>
      <button
        class="tree-node"
        :class="{ active: selectedPath === entry.path }"
        :data-tree-path="entry.path"
        :style="{ paddingLeft: `${depth * 16 + 10}px` }"
        @click="emit('select', entry)"
        @pointerdown.left="emit('pointer-drag-start', entry, $event)"
        @contextmenu.prevent="emit('contextmenu', entry, $event)"
      >
        <el-icon><Document /></el-icon><span>{{ entry.name }}</span>
      </button>
    </el-tooltip>
    <template v-else>
      <button
        class="tree-node"
        :class="{
          active: activeDirectory === entry.path,
          'drop-target': dragOverPath === entry.path,
        }"
        :style="{ paddingLeft: `${depth * 16 + 10}px` }"
        @click="emit('toggle', entry.path)"
        @pointerdown.left="emit('pointer-drag-start', entry, $event)"
        :data-drop-path="entry.path"
        :data-tree-path="entry.path"
        @contextmenu.prevent="emit('contextmenu', entry, $event)"
      >
        <el-icon class="expand-icon"
          ><component :is="expandedPaths.has(entry.path) ? CaretBottom : CaretRight"
        /></el-icon>
        <el-icon><component :is="expandedPaths.has(entry.path) ? FolderOpened : Folder" /></el-icon>
        <span>{{ entry.name }}</span>
      </button>
      <div v-if="expandedPaths.has(entry.path)" class="nested">
        <div v-if="workspace.loadingDirectories[entry.path]" class="tree-loading">正在加载...</div>
        <FolderNode
          v-else
          :entries="workspace.directoryEntries[entry.path] ?? []"
          :depth="depth + 1"
          :active-directory="activeDirectory"
          :selected-path="selectedPath"
          :dragged-path="draggedPath"
          :drag-over-path="dragOverPath"
          :expanded-paths="expandedPaths"
          @toggle="emit('toggle', $event)"
          @open="emit('open', $event)"
          @select="emit('select', $event)"
          @contextmenu="forwardContextMenu"
          @pointer-drag-start="(file, event) => emit('pointer-drag-start', file, event)"
        />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { CaretBottom, CaretRight, Document, Folder, FolderOpened } from '@element-plus/icons-vue';
import { useWorkspaceStore } from '../../stores/workspace';
import type { FileInfo } from '../../types/file';

const props = defineProps<{
  entries: FileInfo[];
  depth: number;
  activeDirectory: string;
  selectedPath?: string;
  draggedPath?: string;
  dragOverPath?: string;
  expandedPaths: Set<string>;
}>();
const emit = defineEmits<{
  open: [path: string];
  select: [file: FileInfo];
  contextmenu: [file: FileInfo, event: MouseEvent];
  'pointer-drag-start': [file: FileInfo, event: PointerEvent];
  toggle: [path: string];
}>();
const workspace = useWorkspaceStore();

const forwardContextMenu = (file: FileInfo, event: MouseEvent) => emit('contextmenu', file, event);

const formatSize = (size: number) => {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
  return `${(size / 1024 / 1024).toFixed(1)} MB`;
};
const formatTime = (seconds: number | null) =>
  seconds ? new Date(seconds * 1000).toLocaleString('zh-CN', { hour12: false }) : '-';
</script>

<style scoped lang="scss">
.tree-node {
  align-items: center;
  background: transparent;
  border: 0;
  color: #4b5563;
  cursor: pointer;
  display: flex;
  font: inherit;
  font-size: 13px;
  gap: 7px;
  height: 30px;
  overflow: hidden;
  padding-right: 8px;
  text-align: left;
  width: 100%;
}
.tree-node {
  cursor: grab;
  user-select: none;
}
.tree-node:active {
  cursor: grabbing;
}
.tree-node.active {
  background: #e9f1ff;
  color: #1d4ed8;
}
.tree-node.drop-target {
  background: #dbeafe;
  box-shadow: inset 0 0 0 1px #3b82f6;
  color: #1d4ed8;
}
.expand-icon {
  color: #98a2b3;
  font-size: 11px;
  margin-right: -4px;
}
.tree-node:hover {
  background: #eef4ff;
  color: #1d4ed8;
}
.tree-node span {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.nested {
  border-left: 1px solid #edf0f4;
  margin-left: 13px;
}
.tree-loading {
  color: #8a94a3;
  font-size: 12px;
  padding: 7px 10px 7px 34px;
}
.file-details {
  display: grid;
  font-size: 12px;
  gap: 4px;
  line-height: 1.35;
  max-width: 300px;
}
.file-details strong {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
