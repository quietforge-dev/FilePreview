import { invoke } from '@tauri-apps/api/core';
import type { FileInfo, WorkspaceInfo } from '../types/file';

export const openWorkspace = (path: string) => invoke<WorkspaceInfo>('open_workspace', { path });

export const listDirectory = (path?: string) => invoke<FileInfo[]>('list_directory', { path });

export const readFile = async (path: string): Promise<Uint8Array> => {
  const bytes = await invoke<number[]>('read_file', { path });
  return Uint8Array.from(bytes);
};

export const getFileInfo = (path: string) => invoke<FileInfo>('file_info', { path });

export const writeMarkdownFile = (path: string, content: string) =>
  invoke<FileInfo>('write_markdown_file', { path, content });

export const createFile = (destinationDirectory: string, fileName: string) =>
  invoke<FileInfo>('create_file', { destinationDirectory, fileName });

export interface ContentSearchResult {
  path: string;
  name: string;
  extension: string;
  lineNumber: number;
  lineContent: string;
}

export const searchFileContents = (query: string) =>
  invoke<ContentSearchResult[]>('search_file_contents', { query });

export const searchWorkspaceEntries = (query: string) =>
  invoke<FileInfo[]>('search_workspace_entries', { query });

export const copyEntry = (source: string, destinationDirectory: string) =>
  invoke<FileInfo>('copy_entry', { source, destinationDirectory });

export const hasSystemClipboardFiles = () => invoke<boolean>('has_system_clipboard_files');

export const copyEntryToSystemClipboard = (path: string) =>
  invoke<void>('copy_entry_to_system_clipboard', { path });

export const pasteSystemClipboardEntries = (destinationDirectory: string) =>
  invoke<FileInfo[]>('paste_system_clipboard_entries', { destinationDirectory });

export const deleteEntry = (path: string) => invoke<void>('delete_entry', { path });
