<template>
  <aside class="markdown-outline" :class="{ collapsed }">
    <div v-if="!collapsed" class="outline-header">文档大纲</div>
    <nav v-if="!collapsed" aria-label="文档大纲">
      <button
        v-for="heading in headings"
        :key="heading.id"
        class="outline-item"
        :class="{ active: heading.id === activeId }"
        :style="{ paddingLeft: `${12 + (heading.depth - minimumDepth) * 14}px` }"
        :title="heading.text"
        type="button"
        @click="emit('navigate', heading.id)"
      >
        {{ heading.text }}
      </button>
    </nav>
    <el-tooltip :content="collapsed ? '展开文档大纲' : '收起文档大纲'" placement="right">
      <button
        class="outline-toggle"
        type="button"
        :aria-label="collapsed ? '展开文档大纲' : '收起文档大纲'"
        @click="collapsed = !collapsed"
      >
        <component :is="collapsed ? PanelLeftOpen : PanelLeftClose" :size="16" />
      </button>
    </el-tooltip>
  </aside>
</template>

<script setup lang="ts">
import { PanelLeftClose, PanelLeftOpen } from 'lucide-vue-next';
import { computed, ref } from 'vue';
import type { MarkdownHeading } from '../../types/file';

const props = defineProps<{ headings: MarkdownHeading[]; activeId: string | null }>();
const emit = defineEmits<{ navigate: [id: string] }>();
const collapsed = ref(false);
const minimumDepth = computed(() => Math.min(...props.headings.map((heading) => heading.depth)));
</script>

<style scoped lang="scss">
.markdown-outline {
  align-self: stretch;
  background: #f8fafc;
  border-right: 1px solid #e1e6ed;
  flex: 0 0 220px;
  min-height: 0;
  overflow-y: auto;
  padding: 14px 8px;
  transition: flex-basis 0.18s ease;
}
.markdown-outline.collapsed {
  flex-basis: 38px;
  overflow: hidden;
  padding: 8px 5px;
}
.outline-header {
  color: #526071;
  font-size: 12px;
  font-weight: 600;
  padding: 0 8px 9px;
}
.markdown-outline nav {
  display: grid;
  gap: 2px;
}
.outline-item {
  background: transparent;
  border: 0;
  border-radius: 4px;
  color: #5b6878;
  cursor: pointer;
  font: inherit;
  font-size: 12px;
  line-height: 1.35;
  overflow: hidden;
  padding-bottom: 6px;
  padding-right: 8px;
  padding-top: 6px;
  text-align: left;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 100%;
}
.outline-item:hover {
  background: #eaf2ff;
  color: #1d4ed8;
}
.outline-item.active {
  background: #dbeafe;
  color: #1d4ed8;
  font-weight: 600;
}
.outline-toggle {
  align-items: center;
  background: transparent;
  border: 0;
  border-radius: 4px;
  color: #64748b;
  cursor: pointer;
  display: flex;
  height: 28px;
  justify-content: center;
  margin: 10px 4px 0;
  padding: 0;
  width: 28px;
}
.outline-toggle:hover {
  background: #eaf2ff;
  color: #1d4ed8;
}
</style>
