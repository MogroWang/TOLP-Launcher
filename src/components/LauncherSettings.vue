<script setup lang="ts">
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import type { GameStatus, LaunchMode, Settings } from '../lib/tauri';
import SegmentedControl from './SegmentedControl.vue';
import tolpLogo from '../assets/tolp-logo.png';

const props = defineProps<{
  settings: Settings;
  status: GameStatus;
  /** 启动器自身版本（tauri app.getVersion） */
  version: string;
}>();

const emit = defineEmits<{
  change: [Settings];
}>();

const modeOptions: Array<{ value: LaunchMode; label: string }> = [
  { value: 'fullscreen', label: '全屏启动' },
  { value: 'windowed', label: '窗口化启动' },
];

function setMode(mode: LaunchMode): void {
  emit('change', { ...props.settings, launchMode: mode });
}

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

function resetDir(): void {
  emit('change', { ...props.settings, gameDir: null });
}
</script>

<template>
  <section class="ls">
    <header class="ls__head">
      <h2>启动器设置</h2>
      <p>配置启动方式与游戏目录，更改会立即保存。</p>
    </header>

    <div class="ls__group">
      <h3 class="ls__label">启动</h3>
      <div class="ls__card">
        <div class="ls__row">
          <div class="ls__row-text">
            <span class="ls__row-title">启动方式</span>
            <span class="ls__row-desc">游戏窗口以全屏或 1280×720 窗口打开</span>
          </div>
          <SegmentedControl
            class="ls__seg"
            :options="modeOptions"
            :model-value="settings.launchMode"
            label="启动方式"
            @update:model-value="setMode"
          />
        </div>

        <div v-if="settings.versionId === null" class="ls__row ls__row--stacked">
          <div class="ls__row-text">
            <span class="ls__row-title">游戏目录</span>
            <span class="ls__row-desc">
              自定义启动时使用此目录，需包含 index.html
              <span class="ls__chip" :class="status.found ? 'is-ok' : 'is-missing'">
                <span class="ls__chip-dot" aria-hidden="true"></span>
                {{ status.found ? '已就绪' : '未找到游戏' }}
              </span>
            </span>
          </div>
          <div class="ls__dirbox">
            <span class="ls__dir" :title="status.dir ?? ''">
              {{ status.dir ?? '默认使用启动器同目录的 game 文件夹' }}
            </span>
            <span class="ls__dir-actions">
              <button class="ls__btn" type="button" @click="chooseDir">选择…</button>
              <button v-if="settings.gameDir" class="ls__btn ls__btn--ghost" type="button" @click="resetDir">
                恢复默认
              </button>
            </span>
          </div>
        </div>

        <p class="ls__hint">
          选择内置版本时优先使用启动器同目录
          <code>games/&lt;版本&gt;/</code> 托管目录；把 GDevelop 导出的网页版游戏（含
          index.html 的文件夹）放入同目录 <code>game</code> 文件夹，即可作为默认游戏。
        </p>
      </div>
    </div>

    <div class="ls__group">
      <h3 class="ls__label">关于</h3>
      <div class="ls__card">
        <div class="ls__about">
          <img class="ls__about-logo" :src="tolpLogo" alt="光点之旅" draggable="false" />
          <div class="ls__about-text">
            <strong>TOLP 启动器</strong>
            <span>TOUR OF LIGHT POINT LAUNCHER</span>
          </div>
          <span class="ls__about-ver">{{ version ? `V${version}` : '—' }}</span>
        </div>
        <div class="ls__row">
          <span class="ls__row-title">游戏</span>
          <span class="ls__row-value">光点之旅 · Tour of Light Point</span>
        </div>
        <div class="ls__row">
          <span class="ls__row-title">开发者</span>
          <span class="ls__row-value">MOGROWANG STUDIO</span>
        </div>
      </div>
      <p class="ls__footnote">为光点之旅打造的桌面启动器 · 设置保存于启动器同目录，不写入系统注册表。</p>
    </div>
  </section>
