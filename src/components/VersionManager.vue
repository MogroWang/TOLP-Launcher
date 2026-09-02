<script setup lang="ts">
import { BUILTIN_GAME_VERSIONS } from '../lib/tauri';
import type { GameStatus, Settings } from '../lib/tauri';

const props = defineProps<{
  settings: Settings;
  status: GameStatus;
}>();

const emit = defineEmits<{
  /** 将某个内置版本设为启动版本 */
  select: [versionId: string];
}>();

const builtinVersion = BUILTIN_GAME_VERSIONS[0];

function onCardClick(): void {
  if (props.settings.versionId !== builtinVersion.id) {
    emit('select', builtinVersion.id);
  }
}
</script>

<template>
  <section class="vm">
    <header class="vm__head">
      <h2>版本管理</h2>
      <p>安装与管理光点之旅的各个版本。</p>
    </header>

    <!-- 占位：在线版本分发接入前仅列出唯一的内置版本 -->
    <button
      class="vm__card"
      type="button"
      :class="{ 'is-current': settings.versionId === builtinVersion.id }"
      :disabled="settings.versionId === builtinVersion.id"
      @click="onCardClick"
    >
      <span class="vm__orb" aria-hidden="true"></span>
      <span class="vm__info">
        <span class="vm__name">{{ builtinVersion.label }}</span>
        <span class="vm__desc">内部开发版本 · 通过启动器直接运行</span>
      </span>
      <span class="vm__badges">
        <span class="vm__badge" :class="status.found ? 'is-ok' : 'is-missing'">
          {{ status.found ? '已就绪' : '未安装' }}
        </span>
        <span v-if="settings.versionId === builtinVersion.id" class="vm__badge vm__badge--current">当前版本</span>
        <span v-else class="vm__badge vm__badge--action">设为当前</span>
      </span>
    </button>

    <p class="vm__note">更多历史版本与在线下载即将推出，届时可在此安装与切换。</p>
  </section>
</template>

<style scoped>
.vm {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 34px 44px 20px;
  overflow-y: auto;
}

.vm__head {
  width: min(640px, 100%);
  margin-bottom: 22px;
}

.vm__head h2 {
  font-size: 21px;
  font-weight: 500;
  letter-spacing: 0.08em;
}

.vm__head p {
  margin-top: 7px;
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--ink-3);
}

.vm__card {
  width: min(640px, 100%);
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 17px 20px;
  border-radius: 18px;
  text-align: left;
  background: rgba(255, 255, 255, 0.045);
  border: 1px solid rgba(255, 255, 255, 0.08);
  transition: background-color 0.16s ease, border-color 0.16s ease, transform 0.1s ease;
}

.vm__card:disabled {
  cursor: default;
  border-color: rgba(168, 85, 247, 0.32);
  background: rgba(168, 85, 247, 0.07);
}

.vm__card:not(:disabled):hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.14);
}

.vm__card:not(:disabled):active {
  transform: scale(0.99);
}

.vm__orb {
  flex: none;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: radial-gradient(circle at 36% 30%, #efd9fc 0%, #cb8df4 40%, #a855f7 78%, #8a3ed2 100%);
  box-shadow: 0 0 16px rgba(168, 85, 247, 0.55);
}

.vm__info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.vm__name {
  font-size: 14.5px;
  letter-spacing: 0.05em;
  color: #fff;
}

.vm__desc {
  font-size: 12px;
  letter-spacing: 0.03em;
  color: var(--ink-3);
}

.vm__badges {
  flex: none;
  display: flex;
  align-items: center;
  gap: 8px;
}

.vm__badge {
  display: inline-flex;
  align-items: center;
  padding: 3px 11px;
  border-radius: 999px;
  font-size: 11px;
  letter-spacing: 0.06em;
  white-space: nowrap;
}

.vm__badge.is-ok {
  color: #86efac;
  background: rgba(74, 222, 128, 0.1);
}

.vm__badge.is-missing {
  color: #fda4af;
  background: rgba(248, 113, 113, 0.1);
}

.vm__badge--current {
  color: var(--accent-soft);
  background: rgba(168, 85, 247, 0.16);
}

.vm__badge--action {
  color: var(--ink-2);
  background: rgba(255, 255, 255, 0.08);
}

.vm__note {
  width: min(640px, 100%);
  margin-top: 16px;
  font-size: 12px;
  line-height: 1.75;
  color: rgba(255, 255, 255, 0.38);
}
</style>
