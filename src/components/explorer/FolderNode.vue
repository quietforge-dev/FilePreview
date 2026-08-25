<template>
  <div v-if="directories.length">
    <div v-for="directory in directories" :key="directory.path">
      <button
        class="tree-node"
        :style="{ paddingLeft: `${depth * 14 + 8}px` }"
        @click="toggle(directory.path)"
      >
        <el-icon
          ><component :is="expandedPaths.has(directory.path) ? FolderOpened : Folder"
        /></el-icon>
        <span>{{ directory.name }}</span>
      </button>
      <div v-if="expandedPaths.has(directory.path)" class="nested">
        <FolderNode :path="directory.path" :depth="depth + 1" @open="emit('open', $event)" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Folder, FolderOpened } from '@element-plus/icons-vue';
import { computed, ref, watch } from 'vue';
import { listDirectory } from '../../api/file';
import type { FileInfo } from '../../types/file';

const props = defineProps<{ path: string; depth: number }>();
const emit = defineEmits<{ open: [path: string] }>();
const entries = ref<FileInfo[]>([]);
const expandedPaths = ref(new Set<string>());

const directories = computed(() => entries.value.filter((entry) => entry.isDirectory));

const load = async () => {
  entries.value = await listDirectory(props.path);
};

const toggle = async (path: string) => {
  const next = new Set(expandedPaths.value);
  if (next.has(path)) next.delete(path);
  else next.add(path);
  expandedPaths.value = next;
  emit('open', path);
};

watch(
  () => props.path,
  () => void load(),
  { immediate: true },
);
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
</style>
