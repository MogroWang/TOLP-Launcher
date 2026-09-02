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

export function getSettings(): Promise<Settings> {
  return invoke<Settings>('get_settings');
}

export function saveSettings(settings: Settings): Promise<Settings> {
  return invoke<Settings>('save_settings', { settings });
}

export function getGameStatus(): Promise<GameStatus> {
  return invoke<GameStatus>('game_status');
}

export function launchGame(): Promise<LaunchResult> {
  return invoke<LaunchResult>('launch_game');
}
