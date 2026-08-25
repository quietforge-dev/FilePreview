export interface RecentWorkspace {
  path: string;
  name: string;
  lastOpenedAt: number;
}

export interface RecentFile {
  path: string;
  name: string;
  extension: string;
  lastOpenedAt: number;
}
