<script setup lang="ts">
import { computed } from 'vue';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { BUILTIN_GAME_VERSIONS } from '../lib/tauri';
import type { GameStatus, LaunchMode, Settings } from '../lib/tauri';
import SegmentedControl from './SegmentedControl.vue';
import tolpLogo from '../assets/tolp-logo.png';

const props = defineProps<{
  settings: Settings;
  status: GameStatus;
  launching: boolean;
  running: boolean;
  error: string | null;
}>();

const emit = defineEmits<{
  launch: [];
  change: [Settings];
}>();

const builtinVersion = BUILTIN_GAME_VERSIONS[0];

const versionOptions = [
  { value: 'builtin', label: builtinVersion.label },
  { value: 'custom', label: '自定义启动' },
] as const;

const modeOptions: Array<{ value: LaunchMode; label: string }> = [
  { value: 'fullscreen', label: '全屏启动' },
  { value: 'windowed', label: '窗口化启动' },
];

const versionValue = computed(() => (props.settings.versionId !== null ? 'builtin' : 'custom'));

/** 运行中点击按钮把已有游戏窗口带回前台（后端复用窗口逻辑） */
const playLabel = computed(() => {
  if (props.launching) return '正在启动…';
  if (props.running) return '回到游戏';
  return props.status.found ? '启动游戏' : '未找到游戏文件';
});

function setVersion(value: 'builtin' | 'custom'): void {
  const versionId = value === 'builtin' ? builtinVersion.id : null;
  if (props.settings.versionId === versionId) return;
  emit('change', { ...props.settings, versionId });
}

function setMode(mode: LaunchMode): void {
  if (props.settings.launchMode === mode) return;
  emit('change', { ...props.settings, launchMode: mode });
}

/** 快捷指定目录：选择后自动切到自定义启动，立即生效 */
async function chooseDir(): Promise<void> {
  const picked = await openDialog({
    directory: true,
    multiple: false,
    title: '选择游戏目录（需包含 index.html）',
  });
  if (typeof picked === 'string') {
    emit('change', { ...props.settings, versionId: null, gameDir: picked });
  }
}
</script>

<template>
  <section class="ql">
    <img class="ql__logo" :src="tolpLogo" alt="光点之旅" draggable="false" />
    <p class="ql__sub">TOUR OF LIGHT POINT</p>

    <!-- 圆角药丸启动按钮：紫色光体质感延续游戏菜单的主球 -->
    <button
      class="ql__play"
      :class="{ 'is-loading': launching }"
      type="button"
      :disabled="(!status.found && !running) || launching"
      :aria-busy="launching || undefined"
      @click="emit('launch')"
    >
      <svg viewBox="0 0 20 20" aria-hidden="true">
        <path
          d="M6.2 4.6 Q6.2 2.9 7.7 3.7 L16 9.2 Q17.4 10 16 10.8 L7.7 16.3 Q6.2 17.1 6.2 15.4 Z"
          fill="currentColor"
        />
      </svg>
      <span>{{ playLabel }}</span>
    </button>

    <div class="ql__chooser">
      <SegmentedControl
        :options="versionOptions"
        :model-value="versionValue"
        label="游戏版本"
        @update:model-value="setVersion($event as 'builtin' | 'custom')"
      />
    </div>

    <div class="ql__chooser">
      <SegmentedControl
        :options="modeOptions"
        :model-value="settings.launchMode"
        label="启动方式"
        @update:model-value="setMode"
      />
    </div>

    <div class="ql__meta" aria-live="polite">
      <template v-if="status.found">
        <span class="ql__dot ql__dot--ok" aria-hidden="true"></span>
        <span>{{ running ? '游戏正在运行' : '游戏已就绪' }}</span>
        <span class="ql__path" :title="status.dir ?? ''">{{ status.dir }}</span>
      </template>
      <template v-else>
        <span class="ql__dot ql__dot--missing" aria-hidden="true"></span>
        <span>{{ status.reason ?? '尚未找到游戏文件夹' }}</span>
      </template>
      <button class="ql__pick" type="button" @click="chooseDir">选择文件夹</button>
    </div>

    <p v-if="error" class="ql__error">{{ error }}</p>
  </section>
</template>

