export type SessionTabKind = 'workspace' | 'file';

export interface SessionTab {
  id: string;
  kind: SessionTabKind;
  workspacePath: string;
  workspaceName: string;
  filePath: string | null;
  fileName: string | null;
  fileExtension: string | null;
  currentDirectory: string | null;
  position: number;
  active: boolean;
}
