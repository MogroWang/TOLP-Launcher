<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import type { GameStatus, LaunchMode, Settings, WindowedSize } from '../lib/tauri';
import SegmentedControl from './SegmentedControl.vue';
import Dropdown from './Dropdown.vue';
import Slider from './Slider.vue';

const props = defineProps<{
  settings: Settings;
  status: GameStatus;
}>();

const emit = defineEmits<{
  change: [Settings];
}>();

const modeOptions: Array<{ value: LaunchMode; label: string }> = [
  { value: 'fullscreen', label: '全屏启动' },
  { value: 'windowed', label: '窗口化启动' },
];

/** 窗口大小预设 */
const WINDOW_PRESETS: ReadonlyArray<{
  value: string;
  label: string;
  desc?: string;
  width: number;
  height: number;
}> = [
  { value: '960x540', label: '960 × 540', desc: '小窗口', width: 960, height: 540 },
  { value: '1280x720', label: '1280 × 720', desc: '推荐', width: 1280, height: 720 },
  { value: '1600x900', label: '1600 × 900', width: 1600, height: 900 },
  { value: '1920x1080', label: '1920 × 1080', desc: '大窗口', width: 1920, height: 1080 },
];

/** 自定义滑块的调节范围（后端另有安全夹取） */
const SIZE_RANGE = { minWidth: 640, maxWidth: 3840, minHeight: 360, maxHeight: 2160 };

const sizeOptions = [
  ...WINDOW_PRESETS.map(({ value, label, desc }) => ({ value, label, desc })),
  { value: 'custom', label: '自定义尺寸', desc: '拖动滑块调节宽高' },
];

/** 「自定义尺寸」是否被选中（选中后展开宽高滑块） */
const customMode = ref(false);

/** 当前生效的窗口大小；设置未保存过时为默认 1280×720 */
function currentSize(): WindowedSize {
  return props.settings.windowedSize ?? { width: 1280, height: 720 };
}

const sizeValue = computed<string>(() => {
  if (customMode.value) return 'custom';
  const { width, height } = currentSize();
  const hit = WINDOW_PRESETS.find((p) => p.width === width && p.height === height);
  return hit ? hit.value : 'custom';
});

/** 自定义滑块的本地值：拖动中实时变化，change（松手）时才写入设置 */
const customWidth = ref(1280);
const customHeight = ref(720);

watch(
  () => props.settings.windowedSize,
  (size) => {
    customWidth.value = size?.width ?? 1280;
    customHeight.value = size?.height ?? 720;
  },
  { immediate: true },
);

function onSizePreset(value: string): void {
  if (value === 'custom') {
    customMode.value = true;
    return;
  }
  customMode.value = false;
  const preset = WINDOW_PRESETS.find((p) => p.value === value);
  if (!preset) return;
  const { width, height } = currentSize();
  if (width === preset.width && height === preset.height) return;
  emit('change', { ...props.settings, windowedSize: { width: preset.width, height: preset.height } });
}

/** 滑块一次调整结束：提交自定义窗口大小 */
function commitSize(): void {
  const size = {
    width: Math.round(customWidth.value),
    height: Math.round(customHeight.value),
  };
  const cur = props.settings.windowedSize;
  if (cur && cur.width === size.width && cur.height === size.height) return;
  emit('change', { ...props.settings, windowedSize: size });
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
            <span class="ls__row-desc">游戏窗口以全屏或设定尺寸的窗口打开</span>
          </div>
          <SegmentedControl
            class="ls__seg"
            :options="modeOptions"
            :model-value="settings.launchMode"
            label="启动方式"
            @update:model-value="setMode"
          />
        </div>

        <div v-if="settings.launchMode === 'windowed'" class="ls__row ls__row--stacked">
          <div class="ls__row-text">
            <span class="ls__row-title">窗口大小</span>
            <span class="ls__row-desc">游戏窗口的显示尺寸，可选预设或自定义</span>
          </div>
          <Dropdown
            :options="sizeOptions"
            :model-value="sizeValue"
            label="窗口大小"
            @update:model-value="onSizePreset"
          />
          <div v-if="sizeValue === 'custom'" class="ls__sliders">
            <div class="ls__slider">
              <span class="ls__slider-name">宽度</span>
              <Slider
                v-model="customWidth"
                :min="SIZE_RANGE.minWidth"
                :max="SIZE_RANGE.maxWidth"
                label="窗口宽度"
                @change="commitSize"
              />
              <span class="ls__slider-value">{{ customWidth }} px</span>
            </div>
            <div class="ls__slider">
              <span class="ls__slider-name">高度</span>
              <Slider
                v-model="customHeight"
                :min="SIZE_RANGE.minHeight"
                :max="SIZE_RANGE.maxHeight"
                label="窗口高度"
                @change="commitSize"
              />
              <span class="ls__slider-value">{{ customHeight }} px</span>
            </div>
          </div>
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
          选择内置版本时优先使用数据文件夹
          <code>launcher-data/versions/&lt;版本&gt;/</code>，未识别到时可在版本管理中指定自定义位置；
          把 GDevelop 导出的网页版游戏（含 index.html 的文件夹）放入同目录
          <code>game</code> 文件夹，即可作为默认游戏。游戏存档保存在数据文件夹的
          <code>saves</code> 子目录中。
        </p>
      </div>
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

/* 自定义窗口大小的宽高滑块 */
.ls__sliders {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 4px 14px 10px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.035);
  border: 1px solid rgba(255, 255, 255, 0.06);
}

.ls__slider {
  display: flex;
  align-items: center;
  gap: 14px;
}

.ls__slider-name {
  flex: none;
  width: 34px;
  font-size: 12px;
  letter-spacing: 0.08em;
  color: var(--ink-3);
}

.ls__slider-value {
  flex: none;
  width: 76px;
  text-align: right;
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--accent-soft);
  font-variant-numeric: tabular-nums;
}
</style>
