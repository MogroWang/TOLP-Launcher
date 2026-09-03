<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import {
  BUILTIN_GAME_VERSIONS,
  closeGame,
  getGameStatus,
  getSettings,
  isGameRunning,
  launchGame,
  listVersions,
  onGameClosed,
  saveSettings,
  type GameStatus,
  type Settings,
} from './lib/tauri';
import type { PageId } from './components/SideNav.vue';
import TitleBar from './components/TitleBar.vue';
import SideNav from './components/SideNav.vue';
import QuickLaunch from './components/QuickLaunch.vue';
import VersionManager from './components/VersionManager.vue';
import LauncherSettings from './components/LauncherSettings.vue';

const settings = ref<Settings>({
  launchMode: 'fullscreen',
  gameDir: null,
  versionId: BUILTIN_GAME_VERSIONS[0].id,
  customVersionDir: null,
});
const status = ref<GameStatus>({ found: false, dir: null, reason: null, version: null, official: false });
const version = ref('');
const page = ref<PageId>('launch');
const launching = ref(false);
const launchError = ref<string | null>(null);
/** 游戏窗口是否正在运行（启动成功置位，窗口关闭事件复位） */
const running = ref(false);
let unlistenGameClosed: (() => void) | null = null;

onMounted(async () => {
  const [loadedSettings, loadedStatus, scan] = await Promise.all([
    getSettings(),
    getGameStatus(),
    listVersions().catch(() => null),
  ]);
  // 旧版本设置的版本 id 不在当前版本表 / 数据文件夹中时，归一化到最新内置版本
  const knownIds = new Set<string>([
    ...BUILTIN_GAME_VERSIONS.map((v) => v.id),
    ...(scan?.versions ?? []).map((v) => v.id),
  ]);
  const validVersion =
    loadedSettings.versionId !== null && knownIds.has(loadedSettings.versionId);
  settings.value = validVersion
    ? loadedSettings
    : { ...loadedSettings, versionId: loadedSettings.versionId === null ? null : BUILTIN_GAME_VERSIONS[0].id };
  if (settings.value !== loadedSettings) {
    void saveSettings(settings.value);
  }
  status.value = loadedStatus;
  version.value = await getVersion().catch(() => '');
  // 页面可能在游戏运行中被刷新，挂载时向后端同步一次真实状态
  running.value = await isGameRunning();
  unlistenGameClosed = await onGameClosed(() => {
    running.value = false;
  });
});

onBeforeUnmount(() => {
  unlistenGameClosed?.();
  unlistenGameClosed = null;
});

async function applySettings(next: Settings): Promise<void> {
  settings.value = next;
  try {
    settings.value = await saveSettings(next);
  } finally {
    status.value = await getGameStatus();
    launchError.value = null;
  }
}

async function onLaunch(): Promise<void> {
  if (launching.value) return;
  launchError.value = null;
  // 运行中主按钮为「取消运行」：关闭游戏窗口，销毁事件会把 running 复位
  if (running.value) {
    try {
      await closeGame();
      running.value = false;
    } catch (error) {
      launchError.value = String(error);
    }
    return;
  }
  launching.value = true;
  try {
    await launchGame();
    running.value = true;
  } catch (error) {
    launchError.value = String(error);
  } finally {
    launching.value = false;
  }
}
</script>

<template>
  <div class="app">
    <TitleBar />
    <div class="app__body">
      <SideNav :page="page" @update="page = $event" />
      <main class="app__main">
        <div class="app__page">
          <Transition name="page" mode="out-in">
            <QuickLaunch
              v-if="page === 'launch'"
              :settings="settings"
              :status="status"
              :launching="launching"
              :running="running"
              :error="launchError"
              @launch="onLaunch"
              @change="applySettings"
            />
            <VersionManager
              v-else-if="page === 'versions'"
              :settings="settings"
              :status="status"
              @select="(id) => applySettings({ ...settings, versionId: id })"
            />
            <LauncherSettings
              v-else
              :settings="settings"
              :status="status"
              :version="version"
              @change="applySettings"
            />
          </Transition>
        </div>
        <footer class="app__footer">
          <span>MOGROWANG STUDIO</span>
          <span>
            光点之旅 TOUR OF LIGHT POINT<template v-if="version"> · V{{ version }}</template>
          </span>
        </footer>
      </main>
    </div>
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.app__body {
  flex: 1;
  min-height: 0;
  display: flex;
}

.app__main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}

.app__page {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 页面切换：短促淡入 + 轻微上浮，避免方向性滑动带来的跳跃感 */
.page-enter-active {
  transition: opacity 0.18s ease-out, transform 0.18s cubic-bezier(0.32, 0.72, 0, 1);
}

.page-leave-active {
  transition: opacity 0.12s ease-in;
}

.page-enter-from {
  opacity: 0;
  transform: translateY(8px);
}

.page-leave-to {
  opacity: 0;
}

@media (prefers-reduced-motion: reduce) {
  .page-enter-active,
  .page-leave-active {
    transition: opacity 0.15s ease;
  }

  .page-enter-from {
    transform: none;
  }
}

.app__footer {
  flex: none;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 22px;
  font-size: 10px;
  letter-spacing: 0.22em;
  color: var(--ink-4);
  user-select: none;
}
</style>
