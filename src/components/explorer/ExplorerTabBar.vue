<template>
  <nav v-if="tabs.tabs.length" class="tab-bar" aria-label="打开的标签">
    <button
      v-for="tab in tabs.tabs"
      :key="tab.id"
      class="workspace-tab"
      :class="{ active: tab.id === tabs.activeId }"
      type="button"
      @click="activateTab(tab.id)"
      @contextmenu.prevent="openContextMenu(tab, $event)"
    >
      <el-icon><component :is="tab.kind === 'workspace' ? FolderOpened : Document" /></el-icon>
      <span>{{ tab.kind === 'workspace' ? tab.workspaceName : tab.fileName }}</span>
      <span
        class="tab-close"
        role="button"
        :aria-label="`关闭 ${tab.kind === 'workspace' ? tab.workspaceName : tab.fileName}`"
        @click.stop="closeTab(tab.id)"
        ><el-icon><Close /></el-icon
      ></span>
    </button>
    <el-tooltip content="打开其他文件夹" placement="bottom">
      <el-button :icon="Plus" circle aria-label="打开其他文件夹" @click="emit('chooseWorkspace')" />
    </el-tooltip>
  </nav>
  <div
    v-if="contextMenu"
    ref="contextMenuElement"
    class="tab-context-menu"
    :style="{ left: `${contextMenu.x}px`, top: `${contextMenu.y}px` }"
    @contextmenu.prevent
  >
    <button :disabled="contextMenu.index === 0" type="button" @click="closeTabs('left')">
      关闭左侧
    </button>
    <button
      :disabled="contextMenu.index === tabs.tabs.length - 1"
      type="button"
      @click="closeTabs('right')"
    >
      关闭右侧
    </button>
    <button type="button" @click="closeTabs('others')">关闭其他</button>
    <button type="button" @click="closeTabs('all')">关闭全部</button>
  </div>
</template>

<script setup lang="ts">
import { Close, Document, FolderOpened, Plus } from '@element-plus/icons-vue';
import { onMounted, onUnmounted, ref } from 'vue';
import { useTabsStore } from '../../stores/tabs';
import type { SessionTab } from '../../types/session';

type ContextMenu = { id: string; index: number; x: number; y: number };
type CloseScope = 'left' | 'right' | 'others' | 'all';

const emit = defineEmits<{
  chooseWorkspace: [];
  activate: [id: string];
  close: [id: string];
  closeTabs: [scope: CloseScope, id: string];
}>();
const tabs = useTabsStore();
const contextMenu = ref<ContextMenu | null>(null);
const contextMenuElement = ref<HTMLElement>();

const activateTab = (id: string) => emit('activate', id);
const closeTab = (id: string) => emit('close', id);
const openContextMenu = (tab: SessionTab, event: MouseEvent) => {
  contextMenu.value = {
    id: tab.id,
    index: tabs.tabs.findIndex((candidate) => candidate.id === tab.id),
    x: Math.min(event.clientX, window.innerWidth - 144),
    y: Math.min(event.clientY, window.innerHeight - 148),
  };
};
const closeTabs = (scope: CloseScope) => {
  const target = contextMenu.value;
  contextMenu.value = null;
  if (!target) return;
  emit('closeTabs', scope, target.id);
};
const closeOnOutsidePointer = (event: PointerEvent) => {
  if (contextMenuElement.value?.contains(event.target as Node)) return;
  contextMenu.value = null;
};

onMounted(() => document.addEventListener('pointerdown', closeOnOutsidePointer));
onUnmounted(() => document.removeEventListener('pointerdown', closeOnOutsidePointer));
</script>

<style scoped lang="scss">
.tab-bar {
  align-items: end;
  background: #f3f5f8;
  border-bottom: 1px solid #dce2ea;
  display: flex;
  gap: 1px;
  min-height: 36px;
  overflow-x: auto;
  padding: 4px 8px 0;
}
.workspace-tab {
  align-items: center;
  background: #e9edf2;
  border: 1px solid transparent;
  border-bottom: 0;
  color: #667085;
  cursor: pointer;
  display: flex;
  flex: 0 0 auto;
  font: inherit;
  font-size: 12px;
  gap: 7px;
  height: 32px;
  max-width: 220px;
  min-width: 120px;
  padding: 0 7px 0 10px;
}
.workspace-tab:hover {
  background: #f7f9fb;
  color: #344054;
}
.workspace-tab.active {
  background: #fff;
  border-color: #dce2ea;
  color: #1d4ed8;
  font-weight: 600;
}
.workspace-tab > span:first-of-type {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.tab-close {
  align-items: center;
  border-radius: 3px;
  display: inline-flex;
  flex: 0 0 auto;
  height: 20px;
  justify-content: center;
  margin-left: auto;
  width: 20px;
}
.tab-close:hover {
  background: #e4e9f0;
  color: #344054;
}
.tab-bar > .el-button {
  align-self: center;
  flex: 0 0 auto;
  margin: 0 2px 3px 8px;
}
.tab-context-menu {
  background: #fff;
  border: 1px solid #d8dee8;
  box-shadow: 0 8px 20px rgb(15 23 42 / 16%);
  min-width: 136px;
  padding: 4px;
  position: fixed;
  z-index: 21;
}
.tab-context-menu button {
  background: transparent;
  border: 0;
  color: #344054;
  cursor: pointer;
  display: block;
  font: inherit;
  font-size: 13px;
  min-height: 30px;
  padding: 0 9px;
  text-align: left;
  width: 100%;
}
.tab-context-menu button:hover:not(:disabled) {
  background: #f1f5f9;
}
.tab-context-menu button:disabled {
  color: #98a2b3;
  cursor: not-allowed;
}
</style>
