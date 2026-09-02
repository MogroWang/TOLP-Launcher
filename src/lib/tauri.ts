import { invoke } from '@tauri-apps/api/core';

export type LaunchMode = 'fullscreen' | 'windowed';

export interface Settings {
  launchMode: LaunchMode;
  gameDir: string | null;
}

export interface GameStatus {
  found: boolean;
  dir: string | null;
  reason: string | null;
}

export interface LaunchResult {
  url: string;
  fullscreen: boolean;
}

/** 非 Tauri 环境（纯浏览器开发预览）下安全降级，避免整页报错 */
const inTauri = '__TAURI_INTERNALS__' in window;

export function getSettings(): Promise<Settings> {
  if (!inTauri) return Promise.resolve({ launchMode: 'windowed', gameDir: null });
  return invoke<Settings>('get_settings');
}

export function saveSettings(settings: Settings): Promise<Settings> {
  if (!inTauri) return Promise.resolve(settings);
  return invoke<Settings>('save_settings', { settings });
}

export function getGameStatus(): Promise<GameStatus> {
  if (!inTauri) {
    return Promise.resolve({ found: false, dir: null, reason: '请在桌面应用中运行' });
  }
  return invoke<GameStatus>('game_status');
}

export function launchGame(): Promise<LaunchResult> {
  return invoke<LaunchResult>('launch_game');
}
