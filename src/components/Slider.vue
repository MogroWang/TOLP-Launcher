<script setup lang="ts">
import { computed, ref } from 'vue';

const props = withDefaults(
  defineProps<{
    modelValue: number;
    min: number;
    max: number;
    step?: number;
    /** 无障碍标签（slider 名称） */
    label: string;
  }>(),
  { step: 1 },
);

const emit = defineEmits<{
  /** 拖动 / 键盘调整过程中持续派发，供界面实时预览 */
  'update:modelValue': [number];
  /** 一次调整结束（松开指针）时派发，供持久化保存 */
  change: [number];
}>();

const track = ref<HTMLElement | null>(null);
const dragging = ref(false);
/** 拖动过程中的临时值：父组件可只在 change 时提交，避免拖动中频繁回写设置 */
const dragValue = ref<number | null>(null);

const displayValue = computed(() => dragValue.value ?? props.modelValue);

const percent = computed(() => {
  const span = props.max - props.min;
  return span <= 0 ? 0 : ((displayValue.value - props.min) / span) * 100;
});

function clamp(value: number): number {
  return Math.min(props.max, Math.max(props.min, value));
}

function valueFromPointer(event: PointerEvent): number {
  const rect = track.value!.getBoundingClientRect();
  const ratio = rect.width > 0 ? (event.clientX - rect.left) / rect.width : 0;
  const raw = props.min + Math.min(1, Math.max(0, ratio)) * (props.max - props.min);
  return clamp(Math.round(raw / props.step) * props.step);
}

function commit(value: number): void {
  if (value !== props.modelValue) emit('update:modelValue', value);
  emit('change', value);
}

function onPointerDown(event: PointerEvent): void {
  event.preventDefault();
  const root = event.currentTarget as HTMLElement;
  dragging.value = true;
  dragValue.value = valueFromPointer(event);
  root.focus();
  // 捕获指针：拖出组件范围仍持续跟踪
  root.setPointerCapture(event.pointerId);
}

function onPointerMove(event: PointerEvent): void {
  if (!dragging.value) return;
  dragValue.value = valueFromPointer(event);
}

function onPointerEnd(event: PointerEvent): void {
  if (!dragging.value) return;
  dragging.value = false;
  const value = valueFromPointer(event);
  dragValue.value = null;
  commit(value);
}

function onKeydown(event: KeyboardEvent): void {
  const stride = props.step * (event.shiftKey ? 10 : 1);
  let value = displayValue.value;
  switch (event.key) {
    case 'ArrowLeft':
    case 'ArrowDown':
      value -= stride;
      break;
    case 'ArrowRight':
    case 'ArrowUp':
      value += stride;
      break;
    case 'Home':
      value = props.min;
      break;
    case 'End':
      value = props.max;
      break;
    default:
      return;
  }
  event.preventDefault();
  const next = clamp(value);
  emit('update:modelValue', next);
  emit('change', next);
}
</script>

<template>
  <div
    class="slider"
    :class="{ 'is-dragging': dragging }"
    role="slider"
    tabindex="0"
    :aria-label="label"
    :aria-valuemin="min"
    :aria-valuemax="max"
    :aria-valuenow="displayValue"
    @pointerdown="onPointerDown"
    @pointermove="onPointerMove"
    @pointerup="onPointerEnd"
    @pointercancel="onPointerEnd"
    @keydown="onKeydown"
  >
    <div ref="track" class="slider__track">
      <div class="slider__fill" :style="{ width: `${percent}%` }" aria-hidden="true"></div>
    </div>
    <div class="slider__thumb" :style="{ left: `${percent}%` }" aria-hidden="true"></div>
  </div>
</template>

<style scoped>
.slider {
  position: relative;
  flex: 1;
  min-width: 0;
  height: 28px;
  display: flex;
  align-items: center;
  cursor: pointer;
  touch-action: none;
  -webkit-tap-highlight-color: transparent;
}

.slider:focus-visible {
  outline-offset: 6px;
  border-radius: 8px;
}

.slider__track {
  width: 100%;
  height: 6px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.09);
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.4);
  overflow: hidden;
}

.slider__fill {
  height: 100%;
  border-radius: inherit;
  background: linear-gradient(135deg, #b96cf5, #9333ea);
}

.slider__thumb {
  position: absolute;
  top: 50%;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  transform: translate(-50%, -50%);
  background: radial-gradient(circle at 35% 30%, #f3e3fd 0%, #dcb6f8 45%, #b06ef0 100%);
  box-shadow:
    0 0 0 3px rgba(168, 85, 247, 0.16),
    0 0 12px rgba(168, 85, 247, 0.55),
    0 2px 6px rgba(0, 0, 0, 0.4);
  transition: transform 0.15s ease, box-shadow 0.15s ease;
  pointer-events: none;
}

.slider:hover .slider__thumb {
  transform: translate(-50%, -50%) scale(1.1);
  box-shadow:
    0 0 0 4px rgba(168, 85, 247, 0.2),
    0 0 16px rgba(168, 85, 247, 0.65),
    0 2px 6px rgba(0, 0, 0, 0.4);
}

/* 拖动 / 键盘聚焦时滑块放大、光晕增强，呼应主题紫色光体质感 */
.slider.is-dragging .slider__thumb,
.slider:focus-visible .slider__thumb {
  transform: translate(-50%, -50%) scale(1.18);
  box-shadow:
    0 0 0 6px rgba(168, 85, 247, 0.22),
    0 0 20px rgba(168, 85, 247, 0.8),
    0 2px 8px rgba(0, 0, 0, 0.45);
}

@media (prefers-reduced-motion: reduce) {
  .slider__thumb {
    transition: none;
  }
}
</style>
