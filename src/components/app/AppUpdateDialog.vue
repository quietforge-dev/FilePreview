<template>
  <el-dialog
    :model-value="visible"
    title="发现新版本"
    width="480px"
    :close-on-click-modal="!installing"
    :close-on-press-escape="!installing"
    :show-close="!installing"
    @update:model-value="$emit('update:visible', $event)"
  >
    <div class="update-summary">
      <div class="update-version">v{{ currentVersion }} <span>-&gt;</span> v{{ version }}</div>
      <p v-if="notes" class="update-notes">{{ notes }}</p>
      <p v-else class="update-notes">已准备好安装最新版本。</p>
    </div>
    <div v-if="installing" class="update-progress">
      <div class="update-progress-label">{{ progressLabel }}</div>
      <el-progress
        :percentage="progressPercentage"
        :indeterminate="progressPercentage === undefined"
        :stroke-width="8"
        :show-text="progressPercentage !== undefined"
      />
    </div>
    <template #footer>
      <el-button :disabled="installing" @click="$emit('manual-download')">手动下载</el-button>
      <el-button :disabled="installing" @click="$emit('update:visible', false)">稍后</el-button>
      <el-button type="primary" :loading="installing" @click="$emit('install')">
        {{ installing ? '正在安装' : '立即更新并重启' }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    visible: boolean;
    currentVersion: string;
    version: string;
    notes?: string;
    installing?: boolean;
    progressPercentage?: number;
    progressLabel?: string;
  }>(),
  {
    notes: '',
    installing: false,
    progressPercentage: undefined,
    progressLabel: '正在下载更新包...',
  },
);

defineEmits<{
  'update:visible': [visible: boolean];
  install: [];
  'manual-download': [];
}>();
</script>

<style scoped lang="scss">
.update-summary {
  padding: 2px 0 8px;
}
.update-version {
  color: #25334c;
  font-size: 16px;
  font-weight: 600;
}
.update-version span {
  color: #8b97aa;
  margin: 0 6px;
}
.update-notes {
  color: #5d6b82;
  line-height: 1.65;
  margin: 12px 0 0;
  max-height: 150px;
  overflow-y: auto;
  white-space: pre-wrap;
}
.update-progress {
  background: #f5f8fc;
  border-radius: 6px;
  margin-top: 10px;
  padding: 12px;
}
.update-progress-label {
  color: #526178;
  font-size: 13px;
  margin-bottom: 8px;
}
</style>
