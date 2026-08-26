<template>
  <el-dialog v-model="visible" title="文件内容搜索" width="720px" :close-on-click-modal="false">
    <el-form @submit.prevent="emit('search')">
      <el-input
        v-model="query"
        autofocus
        clearable
        placeholder="搜索当前工作区中的文本、代码和 Markdown"
        @keyup.enter="emit('search')"
      >
        <template #append>
          <el-button :loading="loading" native-type="submit">搜索</el-button>
        </template>
      </el-input>
    </el-form>
    <el-alert v-if="error" class="search-error" type="error" :title="error" :closable="false" />
    <div v-loading="loading" class="search-results">
      <el-empty v-if="!loading && searched && !results.length" description="没有找到匹配内容" />
      <button
        v-for="result in results"
        :key="`${result.path}:${result.lineNumber}`"
        class="search-result"
        type="button"
        @click="emit('select', result)"
      >
        <span class="search-result-title">{{ result.name }} : {{ result.lineNumber }}</span>
        <span class="search-result-path">{{ result.path }}</span>
        <span class="search-result-content">{{ result.lineContent }}</span>
      </button>
    </div>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { ContentSearchResult } from '../../api/file';

const props = defineProps<{
  modelValue: boolean;
  query: string;
  results: ContentSearchResult[];
  loading: boolean;
  error: string;
}>();
const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'update:query': [value: string];
  search: [];
  select: [result: ContentSearchResult];
}>();
const visible = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit('update:modelValue', value),
});
const query = computed({
  get: () => props.query,
  set: (value: string) => emit('update:query', value),
});
const searched = computed(() => Boolean(props.query.trim() || props.error));
</script>

<style scoped lang="scss">
.search-error {
  margin-top: 12px;
}
.search-results {
  max-height: min(54vh, 520px);
  min-height: 110px;
  overflow: auto;
  padding-top: 12px;
}
.search-result {
  background: transparent;
  border: 0;
  border-bottom: 1px solid #edf0f4;
  color: #344054;
  cursor: pointer;
  display: grid;
  font: inherit;
  gap: 4px;
  padding: 10px 8px;
  text-align: left;
  width: 100%;
}
.search-result:hover {
  background: #f5f8fc;
}
.search-result-title {
  color: #1d4ed8;
  font-size: 13px;
  font-weight: 600;
}
.search-result-path,
.search-result-content {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.search-result-path {
  color: #8b95a4;
  font-family: Consolas, monospace;
  font-size: 11px;
}
.search-result-content {
  color: #475467;
  font-family: Consolas, monospace;
  font-size: 12px;
}
</style>
