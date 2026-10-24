<template>
  <div
      ref="el"
      @pointerdown="onPointerDown"
      @click="$emit('click')"
      class="group flex items-center justify-between gap-2 px-3 h-full text-sm cursor-pointer transition-colors border-r border-border min-w-[120px] max-w-[200px] shrink-0 select-none relative"
      :class="[
       isActive ? 'bg-background text-foreground border-t-primary border-t-[2px]' : 'bg-transparent text-muted-foreground border-t-transparent border-t-[2px] hover:bg-muted/50',
       isOver ? 'bg-muted/60' : ''
    ]"
  >
    <span class="truncate text-xs pointer-events-none">{{ tab.title }}</span>
    <button
        @click.stop="$emit('close')"
        class="rounded-md opacity-0 group-hover:opacity-100 hover:bg-muted-foreground/20 p-1 flex items-center justify-center shrink-0 transition-opacity"
        :class="{ 'opacity-100': isActive }"
    >
      <Icon icon="heroicons:x-mark" class="w-3.5 h-3.5 pointer-events-none" />
    </button>

    <div
        v-if="isOver"
        class="absolute top-0 bottom-0 w-0.5 bg-primary pointer-events-none z-10"
        :class="dropSide === 'left' ? 'left-0' : 'right-0'"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { Icon } from '@iconify/vue';
import { useDragSource, useDragTarget, dragState } from '@/composables/useDrag';
import type { Tab } from '@/store/tabs.store';

const props = defineProps<{ tab: Tab; isActive: boolean; index: number }>();
const emit = defineEmits(['click', 'close', 'drop']);

const el = ref<HTMLElement | null>(null);
const { startDrag } = useDragSource();

const dropSide = computed(() => {
  if (!isOver.value || !el.value) return 'left';
  const rect = el.value.getBoundingClientRect();
  const midPoint = rect.left + rect.width / 2;
  return dragState.x > midPoint ? 'right' : 'left';
});

const { isOver, register } = useDragTarget(
    () => el.value,
    'tab',
    (draggedId) => {
      // If dropped on the right half, insert AFTER this index
      const targetIdx = dropSide.value === 'right' ? props.index + 1 : props.index;
      emit('drop', { draggedId, targetIndex: targetIdx });
    }
);

onMounted(() => {
  register();
});

const onPointerDown = (e: PointerEvent) => {
  startDrag('tab', props.tab.id, props.tab.title, e);
};
</script>
