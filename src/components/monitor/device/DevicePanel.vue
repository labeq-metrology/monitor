<template>
  <component
      :is="resolvedComponent"
      v-bind="panelProps"
      class=""
  />
</template>

<script setup lang="ts">
import {computed, ComputedRef, Reactive, reactive, watchEffect} from "vue";
import type {LayoutPanel} from "@/lib/plugin.ts";
import {DevicePanelProps} from "@/lib/device.ts";

import TextPanel from "@/components/monitor/device/panels/TextPanel.vue";
import NumberPanel from "@/components/monitor/device/panels/NumberPanel.vue";
import NumberUnitPanel from "@/components/monitor/device/panels/NumberUnitPanel.vue";
import ImagePanel from "@/components/monitor/device/panels/ImagePanel.vue";
import ChartPanel from "@/components/monitor/device/panels/ChartPanel.vue";
import VisualModelContainer from "@/components/monitor/device/panels/VisualModelContainer.vue";
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";
import {invoke} from "@tauri-apps/api/core";
import useDevice from "@/composables/useDevice.ts";

const props = defineProps<{
  definition: LayoutPanel,
  data: ComputedRef,
}>();

const basicPanels = {
  text: TextPanel,
  number: NumberPanel,
  "number-unit": NumberUnitPanel,
  chart: ChartPanel,
  image: ImagePanel,
} as const;

const visualModels = useVisualModelsStore();

const resolvedComponent = computed(() => {
  if (Object.keys(basicPanels).includes(props.definition.kind)) return basicPanels[props.definition.kind as keyof typeof basicPanels];
  if (visualModels.models.has(props.definition.kind)) return VisualModelContainer;
});

const device = useDevice();

const panelProps: Reactive<DevicePanelProps<any>> = reactive({
  model: props.definition.kind,
  parameter: props.definition.parameter,
  value: props.data.value,
  readonly: props.definition.readonly
});

watchEffect(async () => {
  if (device.value?.driver && device.value?.model) {
    panelProps.value = await invoke<string>('format_param_to_panel', {
      driver: device.value.driver,
      variant: device.value.model,
      parameter: props.definition.parameter,
      value: props.data.value
    });
  }
});
</script>
