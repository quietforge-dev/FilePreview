import { invoke } from '@tauri-apps/api/core';
import type { RecentFile, RecentWorkspace } from '../types/history';

export const recordBrowsedFile = (path: string) => invoke<void>('record_browsed_file', { path });

export const listRecentWorkspaces = () => invoke<RecentWorkspace[]>('list_recent_workspaces');

export const listRecentFiles = () => invoke<RecentFile[]>('list_recent_files');

export const clearRecentWorkspaces = () => invoke<void>('clear_recent_workspaces');

export const clearRecentFiles = () => invoke<void>('clear_recent_files');
