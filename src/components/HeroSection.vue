<script setup lang="ts">
import { computed } from 'vue';
import type { GameStatus } from '../lib/tauri';
import OrbButton from './OrbButton.vue';

const props = defineProps<{
  status: GameStatus;
  launching: boolean;
  running: boolean;
  error: string | null;
}>();

defineEmits<{
  launch: [];
  requestSettings: [];
}>();

/** 运行中点击主球会把已有游戏窗口带回前台（后端复用窗口逻辑） */
const playLabel = computed(() => {
  if (props.running) return '正在运行';
  return props.status.found ? '开始游戏' : '未找到游戏文件';
});
</script>

<template>
  <section class="hero">
    <header class="hero__brand">
      <h1>光点之旅</h1>
      <p class="hero__sub">TOUR OF LIGHT POINT</p>
    </header>

    <!-- 构图取自游戏菜单设计稿：次级球伴随主球、底部对齐 -->
    <div class="hero__orbs">
      <OrbButton
        class="hero__settings"
        :size="56"
        tone="white"
        label="启动设置"
        @click="$emit('requestSettings')"
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

      <OrbButton
        class="hero__play"
        :size="148"
        tone="purple"
        breathing
        :label="playLabel"
        :loading="launching"
        :disabled="!status.found && !running"
        @click="$emit('launch')"
      >
        <svg viewBox="0 0 48 48" aria-hidden="true">
          <path
            d="M17 14.8 Q17 11.4 20 13 L39 23.2 Q42 24 39 24.8 L20 35 Q17 36.6 17 33.2 Z"
            fill="#fff"
          />
        </svg>
      </OrbButton>
    </div>

    <div class="hero__meta" aria-live="polite">
      <template v-if="launching">
        <span>正在启动…</span>
      </template>
      <template v-else-if="running">
        <span class="hero__dot hero__dot--ok" aria-hidden="true"></span>
        <span>游戏正在运行</span>
        <span class="hero__path" :title="status.dir ?? ''">{{ status.dir }}</span>
      </template>
      <template v-else-if="status.found">
        <span class="hero__dot hero__dot--ok" aria-hidden="true"></span>
        <span>游戏已就绪</span>
        <span class="hero__path" :title="status.dir ?? ''">{{ status.dir }}</span>
      </template>
      <template v-else>
        <span class="hero__dot hero__dot--missing" aria-hidden="true"></span>
        <span>{{ status.reason ?? '尚未找到游戏文件' }}</span>
        <button class="hero__fix" type="button" @click="$emit('requestSettings')">去选择游戏目录</button>
      </template>
    </div>

    <p v-if="error" class="hero__error">{{ error }}</p>
  </section>
</template>

<style scoped>
.hero {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: clamp(26px, 5.5vh, 54px);
  padding: 16px 24px;
  position: relative;
}

/* 光点的环境光，让黑色不再空洞 —— 与游戏菜单一致 */
.hero::before {
  content: '';
  position: absolute;
  width: min(72vmin, 700px);
  height: min(72vmin, 700px);
  border-radius: 50%;
  background: radial-gradient(circle, rgba(168, 85, 247, 0.1) 0%, transparent 62%);
  top: 50%;
  left: 50%;
  transform: translate(-50%, -54%);
  pointer-events: none;
}

.hero__brand {
  position: relative;
  text-align: center;
}

.hero__brand h1 {
  font-size: clamp(30px, 4.6vmin, 42px);
  font-weight: 500;
  letter-spacing: 0.12em;
  text-indent: 0.12em;
  line-height: 1;
}

.hero__sub {
  margin-top: clamp(8px, 1.6vh, 14px);
  font-size: 11px;
  letter-spacing: 0.42em;
  text-indent: 0.42em;
  color: var(--ink-4);
}

.hero__orbs {
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: clamp(40px, 6vmin, 64px);
}

.hero__settings {
  --orb-label-size: 12px;
}

.hero__play {
  margin-bottom: 34px;
}

.hero__meta {
  position: relative;
  display: flex;
  align-items: center;
  gap: 10px;
  max-width: min(72%, 640px);
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
  position: relative;
  max-width: min(72%, 640px);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12.5px;
  color: var(--danger);
  margin-top: -14px;
}
</style>
