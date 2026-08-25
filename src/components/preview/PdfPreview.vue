<template>
  <div class="pdf-preview">
    <div ref="container" class="pdf-pages" />
    <div v-if="message" class="pdf-message">{{ message }}</div>
  </div>
</template>

<script setup lang="ts">
import { GlobalWorkerOptions, getDocument, type PDFDocumentLoadingTask } from 'pdfjs-dist';
import { onBeforeUnmount, ref, watch } from 'vue';

GlobalWorkerOptions.workerSrc = new URL(
  'pdfjs-dist/build/pdf.worker.min.mjs',
  import.meta.url,
).toString();

const props = defineProps<{ data: Uint8Array }>();
const container = ref<HTMLElement>();
const message = ref('正在加载 PDF...');
let loadingTask: PDFDocumentLoadingTask | undefined;
let renderVersion = 0;

const render = async () => {
  const version = ++renderVersion;
  const previousTask = loadingTask;
  loadingTask = undefined;
  if (previousTask) await previousTask.destroy().catch(() => undefined);
  const target = container.value;
  if (!target) return;
  target.replaceChildren();
  message.value = '正在加载 PDF...';

  try {
    const task = getDocument({ data: props.data.slice() });
    loadingTask = task;
    const pdf = await task.promise;
    if (version !== renderVersion) {
      await task.destroy();
      return;
    }
    const width = Math.max(target.clientWidth - 48, 320);
    for (let number = 1; number <= pdf.numPages; number += 1) {
      const page = await pdf.getPage(number);
      if (version !== renderVersion) return;
      const baseViewport = page.getViewport({ scale: 1 });
      const viewport = page.getViewport({ scale: Math.max(1, width / baseViewport.width) });
      const canvas = document.createElement('canvas');
      canvas.width = Math.ceil(viewport.width);
      canvas.height = Math.ceil(viewport.height);
      canvas.setAttribute('aria-label', `PDF 第 ${number} 页`);
      target.append(canvas);
      await page.render({ canvas, viewport }).promise;
    }
    if (version === renderVersion) message.value = '';
  } catch (error) {
    if (version === renderVersion) {
      message.value = `PDF 加载失败：${error instanceof Error ? error.message : String(error)}`;
    }
  }
};

watch(
  () => props.data,
  () => void render(),
  { immediate: true },
);
onBeforeUnmount(() => {
  renderVersion += 1;
  void loadingTask?.destroy();
});
</script>

<style scoped lang="scss">
.pdf-preview {
  align-items: center;
  background: #eef1f5;
  display: flex;
  flex-direction: column;
  min-height: 100%;
  padding: 24px;
}
.pdf-pages {
  align-items: center;
  display: flex;
  flex-direction: column;
  gap: 20px;
  width: 100%;
}
.pdf-pages canvas {
  background: #fff;
  box-shadow: 0 3px 16px rgb(15 23 42 / 16%);
  max-width: 100%;
}
.pdf-message {
  color: #667085;
  font-size: 13px;
  padding: 24px;
}
</style>
