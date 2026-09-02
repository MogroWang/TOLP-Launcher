<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import {
  getGameStatus,
  getSettings,
  isGameRunning,
  launchGame,
  onGameClosed,
  saveSettings,
  type GameStatus,
  type Settings,
} from './lib/tauri';
import TitleBar from './components/TitleBar.vue';
import HeroSection from './components/HeroSection.vue';
import SettingsSheet from './components/SettingsSheet.vue';

const settings = ref<Settings>({ launchMode: 'fullscreen', gameDir: null, versionId: '1.0.0' });
const status = ref<GameStatus>({ found: false, dir: null, reason: null });
const version = ref('');
/** 递增计数作为“打开抽屉”的请求信号，抽屉内部自行管理开关 */
const settingsRequest = ref(0);
const launching = ref(false);
const launchError = ref<string | null>(null);
/** 游戏窗口是否正在运行（启动成功置位，窗口关闭事件复位） */
const running = ref(false);
let unlistenGameClosed: (() => void) | null = null;

onMounted(async () => {
  const [loadedSettings, loadedStatus] = await Promise.all([getSettings(), getGameStatus()]);
  settings.value = loadedSettings;
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
    <main class="app__stage">
      <div class="app__shift">
        <HeroSection
          :status="status"
          :launching="launching"
          :running="running"
          :error="launchError"
          @launch="onLaunch"
          @request-settings="settingsRequest++"
        />
      </div>
      <footer class="app__footer">
        <span>MOGROWANG STUDIO</span>
        <span>
          光点之旅 TOUR OF LIGHT POINT<template v-if="version"> · V{{ version }}</template>
        </span>
      </footer>
      <SettingsSheet
        :open-request="settingsRequest"
        :settings="settings"
        :status="status"
        @change="applySettings"
      />
    </main>
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.app__stage {
  position: relative;
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 抽屉打开时主页内容平移避让（位移量与抽屉宽度精确互补）；
   开合状态由 CSS :has() 直接从抽屉元素读取，不依赖跨组件状态 */
.app__shift {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  transition: transform 0.32s cubic-bezier(0.32, 0.72, 0, 1);
}

.app__stage:has(.drawer[data-open='true']) .app__shift {
  transform: translateX(calc(-1 * min(400px, 88%) / 2));
}

@media (prefers-reduced-motion: reduce) {
  .app__shift {
    transition: none;
  }
}

.app__footer {
  flex: none;
  height: 44px;
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