</template>

<style scoped>
.ls {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 34px 44px 20px;
  overflow-y: auto;
}

.ls__head {
  width: min(640px, 100%);
  margin-bottom: 22px;
}

.ls__head h2 {
  font-size: 21px;
  font-weight: 500;
  letter-spacing: 0.08em;
}

.ls__head p {
  margin-top: 7px;
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--ink-3);
}

.ls__group {
  width: min(640px, 100%);
}

.ls__group + .ls__group {
  margin-top: 24px;
}

.ls__label {
  font-size: 11.5px;
  font-weight: 400;
  letter-spacing: 0.18em;
  color: var(--ink-3);
  margin-bottom: 10px;
}

.ls__card {
  padding: 6px 20px;
  border-radius: 18px;
  background: rgba(255, 255, 255, 0.045);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.ls__row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
  padding: 13px 0;
}

.ls__row + .ls__row {
  border-top: 1px solid rgba(255, 255, 255, 0.06);
}

.ls__row--stacked {
  flex-direction: column;
  align-items: stretch;
  gap: 12px;
}

.ls__row-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.ls__row-title {
  font-size: 13.5px;
  letter-spacing: 0.05em;
  color: #fff;
}

.ls__row-desc {
  font-size: 12px;
  line-height: 1.6;
  letter-spacing: 0.03em;
  color: var(--ink-3);
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.ls__row-value {
  font-size: 13px;
  letter-spacing: 0.04em;
  color: var(--ink-2);
  text-align: right;
}

.ls__seg {
  flex: none;
}

.ls__chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 2px 9px;
  border-radius: 999px;
  font-size: 11px;
  letter-spacing: 0.06em;
}

.ls__chip-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: currentColor;
}

.ls__chip.is-ok {
  color: #86efac;
  background: rgba(74, 222, 128, 0.1);
}

.ls__chip.is-missing {
  color: #fda4af;
  background: rgba(248, 113, 113, 0.1);
}

.ls__dirbox {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.ls__dir {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12.5px;
  color: var(--ink-2);
  user-select: text;
}

.ls__dir-actions {
  display: flex;
  gap: 8px;
  flex: none;
}

.ls__btn {
  padding: 6px 15px;
  border-radius: 999px;
  font-size: 12.5px;
  letter-spacing: 0.06em;
  background: var(--surface-strong);
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.ls__btn:hover {
  background: rgba(255, 255, 255, 0.16);
}

.ls__btn:active {
  transform: scale(0.97);
}

.ls__btn--ghost {
  background: transparent;
  border: 1px solid rgba(255, 255, 255, 0.14);
}

.ls__btn--ghost:hover {
  background: rgba(255, 255, 255, 0.08);
}

.ls__hint {
  padding: 2px 0 14px;
  font-size: 12px;
  line-height: 1.75;
  color: rgba(255, 255, 255, 0.38);
}

.ls__hint code {
  font-family: inherit;
  font-size: 11.5px;
  color: var(--accent-soft);
  background: rgba(168, 85, 247, 0.1);
  padding: 1px 6px;
  border-radius: 6px;
}

.ls__about {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 0;
}

.ls__about-logo {
  flex: none;
  width: 52px;
  height: auto;
  user-select: none;
  pointer-events: none;
}

.ls__about-text {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.ls__about-text strong {
  font-size: 14.5px;
  font-weight: 500;
  letter-spacing: 0.08em;
  color: #fff;
}

.ls__about-text span {
  font-size: 10px;
  letter-spacing: 0.28em;
  color: var(--ink-4);
}

.ls__about-ver {
  flex: none;
  font-size: 12.5px;
  letter-spacing: 0.08em;
  color: var(--ink-2);
}

.ls__footnote {
  margin-top: 12px;
  font-size: 12px;
  line-height: 1.75;
  color: rgba(255, 255, 255, 0.38);
}
</style>
