<template>
  <section class="preview-panel">
    <header class="preview-header">
      <span>{{ file?.name ?? '预览' }}</span
      ><span v-if="file" class="extension">{{ file.extension || '文件' }}</span>
    </header>
    <div v-loading="loading" class="preview-body">
      <el-empty v-if="!file && !loading" description="选择一个文件开始预览" :image-size="86" />
      <el-result v-else-if="error" icon="error" title="无法预览" :sub-title="error" />
      <MarkdownPreview v-else-if="content?.kind === 'markdown'" :html="content.html" />
      <TextPreview
        v-else-if="content?.kind === 'text'"
        :content="content.content"
        :language="content.language"
      />
      <OfficePreview
        v-else-if="content?.kind === 'office'"
        :office-type="content.officeType"
        :html="content.html"
        :slides="content.slides"
      />
      <div v-else-if="content?.kind === 'image'" class="image-preview">
        <img :src="content.url" :alt="file?.name" />
      </div>
      <el-result
        v-else-if="content?.kind === 'unsupported'"
        icon="warning"
        title="暂不支持预览"
        :sub-title="content.message"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import type { FileInfo, PreviewContent } from '../../types/file';
import MarkdownPreview from './MarkdownPreview.vue';
import OfficePreview from './OfficePreview.vue';
import TextPreview from './TextPreview.vue';

defineProps<{
  file: FileInfo | null;
  content: PreviewContent | null;
  loading: boolean;
  error: string;
}>();
</script>

<style scoped lang="scss">
.preview-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-width: 0;
}
.preview-header {
  align-items: center;
  border-bottom: 1px solid #e1e6ed;
  color: #3e4957;
  display: flex;
  font-size: 13px;
  font-weight: 600;
  height: 42px;
  justify-content: space-between;
  padding: 0 16px;
}
.extension {
  color: #8b95a4;
  font-size: 11px;
  font-weight: 500;
  text-transform: uppercase;
}
.preview-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
}
.image-preview {
  align-items: center;
  background: repeating-conic-gradient(#f2f4f7 0% 25%, #fff 0% 50%) 50% / 20px 20px;
  display: flex;
  justify-content: center;
  min-height: 100%;
  padding: 32px;
}
.image-preview img {
  max-height: 100%;
  max-width: 100%;
  object-fit: contain;
}
</style>
