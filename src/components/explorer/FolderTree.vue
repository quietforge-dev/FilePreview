<template>
  <div class="tree">
    <button
      v-if="workspace"
      class="tree-node root"
      :class="{ active: path === workspace.path }"
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
        @open="emit('open', $event)"
        @select="emit('select', $event)"
        @contextmenu="forwardContextMenu"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { FolderOpened } from '@element-plus/icons-vue';
import type { FileInfo, WorkspaceInfo } from '../../types/file';
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
}>();

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
