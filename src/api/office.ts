import { invoke } from '@tauri-apps/api/core';

export interface OfficeRuntimeStatus {
  installed: boolean;
  supportsQuickInstall: boolean;
}

export const getOfficeRuntimeStatus = () => invoke<OfficeRuntimeStatus>('office_runtime_status');

export const installLibreOffice = () => invoke<void>('install_libreoffice');

export const convertOfficeToPdf = async (path: string): Promise<Uint8Array> => {
  const bytes = await invoke<number[]>('convert_office_to_pdf', { path });
  return Uint8Array.from(bytes);
};

export const openLibreOfficeDownloadPage = () => invoke<void>('open_libreoffice_download_page');
