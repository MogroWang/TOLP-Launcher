import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export type LaunchMode = 'fullscreen' | 'windowed';

/** 窗口化启动时的游戏窗口大小（像素） */
export interface WindowedSize {
  width: number;
  height: number;
}

export interface Settings {
  launchMode: LaunchMode;
  /** 窗口化启动时的游戏窗口大小；null 表示默认 1280×720 */
  windowedSize: WindowedSize | null;
  gameDir: string | null;
  /** 选中的游戏版本；null 表示自定义目录启动 */
  versionId: string | null;
  /** 内置版本的备用位置：数据文件夹中未识别到该版本时使用 */
  customVersionDir: string | null;
}

export interface GameStatus {
  found: boolean;
  dir: string | null;
  reason: string | null;
  /** 游戏目录 manifest.webmanifest 识别到的版本号 */
  version: string | null;
  /** manifest id 是否为官方 com.mws.tolp */
  official: boolean;
}

export interface LaunchResult {
  url: string;
  fullscreen: boolean;
}

/** 内置游戏版本（占位）：版本管理接入在线分发前仅提供 4.0.002 DEV */
export const BUILTIN_GAME_VERSIONS = [{ id: '4.0.002', label: '光点之旅 4.0.002 DEV' }] as const;

/** 版本条目：数据文件夹 versions/ 下识别到的版本，或用户指定的自定义位置 */
export interface VersionEntry {
  id: string;
  dir: string;
  /** 目录中存在 index.html（可直接启动） */
  found: boolean;
  reason: string | null;
  /** manifest.webmanifest 的 name */
  name: string | null;
  /** manifest.webmanifest 的 version */
  version: string | null;
  /** manifest id 为官方 com.mws.tolp */
  official: boolean;
}

/** 版本扫描结果：数据文件夹 + 自动识别的版本 + 自定义位置 */
export interface VersionScan {
  dataDir: string;
  versionsDir: string;
  versions: VersionEntry[];
  custom: VersionEntry | null;
}

/** 非 Tauri 环境（纯浏览器开发预览）下安全降级，避免整页报错 */
const inTauri = '__TAURI_INTERNALS__' in window;

export function getSettings(): Promise<Settings> {
  if (!inTauri) {
    return Promise.resolve({
      launchMode: 'windowed',
      windowedSize: null,
      gameDir: null,
      versionId: '4.0.002',
      customVersionDir: null,
    });
  }
  return invoke<Settings>('get_settings');
}

export function saveSettings(settings: Settings): Promise<Settings> {
  if (!inTauri) return Promise.resolve(settings);
  return invoke<Settings>('save_settings', { settings });
}

export function getGameStatus(): Promise<GameStatus> {
  if (!inTauri) {
    return Promise.resolve({ found: false, dir: null, reason: '请在桌面应用中运行', version: null, official: false });
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

/** 关闭正在运行的游戏窗口（快速启动页运行中的「取消运行」） */
export function closeGame(): Promise<void> {
  if (!inTauri) return Promise.resolve();
  return invoke<void>('close_game');
}

/** 扫描数据文件夹 versions/ 下的版本与自定义位置 */
export function listVersions(): Promise<VersionScan> {
  if (!inTauri) {
    return Promise.resolve({ dataDir: '', versionsDir: '', versions: [], custom: null });
  }
  return invoke<VersionScan>('list_versions');
}

/** 启动器数据文件夹路径（启动时已确保存在） */
export function getDataDir(): Promise<string> {
  if (!inTauri) return Promise.resolve('');
  return invoke<string>('data_dir');
}

/** 订阅游戏窗口关闭事件，返回取消订阅函数 */
export function onGameClosed(handler: () => void): Promise<UnlistenFn> {
  if (!inTauri) return Promise.resolve(() => {});
  return listen('game-closed', handler);
}
