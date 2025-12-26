<script setup lang="ts">
import { computed, watch } from 'vue';
import { Icon } from '@iconify/vue';
import { useVisualModelsStore } from '@/store/visual-modules.store.ts';
import {DevicePanelProps} from "@/lib/device.ts";

const props = defineProps<DevicePanelProps<any>>();

const store = useVisualModelsStore();
const model = computed(() => store.getModel(props.model));

watch(() => props.model, (newId) => {
  if (!store.getModel(newId)) {
    console.warn(`Model "${newId}" not found.`);
  }
}, { immediate: true });
</script>

<template>
  <div class="relative w-full h-full min-h-[120px]">

    <div
        v-if="!model"
        class="flex flex-col items-center justify-center w-full h-full min-h-[120px] p-6 text-center border-2 border-dashed rounded-lg border-muted bg-muted/20"
    >
      <div class="flex items-center justify-center rounded-full bg-background p-2 ring-1 ring-border shadow-sm mb-3">
        <Icon
            icon="lucide:component"
            class="size-5 text-muted-foreground/70"
        />
      </div>

      <h3 class="text-sm font-medium text-foreground">
        Component not found
      </h3>

      <span class="mt-1 text-xs text-muted-foreground font-mono bg-muted px-2 py-0.5 rounded">{{ model }}</span>
    </div>

    <component
        v-else
        :is="model.component"
        :data="value"
        :config="config"
        class="w-full h-full animate-in fade-in zoom-in-95 duration-200"
    />

  </div>
</template>