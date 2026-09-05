<script setup lang="ts">
export type PageId = 'launch' | 'versions' | 'settings' | 'about';

const props = defineProps<{
  page: PageId;
  /** 启动器自身版本（tauri app.getVersion），显示在侧边栏底部 */
  version: string;
}>();

const emit = defineEmits<{
  update: [PageId];
}>();

const items: Array<{ id: PageId; label: string }> = [
  { id: 'launch', label: '快速启动' },
  { id: 'versions', label: '版本管理' },
  { id: 'settings', label: '启动器设置' },
  { id: 'about', label: '关于' },
];
</script>

<template>
  <nav class="sidenav" aria-label="启动器导航">
    <button
      v-for="item in items"
      :key="item.id"
      class="sidenav__item"
      :class="{ 'is-active': props.page === item.id }"
      type="button"
      :aria-current="props.page === item.id ? 'page' : undefined"
      @click="emit('update', item.id)"
    >
      <!-- 快速启动：圆角播放三角 -->
      <svg v-if="item.id === 'launch'" viewBox="0 0 20 20" aria-hidden="true">
        <path
          d="M6.2 4.6 Q6.2 2.9 7.7 3.7 L16 9.2 Q17.4 10 16 10.8 L7.7 16.3 Q6.2 17.1 6.2 15.4 Z"
          fill="currentColor"
        />
      </svg>
      <!-- 版本管理：层叠圆角方块 -->
      <svg v-else-if="item.id === 'versions'" viewBox="0 0 20 20" fill="none" aria-hidden="true">
        <rect x="3.2" y="3.2" width="10.4" height="10.4" rx="3" stroke="currentColor" stroke-width="1.7" />
        <path d="M16.9 7.2v5.3a4.4 4.4 0 0 1-4.4 4.4H7.2" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" />
      </svg>
      <!-- 启动器设置：滑块 -->
      <svg v-else-if="item.id === 'settings'" viewBox="0 0 20 20" fill="none" aria-hidden="true">
        <g stroke="currentColor" stroke-width="1.7" stroke-linecap="round">
          <path d="M3.4 5.6h7" />
          <path d="M14.9 5.6h1.7" />
          <path d="M3.4 10h2.2" />
          <path d="M10 10h6.6" />
          <path d="M3.4 14.4h5.6" />
          <path d="M13.4 14.4h3.2" />
        </g>
        <g fill="currentColor">
          <circle cx="12.9" cy="5.6" r="1.9" />
          <circle cx="7.8" cy="10" r="1.9" />
          <circle cx="11.3" cy="14.4" r="1.9" />
        </g>
      </svg>
      <!-- 关于：信息圆圈 -->
      <svg v-else viewBox="0 0 20 20" fill="none" aria-hidden="true">
        <circle cx="10" cy="10" r="6.9" stroke="currentColor" stroke-width="1.7" />
        <circle cx="10" cy="6.9" r="1.05" fill="currentColor" />
        <path d="M10 9.3v4.1" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" />
      </svg>
      <span>{{ item.label }}</span>
    </button>

    <div class="sidenav__footer">
      <span v-if="version" class="sidenav__version">V{{ version }}</span>
    </div>
  </nav>
</template>

<style scoped>
.sidenav {
  width: 196px;
  flex: none;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 18px 12px;
  background: rgba(255, 255, 255, 0.028);
  border-right: 1px solid rgba(255, 255, 255, 0.06);
}

.sidenav__item {
  display: flex;
  align-items: center;
  gap: 11px;
  height: 42px;
  padding: 0 14px;
  border-radius: 12px;
  font-size: 13.5px;
  letter-spacing: 0.06em;
  color: var(--ink-2);
  transition: background-color 0.16s ease, color 0.16s ease, transform 0.1s ease;
}

.sidenav__item svg {
  width: 17px;
  height: 17px;
  flex: none;
  opacity: 0.75;
  transition: opacity 0.16s ease;
}

.sidenav__item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: rgba(255, 255, 255, 0.85);
}

.sidenav__item:active {
  transform: scale(0.98);
}

.sidenav__item.is-active {
  background: rgba(168, 85, 247, 0.16);
  color: #fff;
}

.sidenav__item.is-active svg {
  opacity: 1;
  color: var(--accent-soft);
  filter: drop-shadow(0 0 6px rgba(168, 85, 247, 0.55));
}

.sidenav__footer {
  margin-top: auto;
  padding: 12px 14px 2px;
  border-top: 1px solid rgba(255, 255, 255, 0.06);
}

.sidenav__version {
  font-size: 10.5px;
  letter-spacing: 0.14em;
  color: var(--ink-4);
  user-select: none;
}
</style>
