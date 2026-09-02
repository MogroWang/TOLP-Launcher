<script setup lang="ts">
import type { GameStatus } from '../lib/tauri';
import OrbButton from './OrbButton.vue';

defineProps<{
  status: GameStatus;
  launching: boolean;
  error: string | null;
}>();

defineEmits<{
  launch: [];
  fix: [];
}>();
</script>

<template>
  <section class="hero">
    <header class="hero__brand">
      <h1>光点之旅</h1>
      <p class="hero__sub">TOUR OF LIGHT POINT</p>
    </header>

    <OrbButton
      class="hero__play"
      :size="148"
      tone="purple"
      breathing
      :label="status.found ? '开始游戏' : '未找到游戏文件'"
      :loading="launching"
      :disabled="!status.found"
      @click="$emit('launch')"
    >
      <svg viewBox="0 0 48 48" aria-hidden="true">
        <path
          d="M17 14.8 Q17 11.4 20 13 L39 23.2 Q42 24 39 24.8 L20 35 Q17 36.6 17 33.2 Z"
          fill="#fff"
        />
      </svg>
    </OrbButton>

    <div class="hero__meta" aria-live="polite">
      <template v-if="launching">
        <span>正在启动…</span>
      </template>
      <template v-else-if="status.found">
        <span class="hero__dot hero__dot--ok" aria-hidden="true"></span>
        <span>游戏已就绪</span>
        <span class="hero__path" :title="status.dir ?? ''">{{ status.dir }}</span>
      </template>
      <template v-else>
        <span class="hero__dot hero__dot--missing" aria-hidden="true"></span>
        <span>{{ status.reason ?? '尚未找到游戏文件' }}</span>
        <button class="hero__fix" type="button" @click="$emit('fix')">去选择游戏目录</button>
      </template>
    </div>

    <p v-if="error" class="hero__error">{{ error }}</p>
  </section>
</template>

<style scoped>
.hero {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding-bottom: 5vh;
}

/* 光点的环境光，让黑色不再空洞 —— 与游戏菜单一致 */
.hero::before {
  content: '';
  position: absolute;
  width: 660px;
  height: 660px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(168, 85, 247, 0.1) 0%, transparent 62%);
  top: 50%;
  left: 50%;
  transform: translate(-50%, -54%);
  pointer-events: none;
}

.hero__brand {
  text-align: center;
  margin-bottom: 54px;
}

.hero__brand h1 {
  font-size: 40px;
  font-weight: 500;
  letter-spacing: 0.12em;
  text-indent: 0.12em;
  line-height: 1;
}

.hero__sub {
  margin-top: 12px;
  font-size: 11px;
  letter-spacing: 0.42em;
  text-indent: 0.42em;
  color: var(--ink-4);
}

.hero__play {
  position: relative;
  margin-bottom: 28px;
}

.hero__meta {
  display: flex;
  align-items: center;
  gap: 10px;
  max-width: 72%;
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--ink-2);
}

.hero__dot {
  flex: none;
  width: 7px;
  height: 7px;
  border-radius: 50%;
}

.hero__dot--ok {
  background: var(--ok);
  box-shadow: 0 0 9px rgba(74, 222, 128, 0.8);
}

.hero__dot--missing {
  background: var(--danger);
  box-shadow: 0 0 9px rgba(255, 138, 128, 0.8);
}

.hero__path {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 340px;
  color: var(--ink-3);
}

.hero__fix {
  flex: none;
  margin-left: 2px;
  padding-bottom: 1px;
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--accent-soft);
  border-bottom: 1px solid rgba(216, 180, 254, 0.4);
  transition: border-color 0.15s ease, color 0.15s ease;
}

.hero__fix:hover {
  color: #e9d2ff;
  border-color: rgba(233, 210, 255, 0.9);
}

.hero__error {
  position: absolute;
  bottom: 13vh;
  max-width: 72%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12.5px;
  color: var(--danger);
}
</style>
