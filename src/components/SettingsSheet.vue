<script setup lang="ts">
import { computed, onBeforeUnmount, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import type { GameStatus, LaunchMode, Settings } from '../lib/tauri';

const props = defineProps<{
  open: boolean;
  settings: Settings;
  status: GameStatus;
}>();

const emit = defineEmits<{
  'update:open': [boolean];
  change: [Settings];
}>();

const modes: Array<{ value: LaunchMode; label: string }> = [
  { value: 'fullscreen', label: '全屏启动' },
  { value: 'windowed', label: '窗口化启动' },
];

const thumbShift = computed(() => {
  const index = modes.findIndex((m) => m.value === props.settings.launchMode);
  return index <= 0 ? 0 : index;
});

function setMode(mode: LaunchMode): void {
  emit('change', { ...props.settings, launchMode: mode });
}

async function chooseDir(): Promise<void> {
  const picked = await open({
    directory: true,
    multiple: false,
    title: '选择游戏目录（需包含 index.html）',
  });
  if (typeof picked === 'string') {
    emit('change', { ...props.settings, gameDir: picked });
  }
}

function resetDir(): void {
  emit('change', { ...props.settings, gameDir: null });
}

function onKeydown(event: KeyboardEvent): void {
  if (event.key === 'Escape') {
    emit('update:open', false);
  }
}

watch(
  () => props.open,
  (open) => {
    if (open) {
      window.addEventListener('keydown', onKeydown);
    } else {
      window.removeEventListener('keydown', onKeydown);
    }
  },
);

onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));
</script>

<template>
  <Transition name="sheet">
    <div v-if="open" class="sheet-root">
      <div class="sheet__scrim" @click="emit('update:open', false)"></div>
      <div class="sheet" role="dialog" aria-modal="true" aria-label="启动设置">
        <header class="sheet__head">
          <h2>启动设置</h2>
          <button class="sheet__close" type="button" aria-label="关闭设置" @click="emit('update:open', false)">
            <svg viewBox="0 0 14 14" width="13" height="13" aria-hidden="true">
              <path
                d="M3.4 3.4l7.2 7.2M10.6 3.4l-7.2 7.2"
                stroke="currentColor"
                stroke-width="1.4"
                stroke-linecap="round"
              />
            </svg>
          </button>
        </header>

        <section class="sheet__section">
          <h3 class="sheet__label">启动方式</h3>
          <div class="seg" role="radiogroup" aria-label="启动方式">
            <span class="seg__thumb" :style="{ transform: `translateX(${thumbShift * 100}%)` }" aria-hidden="true"></span>
            <button
              v-for="m in modes"
              :key="m.value"
              class="seg__item"
              :class="{ 'is-active': settings.launchMode === m.value }"
              type="button"
              role="radio"
              :aria-checked="settings.launchMode === m.value"
              @click="setMode(m.value)"
            >
              {{ m.label }}
            </button>
          </div>
        </section>

        <section class="sheet__section">
          <div class="sheet__label-row">
            <h3 class="sheet__label">游戏目录</h3>
            <span class="sheet__chip" :class="status.found ? 'is-ok' : 'is-missing'">
              <span class="sheet__chip-dot" aria-hidden="true"></span>
              {{ status.found ? '已就绪' : '未找到游戏' }}
            </span>
          </div>
          <div class="dirbox">
            <span class="dirbox__path" :title="status.dir ?? ''">
              {{ status.dir ?? '默认使用启动器同目录的 game 文件夹' }}
            </span>
            <div class="dirbox__actions">
              <button class="btn" type="button" @click="chooseDir">选择…</button>
              <button v-if="settings.gameDir" class="btn btn--ghost" type="button" @click="resetDir">
                恢复默认
              </button>
            </div>
          </div>
          <p class="sheet__hint">
            把 GDevelop 导出的网页版游戏（含 index.html 的文件夹）放入启动器同目录的
            game 文件夹，或在此指定其他目录。设置会立即保存。
          </p>
        </section>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.sheet-root {
  position: fixed;
  inset: 0;
  z-index: 40;
  display: grid;
  place-items: end center;
}

