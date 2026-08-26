import { invoke } from '@tauri-apps/api/core';
import type { SessionTab } from '../types/session';

export const listSessionTabs = () => invoke<SessionTab[]>('list_session_tabs');

export const saveSessionTabs = (tabs: SessionTab[]) => invoke<void>('save_session_tabs', { tabs });
