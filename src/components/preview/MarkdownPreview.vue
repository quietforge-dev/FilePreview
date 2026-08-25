<template><article class="markdown-body" v-html="html" /></template>

<script setup lang="ts">
defineProps<{ html: string }>();
</script>

<style scoped lang="scss">
.markdown-body {
  color: #273142;
  font-size: 15px;
  line-height: 1.75;
  margin: 0 auto;
  max-width: 900px;
  padding: 34px 48px 72px;
}
.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3) {
  color: #111827;
  line-height: 1.3;
  margin: 1.5em 0 0.65em;
}
.markdown-body :deep(h1) {
  border-bottom: 1px solid #e5e7eb;
  font-size: 2em;
  padding-bottom: 0.4em;
}
.markdown-body :deep(h2) {
  font-size: 1.5em;
}
.markdown-body :deep(h3) {
  font-size: 1.2em;
}
.markdown-body :deep(pre) {
  background: #182230;
  border-radius: 6px;
  color: #e5edf7;
  overflow: auto;
  padding: 16px;
}
.markdown-body :deep(code) {
  background: #eef1f5;
  border-radius: 3px;
  font-family: Consolas, monospace;
  font-size: 0.9em;
  padding: 2px 5px;
}
.markdown-body :deep(pre code) {
  background: transparent;
  padding: 0;
}
.markdown-body :deep(blockquote) {
  border-left: 4px solid #7aa8e8;
  color: #556070;
  margin: 1em 0;
  padding-left: 16px;
}
.markdown-body :deep(table) {
  border-collapse: collapse;
  width: 100%;
}
.markdown-body :deep(th),
.markdown-body :deep(td) {
  border: 1px solid #dce1e8;
  padding: 8px 10px;
  text-align: left;
}
.markdown-body :deep(th) {
  background: #f5f7fa;
}
.markdown-body :deep(img) {
  max-width: 100%;
}
</style>
