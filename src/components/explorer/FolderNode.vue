<template>
  <div v-for="entry in filteredEntries" :key="entry.path">
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
        :style="{ paddingLeft: `${depth * 16 + 10}px` }"
        @click="emit('select', entry)"
        @contextmenu.prevent="emit('contextmenu', entry, $event)"
      >
        <el-icon><Document /></el-icon><span>{{ entry.name }}</span>
      </button>
    </el-tooltip>
    <template v-else>
      <button
        class="tree-node"
        :class="{ active: activeDirectory === entry.path }"
        :style="{ paddingLeft: `${depth * 16 + 10}px` }"
        @click="toggle(entry.path)"
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
          :filter="filter"
          @open="emit('open', $event)"
          @select="emit('select', $event)"
          @contextmenu="forwardContextMenu"
        />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { CaretBottom, CaretRight, Document, Folder, FolderOpened } from '@element-plus/icons-vue';
import { computed, ref } from 'vue';
import { useWorkspaceStore } from '../../stores/workspace';
import type { FileInfo } from '../../types/file';

const props = defineProps<{
  entries: FileInfo[];
  depth: number;
  activeDirectory: string;
  selectedPath?: string;
  filter: string;
}>();
const emit = defineEmits<{
  open: [path: string];
  select: [file: FileInfo];
  contextmenu: [file: FileInfo, event: MouseEvent];
}>();
const workspace = useWorkspaceStore();
const expandedPaths = ref(new Set<string>());
const filteredEntries = computed(() => {
  const keyword = props.filter.trim().toLowerCase();
  return keyword
    ? props.entries.filter((entry) => entry.name.toLowerCase().includes(keyword))
    : props.entries;
});

const toggle = async (path: string) => {
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
    await workspace.ensureDirectoryLoaded(path);
  } catch {
    // 错误由工作区状态显示，已展开的节点允许用户再次尝试。
  }
};

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
.tree-node.active {
  background: #e9f1ff;
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
