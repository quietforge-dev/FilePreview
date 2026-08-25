import { invoke } from '@tauri-apps/api/core';

export const getAppVersion = () => invoke<string>('app_version');

export const openProjectUrl = (url: string) => invoke<void>('open_external_url', { url });
