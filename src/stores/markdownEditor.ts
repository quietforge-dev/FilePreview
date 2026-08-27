import { defineStore } from 'pinia';
import { writeMarkdownFile } from '../api/file';
import type { FileInfo } from '../types/file';

export type MarkdownEditorMode = 'preview' | 'edit';

export interface MarkdownEditorSession {
  file: FileInfo;
  source: string;
  savedSource: string;
  mode: MarkdownEditorMode;
  dirty: boolean;
  saving: boolean;
  externalChanged: boolean;
}

export const useMarkdownEditorStore = defineStore('markdownEditor', {
  state: () => ({
    sessions: {} as Record<string, MarkdownEditorSession>,
  }),
  actions: {
    ensureSession(file: FileInfo, source: string) {
      const session = this.sessions[file.path];
      if (!session) {
        this.sessions[file.path] = {
          file,
          source,
          savedSource: source,
          mode: 'preview',
          dirty: false,
          saving: false,
          externalChanged: false,
        };
        return;
      }
      session.file = file;
      if (!session.dirty && !session.saving && session.source !== source) {
        session.source = source;
        session.savedSource = source;
        session.externalChanged = false;
      }
    },
    updateSource(path: string, source: string) {
      const session = this.sessions[path];
      if (!session) return;
      session.source = source;
      session.dirty = source !== session.savedSource;
    },
    setMode(path: string, mode: MarkdownEditorMode) {
      const session = this.sessions[path];
      if (session) session.mode = mode;
    },
    markExternalChanged(path: string) {
      const session = this.sessions[path];
      if (session?.dirty) session.externalChanged = true;
    },
    clearExternalChanged(path: string) {
      const session = this.sessions[path];
      if (session) session.externalChanged = false;
    },
    discardChanges(path: string) {
      const session = this.sessions[path];
      if (!session) return;
      session.source = session.savedSource;
      session.dirty = false;
      session.externalChanged = false;
    },
    remove(path: string) {
      delete this.sessions[path];
    },
    movePath(oldPath: string, newFile: FileInfo) {
      const normalized = oldPath.toLowerCase();
      Object.entries(this.sessions).forEach(([path, session]) => {
        const current = path.toLowerCase();
        if (
          current !== normalized &&
          !current.startsWith(`${normalized}\\`) &&
          !current.startsWith(`${normalized}/`)
        )
          return;
        delete this.sessions[path];
        const nextPath = `${newFile.path}${path.slice(oldPath.length)}`;
        session.file = {
          ...session.file,
          path: nextPath,
          name: current === normalized ? newFile.name : session.file.name,
          extension: current === normalized ? newFile.extension : session.file.extension,
        };
        this.sessions[nextPath] = session;
      });
    },
    clear() {
      this.sessions = {};
    },
    async save(path: string) {
      const session = this.sessions[path];
      if (!session || session.saving) return null;
      const source = session.source;
      session.saving = true;
      try {
        const file = await writeMarkdownFile(path, source);
        session.file = file;
        session.savedSource = source;
        session.dirty = session.source !== source;
        if (!session.dirty) session.externalChanged = false;
        return file;
      } finally {
        session.saving = false;
      }
    },
  },
});
