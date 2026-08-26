<template>
  <el-dialog v-model="visible" :title="title" width="680px">
    <div v-loading="loading" class="history-list">
      <el-empty v-if="!loading && !items.length" :description="emptyText" />
      <button
        v-for="item in items"
        :key="item.path"
        class="history-item"
        type="button"
        @click="emit('select', item.path)"
      >
        <span class="history-item-name">{{ item.name }}</span>
        <span class="history-item-path">{{ item.path }}</span>
      </button>
    </div>
    <template #footer>
      <el-button :disabled="!items.length" @click="emit('clear')">清空记录</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue';

type HistoryItem = { path: string; name: string };

const props = defineProps<{
  modelValue: boolean;
  title: string;
  items: HistoryItem[];
  loading: boolean;
  emptyText: string;
}>();
const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  select: [path: string];
  clear: [];
}>();
const visible = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit('update:modelValue', value),
});
</script>

<style scoped lang="scss">
.history-list {
  max-height: min(54vh, 520px);
  min-height: 110px;
  overflow: auto;
}
.history-item {
  background: transparent;
  border: 0;
  border-bottom: 1px solid #edf0f4;
  color: #344054;
  cursor: pointer;
  display: grid;
  font: inherit;
  gap: 4px;
  padding: 11px 8px;
  text-align: left;
  width: 100%;
}
.history-item:hover {
  background: #f5f8fc;
}
.history-item-name {
  color: #1d4ed8;
  font-size: 13px;
  font-weight: 600;
}
.history-item-path {
  color: #8b95a4;
  font-family: Consolas, monospace;
  font-size: 11px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
