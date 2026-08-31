import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { invoke } from '@tauri-apps/api/core';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

export const revealInFileManager = (path: string) => revealItemInDir(path);

export const openWithDefaultApplication = (path: string) =>
  invoke<void>('open_entry_with_default_application', { path });

export const copyTextToClipboard = (text: string) => writeText(text);

export const copyPathToClipboard = (path: string) => copyTextToClipboard(path);
