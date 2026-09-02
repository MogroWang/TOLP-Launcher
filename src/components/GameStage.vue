<script setup lang="ts">
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

const props = defineProps<{
  url: string;
}>();

const emit = defineEmits<{
  exit: [];
}>();

const win = getCurrentWindow();
const leaving = ref(false);

async function exitGame(): Promise<void> {
  if (leaving.value) return;
  leaving.value = true;
  try {
    await win.setFullscreen(false);
  } finally {
    emit('exit');
  }
}
</script>

<template>
  <div class="stage" data-stage>
    <iframe
      class="stage__frame"
      :src="props.url"
      title="光点之旅"
      allow="pointer-lock; gamepad; fullscreen; autoplay"
    ></iframe>
    <button class="stage__exit" type="button" aria-label="退出游戏，返回启动器" @click="exitGame">
      <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true">
        <path
          d="M3.8 3.8l8.4 8.4M12.2 3.8l-8.4 8.4"
          stroke="currentColor"
          stroke-width="1.6"
          stroke-linecap="round"
        />
      </svg>
      <span>退出游戏</span>
    </button>
  </div>
</template>

<style scoped>
.stage {
  position: absolute;
  inset: 0;
  background: #000;
  z-index: 30;
}

.stage__frame {
  width: 100%;
  height: 100%;
  border: 0;
  display: block;
  background: #000;
}

/* 游戏内唯一出口：右上角胶囊，静息半透明，悬停点亮 */
.stage__exit {
  position: absolute;
  top: 14px;
  right: 18px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: 999px;
  background: rgba(20, 18, 24, 0.55);
  backdrop-filter: blur(16px) saturate(140%);
  border: 1px solid rgba(255, 255, 255, 0.14);
  color: rgba(255, 255, 255, 0.72);
  font-size: 12px;
  letter-spacing: 0.1em;
  opacity: 0.5;
  transition: opacity 0.2s ease, background-color 0.2s ease, transform 0.12s ease;
}

.stage__exit:hover {
  opacity: 1;
  background: rgba(20, 18, 24, 0.75);
  color: #fff;
}

.stage__exit:active {
  transform: scale(0.96);
}
</style>
