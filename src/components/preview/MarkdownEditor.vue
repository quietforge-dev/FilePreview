<template><div ref="container" class="markdown-editor" /></template>

<script setup lang="ts">
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
import { markdown } from '@codemirror/lang-markdown';
import { bracketMatching, defaultHighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { searchKeymap } from '@codemirror/search';
import { EditorState } from '@codemirror/state';
import {
  drawSelection,
  EditorView,
  highlightActiveLine,
  highlightActiveLineGutter,
  keymap,
  lineNumbers,
  placeholder,
} from '@codemirror/view';
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';

const props = defineProps<{ modelValue: string }>();
const emit = defineEmits<{
  'update:modelValue': [value: string];
  save: [];
}>();
const container = ref<HTMLElement>();
let editor: EditorView | null = null;
let syncingModelValue = false;

const syncFromModelValue = (value: string) => {
  if (!editor || editor.state.doc.toString() === value) return;
  syncingModelValue = true;
  editor.dispatch({ changes: { from: 0, to: editor.state.doc.length, insert: value } });
  syncingModelValue = false;
};

onMounted(() => {
  if (!container.value) return;
  const state = EditorState.create({
    doc: props.modelValue,
    extensions: [
      lineNumbers(),
      highlightActiveLineGutter(),
      drawSelection(),
      history(),
      bracketMatching(),
      markdown(),
      syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
      highlightActiveLine(),
      EditorView.lineWrapping,
      placeholder('开始编辑 Markdown'),
      keymap.of([
        indentWithTab,
        ...defaultKeymap,
        ...historyKeymap,
        ...searchKeymap,
        {
          key: 'Mod-s',
          run: () => {
            emit('save');
            return true;
          },
        },
      ]),
      EditorView.updateListener.of((update) => {
        if (update.docChanged && !syncingModelValue)
          emit('update:modelValue', update.state.doc.toString());
      }),
      EditorView.theme({
        '&': { height: '100%', fontSize: '14px' },
        '.cm-scroller': { fontFamily: 'Consolas, "Courier New", monospace', overflow: 'auto' },
        '.cm-content': { padding: '16px 0 48px' },
        '.cm-gutters': { backgroundColor: '#f8fafc', borderRight: '1px solid #e6eaf0' },
      }),
    ],
  });
  editor = new EditorView({ state, parent: container.value });
});

watch(() => props.modelValue, syncFromModelValue);
onBeforeUnmount(() => editor?.destroy());
</script>

<style scoped lang="scss">
.markdown-editor {
  height: 100%;
  min-height: 0;
}
</style>
