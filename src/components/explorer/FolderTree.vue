<template>
  <div ref="treeElement" class="tree">
    <button
      v-if="workspace"
      class="tree-node root"
      :data-drop-path="workspace.path"
      :class="{ active: path === workspace.path, 'drop-target': dragOverPath === workspace.path }"
      @click="openRoot"
    >
      <el-icon><FolderOpened /></el-icon><span>{{ workspace.name }}</span>
    </button>
    <div v-if="workspace" class="children">
      <FolderNode
        :entries="entries"
        :depth="0"
        :active-directory="path"
        :selected-path="selectedPath"
        :dragged-path="draggedPath"
        :drag-over-path="dragOverPath"
        @open="emit('open', $event)"
        @select="emit('select', $event)"
        @contextmenu="forwardContextMenu"
        @pointer-drag-start="handlePointerDragStart"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { FolderOpened } from '@element-plus/icons-vue';
import { onUnmounted, ref } from 'vue';
import type { FileInfo, WorkspaceInfo } from '../../types/file';
import { useWorkspaceStore } from '../../stores/workspace';
import FolderNode from './FolderNode.vue';

const props = defineProps<{
  workspace: WorkspaceInfo | null;
  entries: FileInfo[];
  path: string;
  selectedPath?: string;
}>();
const emit = defineEmits<{
  open: [path: string];
  select: [file: FileInfo];
  contextmenu: [file: FileInfo, event: MouseEvent];
  move: [source: FileInfo, destinationDirectory: string];
}>();
const workspaceStore = useWorkspaceStore();
const treeElement = ref<HTMLElement | null>(null);
const draggedPath = ref('');
const draggedEntry = ref<FileInfo | null>(null);
const dragOverPath = ref('');
const pointerCandidate = ref<{ entry: FileInfo; x: number; y: number } | null>(null);
const pointerDragging = ref(false);
const clearPointerDrag = () => {
  window.removeEventListener('pointermove', handlePointerMove);
  window.removeEventListener('pointerup', handlePointerUp);
  window.removeEventListener('pointercancel', handlePointerUp);
  pointerCandidate.value = null;
  pointerDragging.value = false;
};
const handlePointerDragStart = (entry: FileInfo, event: PointerEvent) => {
  pointerCandidate.value = { entry, x: event.clientX, y: event.clientY };
  window.addEventListener('pointermove', handlePointerMove);
  window.addEventListener('pointerup', handlePointerUp);
  window.addEventListener('pointercancel', handlePointerUp);
};
const handlePointerMove = (event: PointerEvent) => {
  const candidate = pointerCandidate.value;
  if (!candidate) return;
  const distance = Math.hypot(event.clientX - candidate.x, event.clientY - candidate.y);
  if (!pointerDragging.value && distance < 6) return;
  pointerDragging.value = true;
  event.preventDefault();
  draggedPath.value = candidate.entry.path;
  draggedEntry.value = candidate.entry;
  const target = document
    .elementFromPoint(event.clientX, event.clientY)
    ?.closest<HTMLElement>('[data-drop-path]');
  if (target && treeElement.value?.contains(target)) {
    const path = target.dataset.dropPath;
    if (path && path !== candidate.entry.path) {
      dragOverPath.value = path;
      return;
    }
  }
  dragOverPath.value = '';
};
const handlePointerUp = (event: PointerEvent) => {
  const candidate = pointerCandidate.value;
  if (!candidate) {
    clearPointerDrag();
    return;
  }
  const target = document
    .elementFromPoint(event.clientX, event.clientY)
    ?.closest<HTMLElement>('[data-drop-path]');
  const destination =
    target && treeElement.value?.contains(target) ? target.dataset.dropPath || '' : '';
  const wasDragging = pointerDragging.value;
  clearPointerDrag();
  if (wasDragging && destination && destination !== candidate.entry.path) {
    emit('move', candidate.entry, destination);
  }
  draggedPath.value = '';
  draggedEntry.value = null;
  dragOverPath.value = '';
};
onUnmounted(clearPointerDrag);
const findEntry = (entries: FileInfo[], path: string): FileInfo | undefined => {
  for (const entry of entries) {
    if (entry.path === path) return entry;
    if (entry.isDirectory) {
      const found = findEntry(workspaceStore.directoryEntries[entry.path] ?? [], path);
      if (found) return found;
    }
  }
};

const openRoot = () => {
  if (props.workspace) emit('open', props.workspace.path);
};
const forwardContextMenu = (file: FileInfo, event: MouseEvent) => emit('contextmenu', file, event);
</script>

<style scoped lang="scss">
.tree {
  font-size: 13px;
  color: #4b5563;
}
.tree-node {
  align-items: center;
  background: transparent;
  border: 0;
  color: inherit;
  cursor: pointer;
  display: flex;
  font: inherit;
  gap: 7px;
  height: 30px;
  overflow: hidden;
  padding: 0 8px;
  text-align: left;
  width: 100%;
}
.tree-node:hover,
.tree-node.active {
  background: #e9f1ff;
  color: #1d4ed8;
}
.tree-node.drop-target {
  background: #dbeafe;
  box-shadow: inset 0 0 0 1px #3b82f6;
  color: #1d4ed8;
}
.tree-node span {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.root {
  font-weight: 600;
}
.children {
  padding-left: 6px;
}
</style>
