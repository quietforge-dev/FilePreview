<template>
  <div ref="treeElement" class="tree">
    <button
      v-if="workspace"
      class="tree-node root"
      :data-drop-path="workspace.path"
      :data-tree-path="workspace.path"
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
        :expanded-paths="expandedPaths"
        @toggle="toggleDirectory"
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
import { nextTick, onUnmounted, ref, watch } from 'vue';
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
const expandedPaths = ref(new Set<string>());
const draggedPath = ref('');
const draggedEntry = ref<FileInfo | null>(null);
const dragOverPath = ref('');
const pointerCandidate = ref<{ entry: FileInfo; x: number; y: number } | null>(null);
const pointerDragging = ref(false);
let revealVersion = 0;

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

const normalizedSeparators = (path: string) => path.replaceAll('\\', '/');
const pathKey = (path: string) => {
  const normalized = normalizedSeparators(path).replace(/\/+$/, '');
  return (normalized || '/').toLowerCase();
};
const joinPath = (base: string, child: string, separator: '\\' | '/') => {
  if (base === separator) return `${base}${child}`;
  return `${base.replace(/[\\/]+$/, '')}${separator}${child}`;
};
const directoryChainForFile = (filePath: string, workspacePath: string) => {
  const root = normalizedSeparators(workspacePath).replace(/\/+$/, '') || '/';
  const target = normalizedSeparators(filePath);
  const rootKey = pathKey(root);
  const targetKey = pathKey(target);
  const isInsideRoot =
    rootKey === '/' ? targetKey.startsWith('/') : targetKey.startsWith(`${rootKey}/`);
  if (targetKey !== rootKey && !isInsideRoot) return null;

  const relative = target.slice(root.length).replace(/^\/+/, '');
  const parts = relative.split('/').filter(Boolean);
  const separator: '\\' | '/' = workspacePath.includes('\\') ? '\\' : '/';
  const directories: string[] = [];
  let current = workspacePath;
  for (const part of parts.slice(0, -1)) {
    current = joinPath(current, part, separator);
    directories.push(current);
  }
  return directories;
};

const toggleDirectory = async (path: string) => {
  const next = new Set(expandedPaths.value);
  if (next.has(path)) {
    next.delete(path);
    expandedPaths.value = next;
    emit('open', path);
    return;
  }

  next.add(path);
  expandedPaths.value = next;
  emit('open', path);
  try {
    await workspaceStore.ensureDirectoryLoaded(path);
  } catch {
    // 错误由工作区状态显示，已展开的节点允许用户再次尝试。
  }
};

const scrollToPath = async (path: string) => {
  await nextTick();
  const target = [
    ...(treeElement.value?.querySelectorAll<HTMLElement>('[data-tree-path]') ?? []),
  ].find(
    (element) => element.dataset.treePath && pathKey(element.dataset.treePath) === pathKey(path),
  );
  target?.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
};

const revealPath = async (filePath: string) => {
  const version = ++revealVersion;
  const workspacePath = props.workspace?.path;
  if (!workspacePath) return;

  const directories = directoryChainForFile(filePath, workspacePath);
  if (!directories) return;

  const next = new Set(expandedPaths.value);
  for (const directory of directories) {
    if (version !== revealVersion) return;
    next.add(directory);
    expandedPaths.value = new Set(next);
    try {
      await workspaceStore.ensureDirectoryLoaded(directory);
    } catch {
      return;
    }
  }

  if (version !== revealVersion) return;
  expandedPaths.value = next;
  await scrollToPath(filePath);
};

defineExpose({ revealPath });

watch(
  () => props.workspace?.path,
  () => {
    revealVersion += 1;
    expandedPaths.value = new Set();
  },
);

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
