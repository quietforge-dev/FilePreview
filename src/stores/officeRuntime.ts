import { defineStore } from 'pinia';
import {
  getOfficeRuntimeStatus,
  installLibreOffice,
  openLibreOfficeDownloadPage,
} from '../api/office';

export const useOfficeRuntimeStore = defineStore('officeRuntime', {
  state: () => ({
    installed: false,
    supportsQuickInstall: false,
    checking: false,
    installing: false,
    error: '',
  }),
  actions: {
    async check() {
      if (this.checking) return this.installed;
      this.checking = true;
      this.error = '';
      try {
        const status = await getOfficeRuntimeStatus();
        this.installed = status.installed;
        this.supportsQuickInstall = status.supportsQuickInstall;
        return this.installed;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        return false;
      } finally {
        this.checking = false;
      }
    },
    async install() {
      if (this.installing) return;
      this.installing = true;
      this.error = '';
      try {
        await installLibreOffice();
        await this.check();
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        throw error;
      } finally {
        this.installing = false;
      }
    },
    async openDownloadPage() {
      try {
        await openLibreOfficeDownloadPage();
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
      }
    },
  },
});
