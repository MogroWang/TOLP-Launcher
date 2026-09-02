<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { BUILTIN_GAME_VERSIONS } from '../lib/tauri';
import type { GameStatus, LaunchMode, Settings } from '../lib/tauri';

const props = defineProps<{
  /** 外部请求打开抽屉：该值每次递增都会打开抽屉 */
  openRequest?: number;
  settings: Settings;
  status: GameStatus;
}>();

const emit = defineEmits<{
  change: [Settings];
}>();

const open = ref(false);
const drawerEl = ref<HTMLElement | null>(null);

const modes: Array<{ value: LaunchMode; label: string }> = [
  { value: 'fullscreen', label: '全屏启动' },
  { value: 'windowed', label: '窗口化启动' },
];

const thumbShift = computed(() => {
  const index = modes.findIndex((m) => m.value === props.settings.launchMode);
  return index <= 0 ? 0 : index;
});

/** 版本来源：内置版本（当前仅 1.0.0 占位）或自定义目录 */
const isBuiltinVersion = computed(() => props.settings.versionId !== null);
const versionThumbShift = computed(() => (isBuiltinVersion.value ? 0 : 1));

function setBuiltinVersion(): void {
  if (isBuiltinVersion.value) return;
  emit('change', { ...props.settings, versionId: BUILTIN_GAME_VERSIONS[0].id });
}

function setCustomVersion(): void {
  if (!isBuiltinVersion.value) return;
  emit('change', { ...props.settings, versionId: null });
}

function openDrawer(): void {
  open.value = true;
  void nextTick(() => {
    drawerEl.value?.querySelector<HTMLButtonElement>('.seg__item')?.focus();
  });
}

function closeDrawer(): void {
  open.value = false;
}

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
    emit('change', { ...props.settings, gameDir: picked });
  }
}

function resetDir(): void {
  emit('change', { ...props.settings, gameDir: null });
}

function onKeydown(event: KeyboardEvent): void {
  if (event.key === 'Escape' && open.value) {
    closeDrawer();
  }
}

watch(
  () => props.openRequest,
  (val) => {
    // 跳过初始挂载（undefined→0）与 0 值，仅响应真正的递增请求
    if (val) {
      openDrawer();
    }
  },
);

window.addEventListener('keydown', onKeydown);
onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));
</script>

<template>
  <aside v-show="open" ref="drawerEl" class="drawer" :data-open="open" role="dialog" aria-label="启动设置">
    <header class="drawer__head">
      <h2>启动设置</h2>
      <button class="drawer__close" type="button" aria-label="关闭设置" @click="closeDrawer">
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

    <section class="drawer__section">
      <h3 class="drawer__label">启动方式</h3>
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

    <section class="drawer__section">
      <h3 class="drawer__label">游戏版本</h3>
      <div class="seg" role="radiogroup" aria-label="游戏版本">
        <span class="seg__thumb" :style="{ transform: `translateX(${versionThumbShift * 100}%)` }" aria-hidden="true"></span>
        <button
          class="seg__item"
          :class="{ 'is-active': isBuiltinVersion }"
          type="button"
          role="radio"
          :aria-checked="isBuiltinVersion"
          @click="setBuiltinVersion"
        >
          {{ BUILTIN_GAME_VERSIONS[0].label }}
        </button>
        <button
          class="seg__item"
          :class="{ 'is-active': !isBuiltinVersion }"
          type="button"
          role="radio"
          :aria-checked="!isBuiltinVersion"
          @click="setCustomVersion"
        >
          自定义目录
        </button>
      </div>
      <p v-if="isBuiltinVersion" class="drawer__hint">
        内置版本 {{ BUILTIN_GAME_VERSIONS[0].id }} 为占位：当前使用启动器同目录的
        game 文件夹启动；后续版本管理将支持多版本的安装与切换。
      </p>
    </section>

    <section v-if="!isBuiltinVersion" class="drawer__section">
      <div class="drawer__label-row">
        <h3 class="drawer__label">游戏目录</h3>
        <span class="drawer__chip" :class="status.found ? 'is-ok' : 'is-missing'">
          <span class="drawer__chip-dot" aria-hidden="true"></span>
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
      <p class="drawer__hint">
        把 GDevelop 导出的网页版游戏（含 index.html 的文件夹）放入启动器同目录的
        game 文件夹，或在此指定其他目录。设置会立即保存。
      </p>
    </section>
  </aside>
</template>

<style scoped>
.drawer {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  width: min(400px, 88%);
  display: flex;
  flex-direction: column;
  gap: 22px;
  padding: 24px 26px;
  overflow-y: auto;
  background: rgba(22, 19, 28, 0.82);
  backdrop-filter: blur(28px) saturate(150%);
  border-left: 1px solid rgba(255, 255, 255, 0.09);
  border-radius: 20px 0 0 20px;
  box-shadow: -28px 0 80px rgba(0, 0, 0, 0.55);
  z-index: 40;
  /* 显示时从右滑入（display 切换会自动重播动画）；隐藏为瞬时，可靠性优先 */
  animation: drawer-in 0.32s cubic-bezier(0.32, 0.72, 0, 1);
}

@keyframes drawer-in {
  from {
    transform: translateX(36px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@media (prefers-reduced-motion: reduce) {
  .drawer {
    animation: none;
  }
}

.drawer__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.drawer__head h2 {
  font-size: 17px;
  font-weight: 500;
  letter-spacing: 0.06em;
}

.drawer__close {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  display: grid;
  place-items: center;
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.75);
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.drawer__close:hover {
  background: rgba(255, 255, 255, 0.16);
  color: #fff;
}

.drawer__close:active {
  transform: scale(0.94);
}

.drawer__section {
  display: flex;
  flex-direction: column;
}

.drawer__label {
  font-size: 11.5px;
  font-weight: 400;
  letter-spacing: 0.18em;
  color: var(--ink-3);
  margin-bottom: 10px;
}

.drawer__label-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.drawer__chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 3px 10px;
  border-radius: 999px;
  font-size: 11px;
  letter-spacing: 0.06em;
}

.drawer__chip-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: currentColor;
}

.drawer__chip.is-ok {
  color: #86efac;
  background: rgba(74, 222, 128, 0.1);
}

.drawer__chip.is-missing {
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

.drawer__hint {
  margin-top: 12px;
  font-size: 12px;
  line-height: 1.75;
  color: rgba(255, 255, 255, 0.38);
}
</style>
