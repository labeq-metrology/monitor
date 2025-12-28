<template>
  <component
      :is="resolvedComponent"
      v-bind="finalPanelProps"
      v-model:value="formattedValue"
      @update:value="handlePanelUpdate"
  />
</template>

<script setup lang="ts">
import {computed, nextTick, ref, watch} from "vue";
import type {LayoutPanel} from "@/lib/plugin.ts";

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
  refreshing: boolean,
}>();

const data = defineModel<string>('data');

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

const formattedValue = ref("");
let isSettingFormattedValue = false; // Track when WE are updating formattedValue

watch(() => data.value, async (newRawValue) => {
  if (device.value?.driver && device.value?.model && newRawValue) {
    const result = await invoke<string>('format_param_to_panel', {
      driver: device.value.driver,
      variant: device.value.model,
      parameter: props.definition.parameter,
      value: newRawValue
    });

    if (formattedValue.value !== result) {
      isSettingFormattedValue = true;
      formattedValue.value = result;
      await nextTick();
      isSettingFormattedValue = false;
    }
  } else {
    isSettingFormattedValue = true;
    formattedValue.value = newRawValue ?? '';
    await nextTick();
    isSettingFormattedValue = false;
  }
}, { immediate: true });

const handlePanelUpdate = async (newVal: string) => {
  if (!newVal || !device.value || !device.value?.driver || !device.value?.model) return;
  if (isSettingFormattedValue) return;
  if (newVal === data.value) return;

  console.log('newVal', newVal)
  const result = await invoke<string>('format_panel_to_param', {
    driver: device.value.driver,
    variant: device.value.model,
    parameter: props.definition.parameter,
    value: newVal
  });

  data.value = result;
  console.log('User edit synced to raw data:', result);
};

const finalPanelProps = computed(() => ({
  model: props.definition.kind,
  parameter: props.definition.parameter,
  readonly: props.definition.readonly,
  refreshing: props.refreshing
}));
</script>