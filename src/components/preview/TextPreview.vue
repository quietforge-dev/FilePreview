<template>
  <div class="text-preview">
    <div class="language">{{ language }}</div>
    <pre><code>{{ content }}</code></pre>
  </div>
</template>

<script setup lang="ts">
defineProps<{ content: string; language: string }>();
</script>

<style scoped lang="scss">
.text-preview {
  background: #1f2937;
  color: #e5edf7;
  height: 100%;
  overflow: auto;
  padding: 0 0 28px;
}
.language {
  background: #263446;
  color: #bac7d8;
  font:
    12px/34px Inter,
    sans-serif;
  height: 34px;
  padding: 0 20px;
  text-transform: uppercase;
}
.text-preview pre {
  font:
    13px/1.65 Consolas,
    'Courier New',
    monospace;
  margin: 0;
  padding: 18px 24px;
  tab-size: 2;
  white-space: pre;
}
.text-preview code {
  color: inherit;
}
</style>
