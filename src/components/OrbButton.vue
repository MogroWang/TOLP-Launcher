<script setup lang="ts">
withDefaults(
  defineProps<{
    size?: number;
    tone?: 'purple' | 'white';
    label?: string;
    breathing?: boolean;
    loading?: boolean;
    disabled?: boolean;
    /** 展开态（如设置抽屉打开时的开关球） */
    active?: boolean;
  }>(),
  {
    size: 64,
    tone: 'purple',
    label: '',
    breathing: false,
    loading: false,
    disabled: false,
    active: false,
  },
);
</script>

<template>
  <div
    class="orb"
    :class="[`orb--${tone}`, { 'orb--breathing': breathing, 'orb--loading': loading, 'orb--disabled': disabled, 'orb--active': active }]"
    :style="{ '--orb-size': `${size}px` }"
  >
    <button
      class="orb__button"
      type="button"
      :disabled="disabled || loading"
      :aria-label="label || undefined"
      :aria-busy="loading || undefined"
      :aria-expanded="active"
    >
      <span class="orb__body" aria-hidden="true">
        <span class="orb__halo"></span>
        <slot />
      </span>
    </button>
    <span v-if="label" class="orb__label" aria-hidden="true">{{ label }}</span>
  </div>
</template>

<style scoped>
.orb {
  display: inline-flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}

.orb__button {
  position: relative;
  width: var(--orb-size);
  height: var(--orb-size);
  border-radius: 50%;
  transition: transform 0.18s cubic-bezier(0.2, 0.7, 0.3, 1), filter 0.18s ease;
  will-change: transform;
  -webkit-tap-highlight-color: transparent;
}

.orb__button:hover:not(:disabled) {
  transform: scale(1.045);
  filter: brightness(1.06);
}

/* 按压反馈在按下瞬间出现，而不是松开时 */
.orb__button:active:not(:disabled) {
  transform: scale(0.955);
  transition-duration: 0.09s;
}

.orb__body {
  position: absolute;
  inset: 0;
  border-radius: 50%;
}

.orb--purple .orb__body {
  background: radial-gradient(circle at 36% 30%, #efd9fc 0%, #cb8df4 38%, #a855f7 70%, #8a3ed2 100%);
  box-shadow:
    0 0 28px rgba(168, 85, 247, 0.55),
    0 0 88px rgba(168, 85, 247, 0.26),
    inset 0 -10px 26px rgba(96, 22, 168, 0.42),
    inset 0 8px 16px rgba(255, 255, 255, 0.28);
}

.orb--white .orb__body {
  background: radial-gradient(circle at 36% 30%, #ffffff 0%, #ececec 48%, #c9c9c9 100%);
  box-shadow:
    0 0 22px rgba(255, 255, 255, 0.24),
    0 0 64px rgba(255, 255, 255, 0.12),
    inset 0 -8px 18px rgba(0, 0, 0, 0.18),
    inset 0 6px 12px rgba(255, 255, 255, 0.9);
}

.orb__body :deep(svg) {
  position: relative;
  width: 44%;
  height: 44%;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.18));
}

.orb--white .orb__body :deep(svg) {
  filter: drop-shadow(0 2px 3px rgba(0, 0, 0, 0.28));
}

/* 环境光晕：悬停增强；呼吸态为游戏里“光点”的静息脉动 */
.orb__halo {
  position: absolute;
  inset: -22%;
  border-radius: 50%;
  background: radial-gradient(circle, var(--halo-color, rgba(168, 85, 247, 0.3)) 0%, transparent 68%);
  opacity: 0;
  transition: opacity 0.22s ease;
  pointer-events: none;
}

.orb--white {
  --halo-color: rgba(255, 255, 255, 0.22);
}

.orb__button:hover:not(:disabled) .orb__halo {
  opacity: 1;
}

.orb--breathing .orb__halo {
  animation: orb-breathe 3.8s ease-in-out infinite;
}

@keyframes orb-breathe {
  0%,
  100% {
    opacity: 0.4;
    transform: scale(1);
  }
  50% {
    opacity: 1;
    transform: scale(1.07);
  }
}

.orb--loading .orb__button::before {
  content: '';
  position: absolute;
  inset: -9px;
  border-radius: 50%;
  border: 2px solid rgba(255, 255, 255, 0.2);
  border-top-color: rgba(255, 255, 255, 0.9);
  animation: orb-spin 0.9s linear infinite;
}

@keyframes orb-spin {
  to {
    transform: rotate(360deg);
  }
}

.orb--disabled {
  opacity: 0.45;
}

/* 展开态：开关球泛起紫色光环，与抽屉状态呼应 */
.orb--active .orb__body {
  box-shadow:
    0 0 26px rgba(168, 85, 247, 0.55),
    0 0 72px rgba(168, 85, 247, 0.28),
    inset 0 -8px 18px rgba(0, 0, 0, 0.18),
    inset 0 6px 12px rgba(255, 255, 255, 0.9);
}

.orb--active .orb__button {
  transform: scale(1.03);
}

.orb__label {
  font-size: var(--orb-label-size, 14px);
  letter-spacing: 0.1em;
  color: var(--ink-2);
}
</style>
