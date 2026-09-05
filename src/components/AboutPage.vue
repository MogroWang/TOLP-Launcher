<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getDataDir } from '../lib/tauri';
import { t } from '../lib/i18n';
import tolpLogo from '../assets/tolp-logo.png';

defineProps<{
  /** 启动器自身版本（tauri app.getVersion） */
  version: string;
}>();

const dataDir = ref('');

onMounted(async () => {
  dataDir.value = await getDataDir();
});
</script>

<template>
  <section class="ab">
    <header class="ab__head">
      <h2>{{ t('ab.title') }}</h2>
      <p>{{ t('ab.subtitle') }}</p>
    </header>

    <div class="ab__group">
      <div class="ab__card">
        <div class="ab__hero">
          <img class="ab__logo" :src="tolpLogo" alt="光点之旅" draggable="false" />
          <div class="ab__hero-text">
            <strong>TOLP Launcher</strong>
            <span>Tour of Light Point Launcher</span>
          </div>
          <span class="ab__ver">{{ version ? `V${version}` : '—' }}</span>
        </div>
      </div>
    </div>

    <div class="ab__group">
      <h3 class="ab__label">{{ t('ab.info') }}</h3>
      <div class="ab__card">
        <div class="ab__row">
          <span class="ab__row-title">{{ t('ab.game') }}</span>
          <span class="ab__row-value">Tour of Light Point</span>
        </div>
        <div class="ab__row">
          <span class="ab__row-title">{{ t('ab.developer') }}</span>
          <span class="ab__row-value">MogroWang Studio</span>
        </div>
        <div class="ab__row ab__row--stacked">
          <span class="ab__row-title">{{ t('ab.dataFolder') }}</span>
          <span class="ab__dir" :title="dataDir">{{ dataDir || '—' }}</span>
        </div>
      </div>
      <p class="ab__footnote">{{ t('ab.footnote') }}</p>
    </div>
  </section>
</template>

<style scoped>
.ab {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 34px 44px 20px;
  overflow-y: auto;
}

.ab__head {
  width: min(640px, 100%);
  margin-bottom: 22px;
}

.ab__head h2 {
  font-size: 21px;
  font-weight: 500;
  letter-spacing: 0.08em;
}

.ab__head p {
  margin-top: 7px;
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--ink-3);
}

.ab__group {
  width: min(640px, 100%);
}

.ab__group + .ab__group {
  margin-top: 24px;
}

.ab__label {
  font-size: 11.5px;
  font-weight: 400;
  letter-spacing: 0.18em;
  color: var(--ink-3);
  margin-bottom: 10px;
}

.ab__card {
  padding: 6px 20px;
  border-radius: 18px;
  background: rgba(255, 255, 255, 0.045);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.ab__hero {
  display: flex;
  align-items: center;
  gap: 18px;
  padding: 18px 0;
}

.ab__logo {
  flex: none;
  width: 72px;
  height: auto;
  user-select: none;
  pointer-events: none;
  filter: drop-shadow(0 0 18px rgba(168, 85, 247, 0.28));
}

.ab__hero-text {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.ab__hero-text strong {
  font-size: 17px;
  font-weight: 500;
  letter-spacing: 0.06em;
  color: #fff;
}

.ab__hero-text span {
  font-size: 11.5px;
  letter-spacing: 0.08em;
  color: var(--ink-4);
}

.ab__ver {
  flex: none;
  padding: 4px 12px;
  border-radius: 999px;
  font-size: 12px;
  letter-spacing: 0.08em;
  color: var(--accent-soft);
  background: rgba(168, 85, 247, 0.12);
  font-variant-numeric: tabular-nums;
}

.ab__row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
  padding: 13px 0;
}

.ab__row + .ab__row {
  border-top: 1px solid rgba(255, 255, 255, 0.06);
}

.ab__row--stacked {
  flex-direction: column;
  align-items: stretch;
  gap: 8px;
}

.ab__row-title {
  font-size: 13.5px;
  letter-spacing: 0.05em;
  color: #fff;
}

.ab__row-value {
  font-size: 13px;
  letter-spacing: 0.04em;
  color: var(--ink-2);
  text-align: right;
}

.ab__dir {
  padding: 10px 14px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.08);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12.5px;
  color: var(--ink-2);
  user-select: text;
}

.ab__footnote {
  margin-top: 12px;
  font-size: 12px;
  line-height: 1.75;
  color: rgba(255, 255, 255, 0.38);
}
</style>
