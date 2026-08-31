<template>
  <section class="image-preview">
    <img ref="image" class="image-source" :src="url" :alt="alt" @load="initializeViewer" />
    <div class="image-toolbar" role="toolbar" aria-label="图片预览工具">
      <el-tooltip content="缩小" placement="top">
        <el-button :icon="ZoomOut" circle aria-label="缩小图片" @click="zoom(-0.1)" />
      </el-tooltip>
      <el-tooltip content="放大" placement="top">
        <el-button :icon="ZoomIn" circle aria-label="放大图片" @click="zoom(0.1)" />
      </el-tooltip>
      <el-tooltip content="原始尺寸" placement="top">
        <el-button :icon="Scan" circle aria-label="按原始尺寸查看" @click="oneToOne" />
      </el-tooltip>
      <el-tooltip content="适应窗口" placement="top">
        <el-button :icon="Undo2" circle aria-label="图片适应窗口" @click="reset" />
      </el-tooltip>
      <span class="toolbar-divider" aria-hidden="true" />
      <el-tooltip content="向左旋转" placement="top">
        <el-button :icon="RotateCcw" circle aria-label="向左旋转图片" @click="rotate(-90)" />
      </el-tooltip>
      <el-tooltip content="向右旋转" placement="top">
        <el-button :icon="RotateCw" circle aria-label="向右旋转图片" @click="rotate(90)" />
      </el-tooltip>
      <el-tooltip content="水平翻转" placement="top">
        <el-button
          :icon="FlipHorizontal2"
          circle
          aria-label="水平翻转图片"
          @click="flipHorizontal"
        />
      </el-tooltip>
      <el-tooltip content="垂直翻转" placement="top">
        <el-button :icon="FlipVertical2" circle aria-label="垂直翻转图片" @click="flipVertical" />
      </el-tooltip>
    </div>
  </section>
</template>

<script setup lang="ts">
import Viewer from 'viewerjs';
import 'viewerjs/dist/viewer.css';
import {
  FlipHorizontal2,
  FlipVertical2,
  RotateCcw,
  RotateCw,
  Scan,
  Undo2,
  ZoomIn,
  ZoomOut,
} from 'lucide-vue-next';
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';

const props = defineProps<{ url: string; alt?: string }>();
const image = ref<HTMLImageElement | null>(null);
let viewer: Viewer | null = null;
let horizontalScale = 1;
let verticalScale = 1;

const destroyViewer = () => {
  viewer?.destroy();
  viewer = null;
};
const initializeViewer = () => {
  destroyViewer();
  if (!image.value) return;
  horizontalScale = 1;
  verticalScale = 1;
  viewer = new Viewer(image.value, {
    backdrop: false,
    button: false,
    inline: true,
    keyboard: false,
    navbar: false,
    title: false,
    toolbar: false,
    transition: false,
    zIndexInline: 0,
  });
};
const zoom = (ratio: number) => viewer?.zoom(ratio, true);
const oneToOne = () => viewer?.zoomTo(1, true);
const reset = () => {
  horizontalScale = 1;
  verticalScale = 1;
  viewer?.reset();
};
const rotate = (degree: number) => viewer?.rotate(degree);
const flipHorizontal = () => {
  horizontalScale *= -1;
  viewer?.scaleX(horizontalScale);
};
const flipVertical = () => {
  verticalScale *= -1;
  viewer?.scaleY(verticalScale);
};

watch(
  () => props.url,
  () => destroyViewer(),
);
onMounted(() => {
  if (image.value?.complete) void nextTick(initializeViewer);
});
onBeforeUnmount(destroyViewer);
</script>

<style scoped lang="scss">
.image-preview {
  background: repeating-conic-gradient(#f2f4f7 0% 25%, #fff 0% 50%) 50% / 20px 20px;
  flex: 1;
  min-height: 0;
  min-width: 0;
  position: relative;
}
.image-source {
  height: 0;
  opacity: 0;
  pointer-events: none;
  position: absolute;
  width: 0;
}
.image-preview :deep(.viewer-container) {
  background: transparent;
}
.image-toolbar {
  align-items: center;
  background: rgb(255 255 255 / 92%);
  border: 1px solid #dce3eb;
  border-radius: 8px;
  bottom: 18px;
  box-shadow: 0 4px 14px rgb(28 39 56 / 14%);
  display: flex;
  gap: 4px;
  left: 50%;
  padding: 5px;
  position: absolute;
  transform: translateX(-50%);
  z-index: 1;
}
.image-toolbar .el-button {
  height: 28px;
  margin: 0;
  width: 28px;
}
.toolbar-divider {
  background: #dce3eb;
  height: 18px;
  margin: 0 2px;
  width: 1px;
}
</style>
