<template>
  <div class="relative w-full group">
    <component
        :is="resolvedComponent"
        v-bind="finalPanelProps"
        :value="formattedValue"
        v-model:error="error"
        :disabled="isPanelDisabled"
        @update:value="handlePanelUpdate"
        @execute="handleExecute"
    />

    <div
        v-if="isPanelDisabled"
        class="absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none flex items-center"
    >
      <Icon
          icon="heroicons:arrow-path"
          class="animate-spin text-muted-foreground opacity-50 w-4 h-4"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch, inject, Ref} from "vue";
import type {LayoutPanel} from "@/lib/plugin.ts";
import {invoke} from "@tauri-apps/api/core";
import {LabeqDevice, ParameterValue} from "@/lib/labeq.ts";
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";
import {Icon} from "@iconify/vue";

import TextPanel from "@/components/monitor/device/panels/TextPanel.vue";
import NumberPanel from "@/components/monitor/device/panels/NumberPanel.vue";
import NumberUnitPanel from "@/components/monitor/device/panels/NumberUnitPanel.vue";
import ImagePanel from "@/components/monitor/device/panels/ImagePanel.vue";
import ChartPanel from "@/components/monitor/device/panels/ChartPanel.vue";
import VisualModelContainer from "@/components/monitor/device/panels/VisualModelContainer.vue";
import ActionPanel from "@/components/monitor/device/panels/ActionPanel.vue";

const props = defineProps<{
  definition: LayoutPanel,
  refreshing: boolean,
  processing: boolean
}>();

const data = defineModel<ParameterValue>('data');
const error = defineModel<string>('error');
const emit = defineEmits(['dirty', 'write']);

const basicPanels = {
  text: TextPanel,
  number: NumberPanel,
  "number-unit": NumberUnitPanel,
  chart: ChartPanel,
  image: ImagePanel,
  action: ActionPanel,
} as const;

const visualModels = useVisualModelsStore();

const resolvedComponent = computed(() => {
  if (Object.keys(basicPanels).includes(props.definition.kind)) return basicPanels[props.definition.kind as keyof typeof basicPanels];
  if (visualModels.models.has(props.definition.kind)) return VisualModelContainer;
});

const device = inject<Ref<LabeqDevice | undefined>>('device');

const formattedValue = ref("");
const isFormatting = ref(false);
const lastSyncedRaw = ref<ParameterValue | undefined>(undefined);

const isPanelDisabled = computed(() => props.processing || props.refreshing || isFormatting.value);

watch(() => data.value, async (newRawValue) => {
  if (newRawValue === lastSyncedRaw.value) return;
  if (newRawValue === undefined || newRawValue === null || !device?.value?.driver || !device?.value?.model) {
    formattedValue.value = newRawValue as string;
    return;
  }

  isFormatting.value = true;
  try {
    const result = await invoke<string>('format_param_to_panel', {
      driver: device.value.driver,
      variant: device.value.model,
      parameter: props.definition.parameter,
      value: newRawValue
    });

    formattedValue.value = result;
    lastSyncedRaw.value = newRawValue;
    error.value = "";
  } catch (err) {
    error.value = "Failed to format parameter for display.";
  } finally {
    isFormatting.value = false;
  }
}, { immediate: true });

const handlePanelUpdate = async (newVal: string) => {
  if (newVal === undefined || newVal === null || newVal === formattedValue.value || !device?.value?.driver || !device?.value?.model) return;

  formattedValue.value = newVal;

  isFormatting.value = true;
  try {
    const raw = await invoke<ParameterValue>('format_panel_to_param', {
      driver: device.value.driver,
      variant: device.value.model,
      parameter: props.definition.parameter,
      value: newVal
    });

    if (JSON.stringify(raw) !== JSON.stringify(data.value)) {
      lastSyncedRaw.value = raw;
      data.value = raw;
      error.value = "";
      emit('dirty');
    }
  } catch (err) {
    error.value = "Failed to prepare parameter for device.";
  } finally {
    isFormatting.value = false;
  }
};

const handleExecute = async (executeVal: string) => {
  if (!device?.value?.driver || !device?.value?.model) return;

  isFormatting.value = true;
  try {
    const raw = await invoke<ParameterValue>('format_panel_to_param', {
      driver: device.value.driver,
      variant: device.value.model,
      parameter: props.definition.parameter,
      value: executeVal
    });

    lastSyncedRaw.value = raw;
    data.value = raw;
    error.value = "";
    emit('write');
  } catch (err) {
    error.value = "Failed to prepare action.";
  } finally {
    isFormatting.value = false;
  }
};

const finalPanelProps = computed(() => ({
  model: props.definition.kind,
  parameter: props.definition.parameter,
  readonly: props.definition.readonly,
  disabled: isPanelDisabled.value,
  refreshing: props.refreshing
}));
</script>