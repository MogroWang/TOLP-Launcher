<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import { getCurrentWindow } from '@tauri-apps/api/window';
import {
  getGameStatus,
  getSettings,
  launchGame,
  saveSettings,
  type GameStatus,
  type Settings,
} from './lib/tauri';
import TitleBar from './components/TitleBar.vue';
import HeroSection from './components/HeroSection.vue';
import SettingsSheet from './components/SettingsSheet.vue';
import GameStage from './components/GameStage.vue';

const settings = ref<Settings>({ launchMode: 'fullscreen', gameDir: null });
const status = ref<GameStatus>({ found: false, dir: null, reason: null });
const version = ref('');
const sheetOpen = ref(false);
const launching = ref(false);
const launchError = ref<string | null>(null);
const gameUrl = ref<string | null>(null);

onMounted(async () => {
  const [loadedSettings, loadedStatus] = await Promise.all([getSettings(), getGameStatus()]);
  settings.value = loadedSettings;
  status.value = loadedStatus;
  version.value = await getVersion().catch(() => '');
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
  if (launching.value || gameUrl.value) return;
  launchError.value = null;
  launching.value = true;
  sheetOpen.value = false;
  try {
    const result = await launchGame();
    const win = getCurrentWindow();
    await win.setFullscreen(result.fullscreen);
    gameUrl.value = result.url;
  } catch (error) {
    launchError.value = String(error);
  } finally {
    launching.value = false;
  }
}

async function onExitGame(): Promise<void> {
  gameUrl.value = null;
  status.value = await getGameStatus().catch(() => status.value);
}
</script>

<template>
  <div class="app">
    <TitleBar v-show="!gameUrl" />
    <main v-show="!gameUrl" class="app__stage">
      <div class="app__shift" :class="{ 'is-shifted': sheetOpen }">
        <HeroSection
          :status="status"
          :launching="launching"
          :error="launchError"
          :settings-open="sheetOpen"
          @launch="onLaunch"
          @toggle-settings="sheetOpen = !sheetOpen"
        />
      </div>
      <footer class="app__footer">
        <span>MOGROWANG STUDIO</span>
        <span>
          光点之旅 TOUR OF LIGHT POINT<template v-if="version"> · V{{ version }}</template>
        </span>
      </footer>
      <SettingsSheet
        :open="sheetOpen"
        :settings="settings"
        :status="status"
        @update:open="sheetOpen = $event"
        @change="applySettings"
      />
    </main>
    <GameStage v-if="gameUrl" :url="gameUrl" @exit="onExitGame" />
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

/* 抽屉打开时主页内容平移避让，位移量与抽屉宽度精确互补 */
.app__shift {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  transition: transform 0.32s cubic-bezier(0.32, 0.72, 0, 1);
}

.app__shift.is-shifted {
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
