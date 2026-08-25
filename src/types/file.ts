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

export type PreviewKind = 'markdown' | 'text' | 'office' | 'image' | 'unsupported';

export type PreviewContent =
  | { kind: 'markdown'; html: string }
  | { kind: 'text'; content: string; language: string }
  | { kind: 'office'; officeType: 'word' | 'powerpoint'; html?: string; slides?: string[][] }
  | { kind: 'image'; url: string }
  | { kind: 'unsupported'; message: string };
