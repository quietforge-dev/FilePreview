<template>
  <section class="workspace-search" aria-label="工作区搜索">
    <el-input
      ref="inputElement"
      :model-value="query"
      :disabled="!workspaceReady"
      :placeholder="mode === 'name' ? '搜索文件和文件夹' : '搜索文件内容'"
      clearable
      @update:model-value="emit('update:query', $event)"
      @keyup.enter="emit('search')"
    >
      <template #prefix
        ><el-icon><Search /></el-icon
      ></template>
    </el-input>
    <div class="search-actions">
      <el-radio-group
        :model-value="mode"
        size="small"
        aria-label="搜索范围"
        @update:model-value="emit('update:mode', $event)"
      >
        <el-radio-button label="name">名称</el-radio-button>
        <el-radio-button label="content">内容</el-radio-button>
      </el-radio-group>
      <el-tooltip v-if="mode === 'content'" content="搜索文件内容" placement="bottom">
        <el-button
          :icon="Search"
          :loading="loading"
          :disabled="!query.trim() || !workspaceReady"
          circle
          aria-label="搜索文件内容"
          @click="emit('search')"
        />
      </el-tooltip>
    </div>
    <div v-if="showResults" v-loading="loading" class="search-results">
      <el-alert v-if="error" :title="error" type="error" :closable="false" />
      <el-empty
        v-else-if="searched && !results.length"
        :image-size="54"
        description="没有找到匹配项"
      />
      <template v-else>
        <button
          v-for="result in results"
          :key="resultKey(result)"
          class="search-result"
          type="button"
          @click="openResult(result)"
        >
          <el-icon><component :is="resultIcon(result)" /></el-icon>
          <span class="search-result-main">
            <span class="search-result-name">{{ result.name }}</span>
            <span class="search-result-path">{{ result.path }}</span>
            <span v-if="isContentResult(result)" class="search-result-content">
              {{ result.lineNumber }}: {{ result.lineContent }}
            </span>
          </span>
        </button>
      </template>
    </div>
  </section>
</template>

<script setup lang="ts">
import { Document, Folder, Search } from '@element-plus/icons-vue';
import { computed, ref } from 'vue';
import type { ContentSearchResult } from '../../api/file';
import type { FileInfo } from '../../types/file';
import type { WorkspaceSearchMode } from '../../stores/search';

type SearchResult = FileInfo | ContentSearchResult;

const props = defineProps<{
  workspaceReady: boolean;
  mode: WorkspaceSearchMode;
  query: string;
  nameResults: FileInfo[];
  contentResults: ContentSearchResult[];
  loading: boolean;
  error: string;
  searched: boolean;
}>();
const emit = defineEmits<{
  'update:mode': [mode: WorkspaceSearchMode];
  'update:query': [query: string];
  search: [];
  openFile: [result: FileInfo | ContentSearchResult];
  openDirectory: [entry: FileInfo];
}>();
const inputElement = ref<{ focus: () => void }>();
const results = computed<SearchResult[]>(() =>
  props.mode === 'name' ? props.nameResults : props.contentResults,
);
const showResults = computed(
  () => Boolean(props.query.trim()) && (props.searched || props.loading),
);
const isContentResult = (result: SearchResult): result is ContentSearchResult =>
  'lineNumber' in result;
const resultIcon = (result: SearchResult) =>
  !isContentResult(result) && result.isDirectory ? Folder : Document;
const resultKey = (result: SearchResult) =>
  isContentResult(result) ? `${result.path}:${result.lineNumber}` : result.path;
const openResult = (result: SearchResult) => {
  if (isContentResult(result)) {
    emit('openFile', result);
    return;
  }
  if (result.isDirectory) emit('openDirectory', result);
  else emit('openFile', result);
};

defineExpose({
  focus: () => inputElement.value?.focus(),
});
</script>

<style scoped lang="scss">
.workspace-search {
  background: #fff;
  border-bottom: 1px solid #e4e9f0;
  display: grid;
  gap: 8px;
  padding: 8px;
  position: sticky;
  top: 0;
  z-index: 1;
}
.search-actions {
  align-items: center;
  display: flex;
  justify-content: space-between;
}
.search-actions :deep(.el-radio-button__inner) {
  font-size: 12px;
  padding: 5px 9px;
}
.search-results {
  border-top: 1px solid #edf0f4;
  max-height: min(44vh, 420px);
  min-height: 54px;
  overflow: auto;
  padding-top: 4px;
}
.search-result {
  align-items: start;
  background: transparent;
  border: 0;
  color: #475467;
  cursor: pointer;
  display: flex;
  font: inherit;
  gap: 7px;
  padding: 7px 5px;
  text-align: left;
  width: 100%;
}
.search-result:hover {
  background: #eef4ff;
  color: #1d4ed8;
}
.search-result-main {
  display: grid;
  gap: 2px;
  min-width: 0;
}
.search-result-name,
.search-result-path,
.search-result-content {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.search-result-name {
  color: inherit;
  font-size: 13px;
  font-weight: 600;
}
.search-result-path,
.search-result-content {
  color: #8b95a4;
  font-family: Consolas, monospace;
  font-size: 11px;
}
.search-result-content {
  color: #667085;
}
</style>
