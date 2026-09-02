import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export type LaunchMode = 'fullscreen' | 'windowed';

export interface Settings {
  launchMode: LaunchMode;
  gameDir: string | null;
  /** 选中的游戏版本；null 表示自定义目录启动 */
  versionId: string | null;
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

/** 内置游戏版本列表（占位）：版本管理接入在线分发前仅提供 4.0.002 DEV */
export const BUILTIN_GAME_VERSIONS = [{ id: '4.0.002', label: '光点之旅 4.0.002 DEV' }] as const;

/** 非 Tauri 环境（纯浏览器开发预览）下安全降级，避免整页报错 */
const inTauri = '__TAURI_INTERNALS__' in window;

export function getSettings(): Promise<Settings> {
  if (!inTauri) {
    return Promise.resolve({ launchMode: 'windowed', gameDir: null, versionId: '4.0.002' });
  }
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

export function isGameRunning(): Promise<boolean> {
  if (!inTauri) return Promise.resolve(false);
  return invoke<boolean>('game_running');
}

export function launchGame(): Promise<LaunchResult> {
  return invoke<LaunchResult>('launch_game');
}

/** 订阅游戏窗口关闭事件，返回取消订阅函数 */
export function onGameClosed(handler: () => void): Promise<UnlistenFn> {
  if (!inTauri) return Promise.resolve(() => {});
  return listen('game-closed', handler);
}
