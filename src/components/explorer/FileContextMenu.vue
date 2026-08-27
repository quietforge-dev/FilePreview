<template>
  <div
    ref="menuElement"
    class="file-context-menu"
    :style="{ left: `${position.x}px`, top: `${position.y}px` }"
    role="menu"
    @contextmenu.prevent
  >
    <button type="button" role="menuitem" @click="emitAction('open')">
      <FolderOpen :size="16" />{{ file.isDirectory ? '打开文件夹' : '打开文件' }}
    </button>
    <button v-if="!file.isDirectory" type="button" role="menuitem" @click="emitAction('reveal')">
      <LocateFixed :size="16" />在文件夹中显示
    </button>
    <button type="button" role="menuitem" @click="emitAction('systemOpen')">
      <ExternalLink :size="16" />{{
        file.isDirectory ? '在系统文件管理器中打开' : '用系统默认程序打开'
      }}
    </button>
    <div class="menu-separator" />
    <button v-if="file.isDirectory" type="button" role="menuitem" @click="emitAction('createFile')">
      <FilePlus2 :size="16" />新建文件
    </button>
    <button type="button" role="menuitem" @click="emitAction('copyPath')">
      <ClipboardCopy :size="16" />复制路径
    </button>
    <button type="button" role="menuitem" @click="emitAction('copy')">
      <Copy :size="16" />复制
    </button>
    <button type="button" role="menuitem" :disabled="!canPaste" @click="emitAction('paste')">
      <ClipboardPaste :size="16" />粘贴到此处
    </button>
    <button type="button" role="menuitem" @click="emitAction('refresh')">
      <RefreshCw :size="16" />刷新
    </button>
    <div class="menu-separator" />
    <button type="button" role="menuitem" class="danger" @click="emitAction('delete')">
      <Trash2 :size="16" />删除
    </button>
  </div>
</template>

<script setup lang="ts">
import {
  ClipboardCopy,
  ClipboardPaste,
  Copy,
  ExternalLink,
  FilePlus2,
  FolderOpen,
  LocateFixed,
  RefreshCw,
  Trash2,
} from 'lucide-vue-next';
import { nextTick, onMounted, onUnmounted, ref } from 'vue';
import type { FileInfo } from '../../types/file';

const props = defineProps<{
  file: FileInfo;
  x: number;
  y: number;
  canPaste: boolean;
}>();
type ContextMenuAction =
  | 'open'
  | 'reveal'
  | 'systemOpen'
  | 'createFile'
  | 'copyPath'
  | 'copy'
  | 'paste'
  | 'refresh'
  | 'delete';

const emit = defineEmits<{
  (event: ContextMenuAction): void;
  (event: 'close'): void;
}>();
const menuElement = ref<HTMLElement>();
const position = ref({ x: props.x, y: props.y });

const updatePosition = () => {
  const menu = menuElement.value;
  if (!menu) return;
  position.value = {
    x: Math.max(8, Math.min(props.x, window.innerWidth - menu.offsetWidth - 8)),
    y: Math.max(8, Math.min(props.y, window.innerHeight - menu.offsetHeight - 8)),
  };
};
const closeOnOutsidePointer = (event: PointerEvent) => {
  if (menuElement.value?.contains(event.target as Node)) return;
  emit('close');
};
const emitAction = (action: ContextMenuAction) => {
  emit('close');
  emit(action);
};

onMounted(() => {
  void nextTick(updatePosition);
  document.addEventListener('pointerdown', closeOnOutsidePointer);
  window.addEventListener('resize', updatePosition);
});
onUnmounted(() => {
  document.removeEventListener('pointerdown', closeOnOutsidePointer);
  window.removeEventListener('resize', updatePosition);
});
</script>

<style scoped lang="scss">
.file-context-menu {
  background: #fff;
  border: 1px solid #d8dee8;
  box-shadow: 0 8px 20px rgb(15 23 42 / 16%);
  min-width: 208px;
  padding: 4px;
  position: fixed;
  z-index: 30;
}
.file-context-menu button {
  align-items: center;
  background: transparent;
  border: 0;
  color: #344054;
  cursor: pointer;
  display: flex;
  font: inherit;
  font-size: 13px;
  gap: 8px;
  min-height: 32px;
  padding: 0 9px;
  text-align: left;
  width: 100%;
}
.file-context-menu button:hover:not(:disabled) {
  background: #f1f5f9;
}
.file-context-menu button:disabled {
  color: #98a2b3;
  cursor: not-allowed;
}
.file-context-menu button.danger {
  color: #b42318;
}
.file-context-menu button.danger:hover {
  background: #fff1f0;
}
.menu-separator {
  border-top: 1px solid #edf0f4;
  margin: 4px 0;
}
</style>
