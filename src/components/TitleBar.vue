<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';

/** 非 Tauri 环境（纯浏览器开发预览）下不创建窗口句柄，避免整页报错 */
const inTauri = '__TAURI_INTERNALS__' in window;
const win = inTauri ? getCurrentWindow() : null;
</script>

<template>
  <!-- 整条标题栏均可拖动：Tauri 以 mousedown 目标元素自身是否带
       data-tauri-drag-region 判定，故每个非交互子元素都需携带该属性 -->
  <header class="titlebar" data-tauri-drag-region>
    <div class="titlebar__brand" data-tauri-drag-region>
      <span class="titlebar__dot" data-tauri-drag-region aria-hidden="true"></span>
      <span class="titlebar__name" data-tauri-drag-region>TOLP LAUNCHER</span>
    </div>
    <div class="titlebar__actions" data-tauri-drag-region>
      <button
        class="titlebar__btn"
        type="button"
        aria-label="最小化"
        @click="win?.minimize()"
      >
        <svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
          <path d="M2.5 6h7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
        </svg>
      </button>
      <button
        class="titlebar__btn titlebar__btn--close"
        type="button"
        aria-label="关闭"
        @click="win?.close()"
      >
        <svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
          <path
            d="M3.2 3.2l5.6 5.6M8.8 3.2L3.2 8.8"
            stroke="currentColor"
            stroke-width="1.2"
            stroke-linecap="round"
          />
        </svg>
      </button>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  height: 40px;
  flex: none;
  display: flex;
  align-items: stretch;
  justify-content: space-between;
}

.titlebar__brand {
  display: flex;
  align-items: center;
  gap: 9px;
  padding-left: 16px;
}

.titlebar__dot {
  width: 11px;
  height: 11px;
  border-radius: 50%;
  background: radial-gradient(circle at 36% 30%, #efd9fc 0%, #cb8df4 40%, #a855f7 78%, #8a3ed2 100%);
  box-shadow: 0 0 10px rgba(168, 85, 247, 0.65);
}

.titlebar__name {
  font-size: 10.5px;
  font-weight: 500;
  letter-spacing: 0.16em;
  color: var(--ink-3);
}

.titlebar__actions {
  display: flex;
}

.titlebar__btn {
  width: 46px;
  display: grid;
  place-items: center;
  color: rgba(255, 255, 255, 0.7);
  transition: background-color 0.15s ease, color 0.15s ease;
}

.titlebar__btn:hover {
  background: rgba(255, 255, 255, 0.09);
  color: #fff;
}

.titlebar__btn--close:hover {
  background: #e81123;
  color: #fff;
}
</style>
