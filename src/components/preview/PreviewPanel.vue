<template>
  <section class="preview-panel">
    <header class="preview-header">
      <span>{{ file?.name ?? '预览' }}{{ markdownSession?.dirty ? ' *' : '' }}</span>
      <div class="preview-actions">
        <span v-if="file" class="extension">{{ file.extension || '文件' }}</span>
        <template v-if="isEditableMarkdown && markdownSession">
          <el-tooltip content="预览" placement="bottom">
            <el-button
              :icon="Eye"
              :disabled="markdownSession.mode === 'preview'"
              circle
              aria-label="预览 Markdown"
              @click="setMarkdownMode('preview')"
            />
          </el-tooltip>
          <el-tooltip content="编辑" placement="bottom">
            <el-button
              :icon="Pencil"
              :disabled="markdownSession.mode === 'edit'"
              circle
              aria-label="编辑 Markdown"
              @click="setMarkdownMode('edit')"
            />
          </el-tooltip>
          <el-tooltip content="保存" placement="bottom">
            <el-button
              :icon="Save"
              :disabled="!markdownSession.dirty"
              :loading="markdownSession.saving"
              circle
              aria-label="保存 Markdown"
              @click="saveMarkdown"
            />
          </el-tooltip>
        </template>
      </div>
    </header>
    <div v-loading="loading" class="preview-body">
      <el-empty v-if="!file && !loading" description="选择一个文件开始预览" :image-size="86" />
      <el-result v-else-if="error" icon="error" title="无法预览" :sub-title="error" />
      <template v-else-if="content?.kind === 'markdown' && markdownSession">
        <div v-if="markdownSession.externalChanged" class="markdown-conflict">
          <span>文件已在外部修改</span>
          <el-button link type="primary" @click="emit('reloadMarkdown')">重新加载</el-button>
          <el-button link @click="markdownEditor.clearExternalChanged(file!.path)"
            >保留本地草稿</el-button
          >
        </div>
        <MarkdownEditor
          v-if="markdownSession.mode === 'edit'"
          :model-value="markdownSession.source"
          @update:model-value="markdownEditor.updateSource(file!.path, $event)"
          @save="saveMarkdown"
        />
        <MarkdownPreview v-else :html="content.html" />
      </template>
      <TextPreview
        v-else-if="content?.kind === 'text'"
        :content="content.content"
        :language="content.language"
      />
      <PdfPreview v-else-if="content?.kind === 'pdf'" :data="content.data" />
      <el-result
        v-else-if="content?.kind === 'office-unavailable'"
        icon="warning"
        title="需要 LibreOffice"
        :sub-title="content.message"
      >
        <template #extra>
          <div class="office-actions">
            <el-button
              v-if="officeRuntime.supportsQuickInstall"
              type="primary"
              :loading="officeRuntime.installing"
              @click="installLibreOffice"
            >
              安装 LibreOffice
            </el-button>
            <el-button @click="openDownloadPage">官方下载</el-button>
            <el-button :loading="officeRuntime.checking" @click="retryOfficePreview">
              重新检测
            </el-button>
          </div>
          <p v-if="officeRuntime.error" class="office-error">{{ officeRuntime.error }}</p>
        </template>
      </el-result>
      <div v-else-if="content?.kind === 'image'" class="image-preview">
        <img :src="content.url" :alt="file?.name" />
      </div>
      <el-result
        v-else-if="content?.kind === 'unsupported'"
        icon="warning"
        title="暂不支持预览"
        :sub-title="content.message"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import type { FileInfo, PreviewContent } from '../../types/file';
import MarkdownPreview from './MarkdownPreview.vue';
import MarkdownEditor from './MarkdownEditor.vue';
import PdfPreview from './PdfPreview.vue';
import TextPreview from './TextPreview.vue';
import { useOfficeRuntimeStore } from '../../stores/officeRuntime';
import { useMarkdownEditorStore } from '../../stores/markdownEditor';
import { usePreviewStore } from '../../stores/preview';
import { Eye, Pencil, Save } from 'lucide-vue-next';
import { ElMessage, ElMessageBox } from 'element-plus';
import { computed, watch } from 'vue';

