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
import OrbButton from './components/OrbButton.vue';
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
      <HeroSection
        :status="status"
        :launching="launching"
        :error="launchError"
        @launch="onLaunch"
        @fix="sheetOpen = true"
      />
      <OrbButton
        class="app__settings-orb"
        :size="52"
        tone="white"
        label="启动设置"
        @click="sheetOpen = true"
      >
        <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
          <g stroke="#fff" stroke-width="2" stroke-linecap="round">
            <path d="M4 6.5h8.8" />
            <path d="M18.2 6.5H20" />
            <path d="M4 12h2.6" />
            <path d="M11.8 12H20" />
            <path d="M4 17.5h6.8" />
            <path d="M16 17.5h4" />
          </g>
          <g fill="#fff">
            <circle cx="15.4" cy="6.5" r="2.2" />
            <circle cx="9.2" cy="12" r="2.2" />
            <circle cx="13.4" cy="17.5" r="2.2" />
          </g>
        </svg>
      </OrbButton>
      <footer class="app__footer">
        <span>MOGROWANG STUDIO</span>
        <span>
          光点之旅 TOUR OF LIGHT POINT<template v-if="version"> · V{{ version }}</template>
        </span>
      </footer>
    </main>
    <GameStage v-if="gameUrl" :url="gameUrl" @exit="onExitGame" />
    <SettingsSheet
      :open="sheetOpen"
      :settings="settings"
      :status="status"
      @update:open="sheetOpen = $event"
      @change="applySettings"
    />
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
}

.app__settings-orb {
  position: absolute;
  right: 30px;
  bottom: 52px;
}

.app__footer {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 22px;
  font-size: 10px;
  letter-spacing: 0.22em;
  color: var(--ink-4);
  pointer-events: none;
}
</style>

