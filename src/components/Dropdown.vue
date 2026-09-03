<script setup lang="ts" generic="T extends string">
import { computed, onBeforeUnmount, onMounted, ref, useId } from 'vue';

const props = defineProps<{
  options: ReadonlyArray<{ value: T; label: string; desc?: string }>;
  modelValue: T;
  /** 无障碍标签（listbox 名称） */
  label: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  'update:modelValue': [T];
}>();

const uid = useId();
const root = ref<HTMLElement | null>(null);
const open = ref(false);
/** 键盘 / 悬停高亮项 */
const activeIndex = ref(0);

const selected = computed(
  () => props.options.find((o) => o.value === props.modelValue) ?? props.options[0],
);

function openList(): void {
  if (props.disabled) return;
  activeIndex.value = Math.max(
    0,
    props.options.findIndex((o) => o.value === props.modelValue),
  );
  open.value = true;
}

function choose(option: { value: T }): void {
  open.value = false;
  if (option.value !== props.modelValue) emit('update:modelValue', option.value);
}

function onDocumentPointerDown(event: PointerEvent): void {
  if (open.value && root.value && !root.value.contains(event.target as Node)) {
    open.value = false;
  }
}

/** roving focus：焦点保持在触发按钮上，高亮项由 aria-activedescendant 表达 */
function onKeydown(event: KeyboardEvent): void {
  if (props.disabled) return;
  if (!open.value) {
    if (['Enter', ' ', 'ArrowDown', 'ArrowUp'].includes(event.key)) {
      event.preventDefault();
      openList();
    }
    return;
  }
  switch (event.key) {
    case 'Escape':
      event.preventDefault();
      open.value = false;
      break;
    case 'ArrowDown':
      event.preventDefault();
      activeIndex.value = Math.min(props.options.length - 1, activeIndex.value + 1);
      break;
    case 'ArrowUp':
      event.preventDefault();
      activeIndex.value = Math.max(0, activeIndex.value - 1);
      break;
    case 'Home':
      event.preventDefault();
      activeIndex.value = 0;
      break;
    case 'End':
      event.preventDefault();
      activeIndex.value = props.options.length - 1;
      break;
    case 'Enter':
    case ' ':
      event.preventDefault();
      if (props.options[activeIndex.value]) choose(props.options[activeIndex.value]);
      break;
    case 'Tab':
      open.value = false;
      break;
  }
}

onMounted(() => document.addEventListener('pointerdown', onDocumentPointerDown));
onBeforeUnmount(() => document.removeEventListener('pointerdown', onDocumentPointerDown));
</script>

<template>
  <div ref="root" class="dd" :class="{ 'is-open': open, 'is-disabled': disabled }">
    <button
      class="dd__trigger"
      type="button"
      role="combobox"
      aria-haspopup="listbox"
      :aria-expanded="open"
      :aria-label="label"
      :aria-activedescendant="open ? `${uid}-opt-${activeIndex}` : undefined"
      @click="open ? (open = false) : openList()"
      @keydown="onKeydown"
    >
      <span class="dd__value">{{ selected?.label }}</span>
      <svg class="dd__chevron" viewBox="0 0 12 12" aria-hidden="true">
        <path
          d="M2.8 4.4 6 7.6l3.2-3.2"
          stroke="currentColor"
          stroke-width="1.4"
          fill="none"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </button>
    <Transition name="dd-pop">
      <ul v-if="open" class="dd__list" role="listbox" :aria-label="label">
        <li
          v-for="(o, i) in options"
          :id="`${uid}-opt-${i}`"
          :key="o.value"
          class="dd__option"
          :class="{ 'is-active': i === activeIndex, 'is-selected': o.value === modelValue }"
          role="option"
          :aria-selected="o.value === modelValue"
          @pointerenter="activeIndex = i"
          @click="choose(o)"
        >
          <span class="dd__check" aria-hidden="true">
            <svg v-if="o.value === modelValue" viewBox="0 0 12 12">
              <path
                d="M2.2 6.4 4.8 9l5-6"
                stroke="currentColor"
                stroke-width="1.5"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </span>
          <span class="dd__option-text">
            <span class="dd__option-label">{{ o.label }}</span>
            <span v-if="o.desc" class="dd__option-desc">{{ o.desc }}</span>
          </span>
        </li>
      </ul>
    </Transition>
  </div>
</template>

<style scoped>
.dd {
  position: relative;
  width: 100%;
}

.dd__trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  width: 100%;
  height: 46px;
  padding: 0 14px 0 18px;
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.07);
  border: 1px solid rgba(255, 255, 255, 0.1);
  font-size: 13.5px;
  letter-spacing: 0.06em;
  color: rgba(255, 255, 255, 0.78);
  text-align: left;
  transition: background-color 0.16s ease, border-color 0.16s ease, box-shadow 0.16s ease;
}

.dd__trigger:hover,
.dd.is-open .dd__trigger {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(168, 85, 247, 0.35);
}

.dd.is-open .dd__trigger {
  box-shadow: 0 0 0 3px rgba(168, 85, 247, 0.12), 0 12px 28px rgba(0, 0, 0, 0.5);
}

.dd.is-disabled .dd__trigger {
  opacity: 0.45;
  cursor: default;
}

.dd__value {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dd__chevron {
  flex: none;
  width: 12px;
  height: 12px;
  color: var(--ink-3);
  transition: transform 0.18s cubic-bezier(0.32, 0.72, 0, 1);
}

.dd.is-open .dd__chevron {
  transform: rotate(180deg);
}

.dd__list {
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  right: 0;
  z-index: 30;
  margin: 0;
  padding: 5px;
  list-style: none;
  border-radius: 14px;
  background: #14101c;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow:
    0 18px 44px rgba(0, 0, 0, 0.55),
    0 0 34px rgba(168, 85, 247, 0.14);
  max-height: 260px;
  overflow-y: auto;
}

.dd__option {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 10px;
  border-radius: 10px;
  cursor: pointer;
  color: rgba(255, 255, 255, 0.72);
}

.dd__option.is-active {
  background: rgba(255, 255, 255, 0.07);
  color: #fff;
}

.dd__option.is-selected {
  color: var(--accent-soft);
}

.dd__check {
  flex: none;
  width: 14px;
  height: 14px;
  display: grid;
  place-items: center;
  color: var(--accent-soft);
}

.dd__check svg {
  width: 11px;
  height: 11px;
}

.dd__option-text {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.dd__option-label {
  font-size: 13px;
  letter-spacing: 0.05em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.dd__option-desc {
  font-size: 11px;
  letter-spacing: 0.03em;
  color: var(--ink-4);
}

.dd-pop-enter-active {
  transition: opacity 0.16s ease-out, transform 0.16s cubic-bezier(0.32, 0.72, 0, 1);
}

.dd-pop-leave-active {
  transition: opacity 0.12s ease-in, transform 0.12s ease-in;
}

.dd-pop-enter-from,
.dd-pop-leave-to {
  opacity: 0;
  transform: translateY(-4px) scale(0.98);
}
</style>
