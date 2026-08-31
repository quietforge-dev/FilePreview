<template>
  <article ref="article" class="markdown-body">
    <div class="markdown-content" v-html="html" />
  </article>
</template>

<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';

const props = defineProps<{ html: string }>();
const emit = defineEmits<{ activeHeadingChange: [id: string | null] }>();
const article = ref<HTMLElement | null>(null);
let observer: IntersectionObserver | null = null;

const stopObservingHeadings = () => {
  observer?.disconnect();
  observer = null;
};
const observeHeadings = async () => {
  stopObservingHeadings();
  await nextTick();
  const root = article.value;
  const headings = article.value?.querySelectorAll<HTMLElement>(
    'h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]',
  );
  if (!headings?.length) {
    emit('activeHeadingChange', null);
    return;
  }
  emit('activeHeadingChange', headings[0].id);
  observer = new IntersectionObserver(
    (entries) => {
      const visible = entries
        .filter((entry) => entry.isIntersecting)
        .sort((left, right) => left.boundingClientRect.top - right.boundingClientRect.top);
      const heading = visible[0]?.target as HTMLElement | undefined;
      if (heading) emit('activeHeadingChange', heading.id);
    },
    { root, rootMargin: '-8% 0px -72% 0px', threshold: [0, 1] },
  );
  headings.forEach((heading) => observer?.observe(heading));
};
const scrollToHeading = (id: string) => {
  const heading = article.value?.querySelector<HTMLElement>(`#${CSS.escape(id)}`);
  heading?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  if (heading) emit('activeHeadingChange', id);
};

watch(
  () => props.html,
  () => void observeHeadings(),
  { flush: 'post' },
);
onMounted(() => void observeHeadings());
onBeforeUnmount(stopObservingHeadings);
defineExpose({ scrollToHeading });
</script>

<style scoped lang="scss">
.markdown-body {
  flex: 1;
  height: 100%;
  min-width: 0;
  overflow: auto;
}
.markdown-content {
  color: #273142;
  font-size: 15px;
  line-height: 1.75;
  margin: 0 auto;
  max-width: 900px;
  padding: 34px 48px 72px;
}
.markdown-content :deep(h1),
.markdown-content :deep(h2),
.markdown-content :deep(h3) {
  color: #111827;
  line-height: 1.3;
  margin: 1.5em 0 0.65em;
}
.markdown-content :deep(h1) {
  border-bottom: 1px solid #e5e7eb;
  font-size: 2em;
  padding-bottom: 0.4em;
}
.markdown-content :deep(h2) {
  font-size: 1.5em;
}
.markdown-content :deep(h3) {
  font-size: 1.2em;
}
.markdown-content :deep(pre) {
  background: #182230;
  border-radius: 6px;
  color: #e5edf7;
  overflow: auto;
  padding: 16px;
}
.markdown-content :deep(code) {
  background: #eef1f5;
  border-radius: 3px;
  font-family: Consolas, monospace;
  font-size: 0.9em;
  padding: 2px 5px;
}
.markdown-content :deep(pre code) {
  background: transparent;
  padding: 0;
}
.markdown-content :deep(blockquote) {
  border-left: 4px solid #7aa8e8;
  color: #556070;
  margin: 1em 0;
  padding-left: 16px;
}
.markdown-content :deep(table) {
  border-collapse: collapse;
  width: 100%;
}
.markdown-content :deep(th),
.markdown-content :deep(td) {
  border: 1px solid #dce1e8;
  padding: 8px 10px;
  text-align: left;
}
.markdown-content :deep(th) {
  background: #f5f7fa;
}
.markdown-content :deep(img) {
  max-width: 100%;
}
</style>
