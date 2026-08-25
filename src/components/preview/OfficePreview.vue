<template>
  <div v-if="officeType === 'word'" class="word-preview"><article v-html="html" /></div>
  <div v-else class="ppt-preview">
    <div class="slide">
      <p v-for="(line, index) in currentSlide" :key="`${slideIndex}-${index}`">{{ line }}</p>
      <p v-if="!currentSlide.length" class="empty-slide">此页没有可提取的文本</p>
    </div>
    <div class="ppt-controls">
      <el-button
        :icon="ArrowLeft"
        circle
        :disabled="slideIndex === 0"
        aria-label="上一页"
        @click="slideIndex -= 1"
      />
      <span>{{ slides.length ? `${slideIndex + 1} / ${slides.length}` : '0 / 0' }}</span>
      <el-button
        :icon="ArrowRight"
        circle
        :disabled="slideIndex >= slides.length - 1"
        aria-label="下一页"
        @click="slideIndex += 1"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ArrowLeft, ArrowRight } from '@element-plus/icons-vue';
import { computed, ref, watch } from 'vue';

const props = withDefaults(
  defineProps<{ officeType: 'word' | 'powerpoint'; html?: string; slides?: string[][] }>(),
  { html: '', slides: () => [] },
);
const slideIndex = ref(0);
const currentSlide = computed(() => props.slides[slideIndex.value] ?? []);
watch(
  () => props.slides,
  () => {
    slideIndex.value = 0;
  },
);
</script>

<style scoped lang="scss">
.word-preview {
  background: #edf0f4;
  min-height: 100%;
  overflow: auto;
  padding: 28px;
}
.word-preview article {
  background: #fff;
  box-shadow: 0 2px 12px rgb(15 23 42 / 10%);
  color: #253041;
  line-height: 1.75;
  margin: 0 auto;
  max-width: 850px;
  min-height: 100%;
  padding: 52px 64px;
}
.word-preview :deep(img) {
  max-width: 100%;
}
.ppt-preview {
  align-items: center;
  background: #eef1f5;
  display: flex;
  flex-direction: column;
  height: 100%;
  justify-content: center;
  padding: 24px;
}
.slide {
  aspect-ratio: 16 / 9;
  background: #fff;
  box-shadow: 0 5px 24px rgb(15 23 42 / 14%);
  color: #1f2937;
  display: flex;
  flex-direction: column;
  font-size: clamp(16px, 1.6vw, 28px);
  gap: 14px;
  justify-content: center;
  max-height: calc(100% - 70px);
  max-width: 100%;
  overflow: auto;
  padding: 8% 10%;
  width: min(880px, 100%);
}
.slide p {
  margin: 0;
}
.empty-slide {
  color: #8a94a3;
  font-size: 15px;
  text-align: center;
}
.ppt-controls {
  align-items: center;
  display: flex;
  gap: 16px;
  margin-top: 20px;
}
</style>
