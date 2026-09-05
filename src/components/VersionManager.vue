<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { BUILTIN_GAME_VERSIONS, listVersions } from '../lib/tauri';
import type { GameStatus, Settings, VersionEntry, VersionScan } from '../lib/tauri';
import { t } from '../lib/i18n';

const props = defineProps<{
  settings: Settings;
  status: GameStatus;
}>();

const emit = defineEmits<{
  /** 将某个版本设为启动版本 */
  select: [versionId: string];
  change: [Settings];
}>();

const builtinVersion = BUILTIN_GAME_VERSIONS[0];

/** 内置版本名：产品名随界面语言翻译，版本号保持原样 */
const builtinLabel = computed(() => `${t('game.name')} ${builtinVersion.id} DEV`);

const scan = ref<VersionScan | null>(null);

async function refresh(): Promise<void> {
  scan.value = await listVersions();
}

onMounted(refresh);
// 自定义位置变化后重新扫描，让卡片即时反映识别结果
watch(
  () => props.settings.customVersionDir,
  () => void refresh(),
);

/** 数据文件夹 versions/ 下识别到的内置版本 */
const builtinAuto = computed(() => scan.value?.versions.find((v) => v.id === builtinVersion.id) ?? null);

const builtinReady = computed(() => {
  if (builtinAuto.value?.found) return true;
  if (scan.value?.custom?.found) return true;
  // 内置版本经默认 game 文件夹回落解析成功
  return props.settings.versionId === builtinVersion.id && props.status.found;
});

/** 内置版本的展示信息：优先自动识别，其次自定义位置，最后默认回落目录 */
const builtinEntry = computed<VersionEntry | null>(() => {
  if (builtinAuto.value) return builtinAuto.value;
  if (scan.value?.custom) return scan.value.custom;
  if (props.settings.versionId === builtinVersion.id && props.status.found && props.status.dir) {
    return {
      id: builtinVersion.id,
      dir: props.status.dir,
      found: true,
      reason: null,
      name: null,
      version: props.status.version,
      official: props.status.official,
    };
  }
  return null;
});

const builtinSource = computed(() => {
  if (builtinAuto.value?.found) return t('vm.srcDataFolder');
  if (scan.value?.custom?.found) return t('vm.srcCustom');
  if (props.settings.versionId === builtinVersion.id && props.status.found) return t('vm.srcDefaultGame');
  return null;
});

/** 数据文件夹中识别到的其他版本 */
const otherVersions = computed(() => scan.value?.versions.filter((v) => v.id !== builtinVersion.id) ?? []);

/** 是否展示内置版本卡片下的自定义位置操作区 */
const showBuiltinActions = computed(() => !builtinAuto.value?.found || !!scan.value?.custom);

async function chooseCustomLocation(): Promise<void> {
  const picked = await openDialog({
    directory: true,
    multiple: false,
    title: t('dialog.pickVersionDir'),
  });
  if (typeof picked === 'string') {
    emit('change', { ...props.settings, versionId: builtinVersion.id, customVersionDir: picked });
  }
}

function clearCustomLocation(): void {
  emit('change', { ...props.settings, customVersionDir: null });
}
</script>