<style scoped>
.ql {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 22px;
  padding: 12px 32px 20px;
  position: relative;
  overflow-y: auto;
}

/* 光点的环境光，让黑色不再空洞 —— 与游戏菜单一致 */
.ql::before {
  content: '';
  position: absolute;
  width: min(66vmin, 620px);
  height: min(66vmin, 620px);
  border-radius: 50%;
  background: radial-gradient(circle, rgba(168, 85, 247, 0.1) 0%, transparent 62%);
  top: 44%;
  left: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
}

.ql__logo {
  position: relative;
  height: clamp(96px, 20vmin, 150px);
  width: auto;
  user-select: none;
  pointer-events: none;
}

.ql__sub {
  position: relative;
  margin-top: -6px;
  font-size: 10.5px;
  letter-spacing: 0.46em;
  text-indent: 0.46em;
  color: var(--ink-4);
}

.ql__play {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 13px;
  height: 62px;
  padding: 0 46px;
  margin-top: 12px;
  border-radius: 999px;
  font-size: 15.5px;
  letter-spacing: 0.2em;
  text-indent: 0.1em;
  color: #fff;
  background: radial-gradient(140% 220% at 50% -40%, #efd9fc 0%, #cb8df4 32%, #a855f7 64%, #8a3ed2 100%);
  box-shadow:
    0 0 26px rgba(168, 85, 247, 0.5),
    0 0 84px rgba(168, 85, 247, 0.24),
    inset 0 2px 5px rgba(255, 255, 255, 0.38),
    inset 0 -9px 18px rgba(96, 22, 168, 0.42);
  transition: transform 0.18s cubic-bezier(0.2, 0.7, 0.3, 1), filter 0.18s ease, box-shadow 0.18s ease;
  will-change: transform;
  -webkit-tap-highlight-color: transparent;
}

.ql__play svg {
  width: 19px;
  height: 19px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.22));
}

.ql__play:hover:not(:disabled) {
  transform: scale(1.03);
  filter: brightness(1.07);
  box-shadow:
    0 0 34px rgba(168, 85, 247, 0.62),
    0 0 110px rgba(168, 85, 247, 0.3),
    inset 0 2px 5px rgba(255, 255, 255, 0.38),
    inset 0 -9px 18px rgba(96, 22, 168, 0.42);
}

/* 按压反馈在按下瞬间出现，而不是松开时 */
.ql__play:active:not(:disabled) {
  transform: scale(0.97);
  transition-duration: 0.09s;
}

.ql__play:disabled {
  opacity: 0.45;
}

/* 加载态：外圈旋转弧线 */
.ql__play.is-loading::before {
  content: '';
  position: absolute;
  inset: -8px;
  border-radius: 999px;
  border: 2px solid rgba(255, 255, 255, 0.2);
  border-top-color: rgba(255, 255, 255, 0.9);
  animation: ql-spin 0.9s linear infinite;
}

@keyframes ql-spin {
  to {
    transform: rotate(360deg);
  }
}

.ql__chooser {
  position: relative;
  width: min(340px, 100%);
}

.ql__chooser + .ql__chooser {
  margin-top: -4px;
}

.ql__meta {
  position: relative;
  display: flex;
  align-items: center;
  gap: 10px;
  max-width: min(88%, 620px);
  margin-top: 10px;
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--ink-2);
}

.ql__dot {
  flex: none;
  width: 7px;
  height: 7px;
  border-radius: 50%;
}

.ql__dot--ok {
  background: var(--ok);
  box-shadow: 0 0 9px rgba(74, 222, 128, 0.8);
}

.ql__dot--missing {
  background: var(--danger);
  box-shadow: 0 0 9px rgba(255, 138, 128, 0.8);
}

.ql__path {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 300px;
  color: var(--ink-3);
}

.ql__pick {
  flex: none;
  padding: 5px 14px;
  border-radius: 999px;
  font-size: 12px;
  letter-spacing: 0.05em;
  color: var(--accent-soft);
  background: rgba(168, 85, 247, 0.1);
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.ql__pick:hover {
  background: rgba(168, 85, 247, 0.2);
}

.ql__pick:active {
  transform: scale(0.96);
}

.ql__error {
  position: relative;
  max-width: min(88%, 620px);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12.5px;
  color: var(--danger);
  margin-top: -6px;
}
</style>