const props = defineProps<{
  file: FileInfo | null;
  content: PreviewContent | null;
  loading: boolean;
  error: string;
}>();
const emit = defineEmits<{ retry: []; reloadMarkdown: [] }>();
const officeRuntime = useOfficeRuntimeStore();
const markdownEditor = useMarkdownEditorStore();
const preview = usePreviewStore();
const markdownSession = computed(() =>
  props.file ? (markdownEditor.sessions[props.file.path] ?? null) : null,
);
const isEditableMarkdown = computed(
  () => props.file && ['md', 'markdown'].includes(props.file.extension),
);

watch(
  () => props.content?.kind,
  (kind) => {
    if (kind === 'office-unavailable') void officeRuntime.check();
  },
  { immediate: true },
);

watch(
  () =>
    props.content?.kind === 'markdown' && props.file
      ? { file: props.file, source: props.content.source }
      : null,
  (value) => {
    if (value) markdownEditor.ensureSession(value.file, value.source);
  },
  { immediate: true },
);

const setMarkdownMode = async (mode: 'preview' | 'edit') => {
  if (!props.file || !markdownSession.value) return;
  if (mode === 'preview') await preview.renderMarkdownSource(markdownSession.value.source);
  markdownEditor.setMode(props.file.path, mode);
};
const saveMarkdown = async () => {
  if (!props.file || !markdownSession.value) return;
  if (markdownSession.value.externalChanged) {
    try {
      await ElMessageBox.confirm('文件已在外部修改，保存将覆盖外部内容。', '确认覆盖保存', {
        confirmButtonText: '覆盖保存',
        cancelButtonText: '取消',
        type: 'warning',
      });
    } catch {
      return;
    }
  }
  try {
    const saved = await markdownEditor.save(props.file.path);
    if (!saved) return;
    preview.updateFileMetadata(saved);
    ElMessage.success('已保存');
  } catch (error) {
    ElMessage.error(`保存失败：${error instanceof Error ? error.message : String(error)}`);
  }
};

const retryOfficePreview = async () => {
  if (await officeRuntime.check()) emit('retry');
};
const installLibreOffice = async () => {
  try {
    await officeRuntime.install();
    if (officeRuntime.installed) emit('retry');
  } catch {
    // 安装错误通过当前预览区呈现，用户可选择官网安装。
  }
};
const openDownloadPage = () => void officeRuntime.openDownloadPage();
</script>

<style scoped lang="scss">
.preview-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-width: 0;
}
.preview-header {
  align-items: center;
  border-bottom: 1px solid #e1e6ed;
  color: #3e4957;
  display: flex;
  font-size: 13px;
  font-weight: 600;
  height: 42px;
  justify-content: space-between;
  padding: 0 16px;
}
.preview-actions {
  align-items: center;
  display: flex;
  gap: 6px;
}
.preview-actions .el-button {
  height: 28px;
  width: 28px;
}
.extension {
  color: #8b95a4;
  font-size: 11px;
  font-weight: 500;
  text-transform: uppercase;
}
.preview-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
}
.markdown-conflict {
  align-items: center;
  background: #fff8e6;
  border-bottom: 1px solid #f3d38a;
  color: #8a5a00;
  display: flex;
  font-size: 13px;
  gap: 8px;
  padding: 7px 16px;
}
.image-preview {
  align-items: flex-start;
  background: repeating-conic-gradient(#f2f4f7 0% 25%, #fff 0% 50%) 50% / 20px 20px;
  display: flex;
  justify-content: center;
  min-height: 100%;
  padding: 32px;
}
.image-preview img {
  height: auto;
  max-width: 100%;
  width: 100%;
}
.office-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: center;
}
.office-error {
  color: #b42318;
  font-size: 13px;
  margin: 12px 0 0;
}
</style>
