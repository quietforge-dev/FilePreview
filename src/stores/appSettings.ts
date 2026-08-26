import { defineStore } from 'pinia';
import { getAppSetting, setAppSetting } from '../api/settings';

const FOLDER_PANE_WIDTH_KEY = 'folder_pane_width';
const MIN_FOLDER_PANE_WIDTH = 180;

const parseWidth = (value: string | null) => {
  if (!value) return null;
  const width = Number(value);
  return Number.isInteger(width) && width >= MIN_FOLDER_PANE_WIDTH ? width : null;
};

export const useAppSettingsStore = defineStore('appSettings', {
  state: () => ({
    folderPaneWidth: null as number | null,
  }),
  actions: {
    async restoreFolderPaneWidth() {
      try {
        this.folderPaneWidth = parseWidth(await getAppSetting(FOLDER_PANE_WIDTH_KEY));
      } catch {
        this.folderPaneWidth = null;
      }
      return this.folderPaneWidth;
    },
    async saveFolderPaneWidth(width: number) {
      const normalizedWidth = Math.round(width);
      if (this.folderPaneWidth === normalizedWidth) return;
      this.folderPaneWidth = normalizedWidth;
      try {
        await setAppSetting(FOLDER_PANE_WIDTH_KEY, String(normalizedWidth));
      } catch {
        // 界面偏好保存失败不应阻止当前文件浏览。
      }
    },
  },
});
