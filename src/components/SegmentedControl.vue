<script setup lang="ts" generic="T extends string">
const props = defineProps<{
  options: ReadonlyArray<{ value: T; label: string }>;
  modelValue: T;
  /** 无障碍标签（radiogroup 名称） */
  label: string;
}>();

const emit = defineEmits<{
  'update:modelValue': [T];
}>();

/** 滑块位移：按选中项索引平移一个轨道宽度 */
function thumbShift(): number {
  const index = props.options.findIndex((o) => o.value === props.modelValue);
  return index <= 0 ? 0 : index;
}
</script>

<template>
  <div class="seg" role="radiogroup" :aria-label="label">
    <span class="seg__thumb" :style="{ transform: `translateX(${thumbShift() * 100}%)` }" aria-hidden="true"></span>
    <button
      v-for="o in options"
      :key="o.value"
      class="seg__item"
      :class="{ 'is-active': modelValue === o.value }"
      type="button"
      role="radio"
      :aria-checked="modelValue === o.value"
      @click="emit('update:modelValue', o.value)"
    >
      {{ o.label }}
    </button>
  </div>
</template>

<style scoped>
.seg {
  position: relative;
  display: grid;
  grid-auto-flow: column;
  grid-auto-columns: 1fr;
  padding: 4px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.07);
}

.seg__thumb {
  position: absolute;
  top: 4px;
  left: 4px;
  width: calc(100% / v-bind('options.length') - 4px);
  height: calc(100% - 8px);
  border-radius: 999px;
  background: linear-gradient(135deg, #b96cf5, #9333ea);
  box-shadow: 0 0 16px rgba(168, 85, 247, 0.45), inset 0 1px 0 rgba(255, 255, 255, 0.25);
  transition: transform 0.26s cubic-bezier(0.32, 0.72, 0, 1);
}

.seg__item {
  position: relative;
  z-index: 1;
  height: 38px;
  border-radius: 999px;
  font-size: 13px;
  letter-spacing: 0.08em;
  color: rgba(255, 255, 255, 0.55);
  white-space: nowrap;
  transition: color 0.2s ease;
}

.seg__item:hover {
  color: rgba(255, 255, 255, 0.8);
}

.seg__item.is-active {
  color: #fff;
}
</style>
