<template>
  <div
      v-if="ghost.visible"
      class="fixed pointer-events-none z-[999999] px-2 py-[2px] text-[11px] whitespace-nowrap rounded border opacity-90 bg-[var(--color-accent)] text-[var(--color-accent-foreground)] border-[var(--color-border)]"
      :style="positionStyle"
  >
    {{ ghost.label }}
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { dragState } from '@/composables/useDrag';

defineProps<{
  ghost: { label: string; visible: boolean }
}>();

const positionStyle = computed(() => ({
  left: `${dragState.x + 12}px`,
  top: `${dragState.y + 4}px`,
}));
</script>