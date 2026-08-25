import { invoke } from '@tauri-apps/api/core';
import type { FileInfo, WorkspaceInfo } from '../types/file';

export const openWorkspace = (path: string) => invoke<WorkspaceInfo>('open_workspace', { path });

export const listDirectory = (path?: string) => invoke<FileInfo[]>('list_directory', { path });

export const readFile = async (path: string): Promise<Uint8Array> => {
  const bytes = await invoke<number[]>('read_file', { path });
  return Uint8Array.from(bytes);
};

export const copyEntry = (source: string, destinationDirectory: string) =>
  invoke<FileInfo>('copy_entry', { source, destinationDirectory });
