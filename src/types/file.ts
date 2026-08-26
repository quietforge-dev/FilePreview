export interface FileInfo {
  path: string;
  name: string;
  extension: string;
  size: number;
  modifiedAt: number | null;
  isDirectory: boolean;
}

export interface WorkspaceInfo {
  path: string;
  name: string;
}

export type PreviewKind =
  'markdown' | 'text' | 'pdf' | 'office-unavailable' | 'image' | 'unsupported';

export type PreviewContent =
  | { kind: 'markdown'; source: string; html: string }
  | { kind: 'text'; content: string; language: string }
  | { kind: 'pdf'; data: Uint8Array }
  | { kind: 'office-unavailable'; message: string }
  | { kind: 'image'; url: string }
  | { kind: 'unsupported'; message: string };