.sheet__scrim {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
}

.sheet {
  position: relative;
  width: min(560px, calc(100vw - 48px));
  margin-bottom: 22px;
  padding: 22px 26px 26px;
  border-radius: 22px;
  background: rgba(22, 19, 28, 0.78);
  backdrop-filter: blur(28px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.09);
  box-shadow: 0 24px 80px rgba(0, 0, 0, 0.6);
}

.sheet__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.sheet__head h2 {
  font-size: 17px;
  font-weight: 500;
  letter-spacing: 0.06em;
}

.sheet__close {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  display: grid;
  place-items: center;
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.75);
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.sheet__close:hover {
  background: rgba(255, 255, 255, 0.16);
  color: #fff;
}

.sheet__close:active {
  transform: scale(0.94);
}

.sheet__section + .sheet__section {
  margin-top: 22px;
}

.sheet__label {
  font-size: 11.5px;
  font-weight: 400;
  letter-spacing: 0.18em;
  color: var(--ink-3);
  margin-bottom: 10px;
}

.sheet__label-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sheet__chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 3px 10px;
  border-radius: 999px;
  font-size: 11px;
  letter-spacing: 0.06em;
}

.sheet__chip-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: currentColor;
}

.sheet__chip.is-ok {
  color: #86efac;
  background: rgba(74, 222, 128, 0.1);
}

.sheet__chip.is-missing {
  color: #fda4af;
  background: rgba(248, 113, 113, 0.1);
}

.seg {
  position: relative;
  display: grid;
  grid-template-columns: 1fr 1fr;
  padding: 4px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.07);
}

.seg__thumb {
  position: absolute;
  top: 4px;
  left: 4px;
  width: calc(50% - 4px);
  height: calc(100% - 8px);
  border-radius: 999px;
  background: linear-gradient(135deg, #b96cf5, #9333ea);
  box-shadow: 0 0 16px rgba(168, 85, 247, 0.45), inset 0 1px 0 rgba(255, 255, 255, 0.25);
  transition: transform 0.26s cubic-bezier(0.32, 0.72, 0, 1);
}

.seg__item {
  position: relative;
  z-index: 1;
  height: 38px;
  border-radius: 999px;
  font-size: 13.5px;
  letter-spacing: 0.08em;
  color: rgba(255, 255, 255, 0.55);
  transition: color 0.2s ease;
}

.seg__item:hover {
  color: rgba(255, 255, 255, 0.8);
}

.seg__item.is-active {
  color: #fff;
}

.dirbox {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 11px 14px;
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.dirbox__path {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12.5px;
  color: var(--ink-2);
  user-select: text;
}

.dirbox__actions {
  display: flex;
  gap: 8px;
  flex: none;
}

.btn {
  padding: 7px 16px;
  border-radius: 999px;
  font-size: 12.5px;
  letter-spacing: 0.06em;
  background: var(--surface-strong);
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.btn:hover {
  background: rgba(255, 255, 255, 0.16);
}

.btn:active {
  transform: scale(0.97);
}

.btn--ghost {
  background: transparent;
  border: 1px solid rgba(255, 255, 255, 0.14);
}

.btn--ghost:hover {
  background: rgba(255, 255, 255, 0.08);
}

.sheet__hint {
  margin-top: 12px;
  font-size: 12px;
  line-height: 1.75;
  color: rgba(255, 255, 255, 0.38);
}

.sheet-enter-active .sheet {
  transition: transform 0.34s cubic-bezier(0.32, 0.72, 0, 1), opacity 0.34s ease;
}

.sheet-leave-active .sheet {
  transition: transform 0.22s cubic-bezier(0.4, 0, 1, 1), opacity 0.22s ease;
}

.sheet-enter-from .sheet,
.sheet-leave-to .sheet {
  transform: translateY(26px) scale(0.97);
  opacity: 0;
}

.sheet-enter-active .sheet__scrim,
.sheet-leave-active .sheet__scrim {
  transition: opacity 0.28s ease;
}

.sheet-enter-from .sheet__scrim,
.sheet-leave-to .sheet__scrim {
  opacity: 0;
}
</style>
