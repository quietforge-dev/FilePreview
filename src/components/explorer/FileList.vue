<template>
  <el-table
    :data="entries"
    v-loading="loading"
    height="100%"
    class="file-list"
    @row-dblclick="emit('open', $event)"
  >
    <el-table-column label="名称" min-width="220">
      <template #default="{ row }: { row: FileInfo }">
        <button
          class="file-name"
          :class="{ selected: selectedPath === row.path }"
          @click="emit('select', row)"
          @dblclick.stop="emit('open', row)"
        >
          <el-icon :class="row.isDirectory ? 'folder-icon' : 'file-icon'"
            ><component :is="row.isDirectory ? Folder : Document"
          /></el-icon>
          <span>{{ row.name }}</span>
        </button>
      </template>
    </el-table-column>
    <el-table-column label="类型" width="110"
      ><template #default="{ row }: { row: FileInfo }">{{
        row.isDirectory ? '文件夹' : row.extension.toUpperCase() || '文件'
      }}</template></el-table-column
    >
    <el-table-column label="大小" width="100"
      ><template #default="{ row }: { row: FileInfo }">{{
        row.isDirectory ? '-' : formatSize(row.size)
      }}</template></el-table-column
    >
    <el-table-column label="修改时间" width="170"
      ><template #default="{ row }: { row: FileInfo }">{{
        formatTime(row.modifiedAt)
      }}</template></el-table-column
    >
  </el-table>
</template>

<script setup lang="ts">
import { Document, Folder } from '@element-plus/icons-vue';
import type { FileInfo } from '../../types/file';

defineProps<{ entries: FileInfo[]; loading: boolean; selectedPath?: string }>();
const emit = defineEmits<{ select: [file: FileInfo]; open: [file: FileInfo] }>();

const formatSize = (size: number) => {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
  return `${(size / 1024 / 1024).toFixed(1)} MB`;
};
const formatTime = (seconds: number | null) =>
  seconds ? new Date(seconds * 1000).toLocaleString('zh-CN', { hour12: false }) : '-';
</script>

<style scoped lang="scss">
.file-list {
  --el-table-header-bg-color: #f8fafc;
  --el-table-row-hover-bg-color: #f6f9ff;
}
.file-name {
  align-items: center;
  background: transparent;
  border: 0;
  color: #374151;
  cursor: pointer;
  display: flex;
  font: inherit;
  gap: 8px;
  height: 32px;
  min-width: 0;
  padding: 0 6px;
  text-align: left;
  width: 100%;
}
.file-name.selected {
  color: #1d4ed8;
  font-weight: 600;
}
.file-name span {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.folder-icon {
  color: #d49a18;
}
.file-icon {
  color: #6b7280;
}
</style>
