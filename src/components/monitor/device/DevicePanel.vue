<template>
  <component
      :is="resolvedComponent"
      v-bind="panelProps"
      class=""
  />
</template>

<script setup lang="ts">
import {computed, ComputedRef} from "vue";
import type {LayoutPanel} from "@/lib/plugin.ts";
import {DevicePanelProps} from "@/lib/device.ts";

import NumberPanel from "@/components/monitor/device/panels/NumberPanel.vue";
import ImagePanel from "@/components/monitor/device/panels/ImagePanel.vue";
import ChartPanel from "@/components/monitor/device/panels/ChartPanel.vue";
import TextPanel from "@/components/monitor/device/panels/TextPanel.vue";

const props = defineProps<{
  definition: LayoutPanel,
  data: ComputedRef,
}>();

const panelMap = {
  text: TextPanel,
  number: NumberPanel,
  chart: ChartPanel,
  image: ImagePanel,
  custom: TextPanel,
} as const;

const resolvedComponent = computed(() => {
  return panelMap[props.definition.kind];
});

const panelProps: ComputedRef<DevicePanelProps<any>> = computed(() => ({
  value: props.data.value,
  readonly: props.definition.readonly,
}));
</script>
