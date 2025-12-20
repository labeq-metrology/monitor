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

import TextPanel from "@/components/monitor/device/panels/TextPanel.vue";
import NumberPanel from "@/components/monitor/device/panels/NumberPanel.vue";
import NumberUnitPanel from "@/components/monitor/device/panels/NumberUnitPanel.vue";
import ImagePanel from "@/components/monitor/device/panels/ImagePanel.vue";
import ChartPanel from "@/components/monitor/device/panels/ChartPanel.vue";
import VisualModelContainer from "@/components/monitor/device/panels/VisualModelContainer.vue";

const props = defineProps<{
  definition: LayoutPanel,
  data: ComputedRef,
}>();

const panelMap = {
  text: TextPanel,
  number: NumberPanel,
  "number-unit": NumberUnitPanel,
  chart: ChartPanel,
  image: ImagePanel,
  custom: VisualModelContainer,
} as const;

const resolvedComponent = computed(() => {
  return panelMap[props.definition.kind];
});

const panelProps: ComputedRef<DevicePanelProps<any>> = computed(() => ({
  parameter: props.definition.parameter,
  value: props.data.value,
  readonly: props.definition.readonly,
}));
</script>