<template>
  <section class="vm">
    <header class="vm__head">
      <div class="vm__head-row">
        <div>
          <h2>{{ t('vm.title') }}</h2>
          <p>{{ t('vm.subtitle') }}</p>
        </div>
        <button class="vm__btn vm__btn--ghost" type="button" @click="refresh">{{ t('vm.rescan') }}</button>
      </div>
      <p v-if="scan?.dataDir" class="vm__dir">
        {{ t('vm.dataDir') }}<code :title="scan.versionsDir">{{ scan.dataDir }}</code>
      </p>
    </header>

    <!-- 内置版本：优先在数据文件夹 versions/ 中识别，未识别到时可指定自定义位置 -->
    <div class="vm__card" :class="{ 'is-current': settings.versionId === builtinVersion.id }">
      <span class="vm__orb" aria-hidden="true"></span>
      <span class="vm__info">
        <span class="vm__name">{{ builtinLabel }}</span>
        <span class="vm__desc">
          {{ t('vm.devBuild') }}
          <template v-if="builtinEntry?.version"> · v{{ builtinEntry.version }}</template>
          <template v-if="builtinSource"> · {{ t('vm.from') }}{{ builtinSource }}</template>
        </span>
        <span v-if="builtinEntry" class="vm__path" :title="builtinEntry.dir">{{ builtinEntry.dir }}</span>
      </span>
      <span class="vm__badges">
        <span v-if="builtinEntry?.official" class="vm__badge vm__badge--official">{{ t('vm.official') }}</span>
        <span class="vm__badge" :class="builtinReady ? 'is-ok' : 'is-missing'">
          {{ builtinReady ? t('vm.ready') : t('vm.unrecognized') }}
        </span>
        <button
          v-if="settings.versionId !== builtinVersion.id"
          class="vm__badge vm__badge--action"
          type="button"
          @click="emit('select', builtinVersion.id)"
        >
          {{ t('vm.setCurrent') }}
        </button>
        <span v-else class="vm__badge vm__badge--current">{{ t('vm.current') }}</span>
      </span>
    </div>

    <div v-if="showBuiltinActions" class="vm__actions">
      <button class="vm__btn" type="button" @click="chooseCustomLocation">{{ t('vm.customLocation') }}</button>
      <button v-if="scan?.custom" class="vm__btn vm__btn--ghost" type="button" @click="clearCustomLocation">
        {{ t('vm.clearCustom') }}
      </button>
      <span v-if="scan?.custom && !scan.custom.found" class="vm__action-hint">{{ t('vm.missingIndexHint') }}</span>
    </div>

    <!-- 数据文件夹中识别到的其他版本 -->
    <div
      v-for="v in otherVersions"
      :key="v.id"
      class="vm__card vm__card--other"
      :class="{ 'is-current': settings.versionId === v.id }"
    >
      <span class="vm__orb" aria-hidden="true"></span>
      <span class="vm__info">
        <span class="vm__name">{{ v.name ?? v.id }}</span>
        <span class="vm__desc">
          <template v-if="v.version">v{{ v.version }}</template>
          <template v-else>{{ t('vm.noVersionNum') }}</template>
          <template v-if="v.official"> · {{ t('vm.official') }}</template>
        </span>
        <span class="vm__path" :title="v.dir">{{ v.dir }}</span>
      </span>
      <span class="vm__badges">
        <span class="vm__badge" :class="v.found ? 'is-ok' : 'is-missing'">
          {{ v.found ? t('vm.ready') : t('vm.notReady') }}
        </span>
        <button
          v-if="settings.versionId !== v.id && v.found"
          class="vm__badge vm__badge--action"
          type="button"
          @click="emit('select', v.id)"
        >
          {{ t('vm.setCurrent') }}
        </button>
        <span v-else-if="settings.versionId === v.id" class="vm__badge vm__badge--current">{{ t('vm.current') }}</span>
      </span>
    </div>

    <p class="vm__note">
      {{
        scan === null
          ? t('vm.scanning')
          : scan.versions.length === 0 && !scan.custom
            ? t('vm.emptyTip')
            : t('vm.autoTip')
      }}
    </p>
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

.vm__head-row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
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

.vm__dir {
  margin-top: 10px;
  font-size: 11.5px;
  letter-spacing: 0.03em;
  color: var(--ink-4);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.vm__dir code {
  font-family: inherit;
  font-size: 11px;
  color: var(--accent-soft);
  background: rgba(168, 85, 247, 0.1);
  padding: 1px 7px;
  border-radius: 6px;
  user-select: text;
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
  transition: border-color 0.16s ease, background-color 0.16s ease;
}

.vm__card.is-current {
  border-color: rgba(168, 85, 247, 0.32);
  background: rgba(168, 85, 247, 0.07);
}

.vm__card--other {
  margin-top: 12px;
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

.vm__path {
  font-size: 11px;
  letter-spacing: 0.02em;
  color: var(--ink-4);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  user-select: text;
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

.vm__badge--official {
  color: var(--accent-soft);
  border: 1px solid rgba(168, 85, 247, 0.38);
}

.vm__badge--current {
  color: var(--accent-soft);
  background: rgba(168, 85, 247, 0.16);
}

.vm__badge--action {
  color: var(--ink-2);
  background: rgba(255, 255, 255, 0.08);
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.vm__badge--action:hover {
  color: #fff;
  background: rgba(168, 85, 247, 0.2);
}

.vm__badge--action:active {
  transform: scale(0.96);
}

.vm__actions {
  width: min(640px, 100%);
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 12px;
}

.vm__action-hint {
  font-size: 11.5px;
  letter-spacing: 0.03em;
  color: #fda4af;
}

.vm__btn {
  padding: 6px 15px;
  border-radius: 999px;
  font-size: 12px;
  letter-spacing: 0.05em;
  color: var(--accent-soft);
  background: rgba(168, 85, 247, 0.12);
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.vm__btn:hover {
  background: rgba(168, 85, 247, 0.22);
}

.vm__btn:active {
  transform: scale(0.97);
}

.vm__btn--ghost {
  color: var(--ink-2);
  background: rgba(255, 255, 255, 0.07);
}

.vm__btn--ghost:hover {
  color: #fff;
  background: rgba(255, 255, 255, 0.13);
}

.vm__note {
  width: min(640px, 100%);
  margin-top: 16px;
  font-size: 12px;
  line-height: 1.75;
  color: rgba(255, 255, 255, 0.38);
}
</style>
