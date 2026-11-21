<template>
  <div
      class="relative w-full group"
      :aria-busy="pendingFormattingRequests > 0"
  >
    <component
        v-if="resolvedComponent"
        :is="resolvedComponent"
        v-bind="finalPanelProps"
        :value="formattedValue"
        :model-value="formattedValue"
        :data="formattedValue"
        v-model:error="error"
        :disabled="isPanelDisabled"
        @update:value="handlePanelUpdate"
        @update:model-value="handlePanelUpdate"
        @execute="handleExecute"
    />
    <div v-else class="text-xs text-destructive">
      Panel plugin "{{ definition.kind }}" is not available.
    </div>

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
import {computed, inject, ref, Ref, watch} from "vue";
import type {LayoutPanel} from "@/lib/plugin.ts";
import {invoke} from "@tauri-apps/api/core";
import {LabeqDevice, ParameterValue} from "@/lib/labeq.ts";
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";
import {Icon} from "@iconify/vue";

const props = defineProps<{
  definition: LayoutPanel,
  refreshing: boolean,
  processing: boolean
}>();

const data = defineModel<ParameterValue>('data');
const error = defineModel<string>('error');
const emit = defineEmits(['dirty', 'write']);

const visualModels = useVisualModelsStore();

const resolvedComponent = computed(() => {
  return visualModels.getModel(props.definition.kind)?.component;
});

const device = inject<Ref<LabeqDevice | undefined>>('device');

const formattedValue = ref("");
const pendingFormattingRequests = ref(0);
const latestFormattingRequest = ref(0);
const lastSyncedRaw = ref<ParameterValue | undefined>(undefined);

const isPanelDisabled = computed(() => props.processing || props.refreshing);

const beginFormatting = () => {
  pendingFormattingRequests.value += 1;
  const requestId = latestFormattingRequest.value + 1;
  latestFormattingRequest.value = requestId;
  return requestId;
};

const endFormatting = () => {
  pendingFormattingRequests.value = Math.max(
    0,
    pendingFormattingRequests.value - 1,
  );
};

watch(() => data.value, async (newRawValue) => {
  if (newRawValue === lastSyncedRaw.value) return;
  if (newRawValue === undefined || newRawValue === null || !device?.value?.driver || !device?.value?.model) {
    formattedValue.value = newRawValue as string;
    return;
  }

  const requestId = beginFormatting();
  try {
    const formatted = await invoke<string>('format_param_to_panel', {
      driver: device.value.driver,
      variant: device.value.model,
      parameter: props.definition.parameter,
      value: newRawValue
    });
    if (requestId === latestFormattingRequest.value) {
      formattedValue.value = formatted;
    }
    lastSyncedRaw.value = newRawValue;
    error.value = "";
  } catch (err) {
    error.value = "Failed to format parameter for display.";
  } finally {
    endFormatting();
  }
}, { immediate: true });

const handlePanelUpdate = async (newVal: string) => {
  if (newVal === undefined || newVal === null || newVal === formattedValue.value || !device?.value?.driver || !device?.value?.model) return;

  formattedValue.value = newVal;

  const requestId = beginFormatting();
  try {
    const raw = await invoke<ParameterValue>('format_panel_to_param', {
      driver: device.value.driver,
      variant: device.value.model,
      parameter: props.definition.parameter,
      value: newVal
    });

    if (requestId === latestFormattingRequest.value &&
        JSON.stringify(raw) !== JSON.stringify(data.value)) {
      lastSyncedRaw.value = raw;
      data.value = raw;
      error.value = "";
      emit('dirty');
    }
  } catch (err) {
    error.value = "Failed to prepare parameter for device.";
  } finally {
    endFormatting();
  }
};

const handleExecute = async (executeVal: string) => {
  if (!device?.value?.driver || !device?.value?.model) return;

  beginFormatting();
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
    endFormatting();
  }
};

const finalPanelProps = computed(() => ({
  parameter: props.definition.parameter,
  readonly: props.definition.readonly,
  disabled: isPanelDisabled.value,
  refreshing: props.refreshing,
  panel: props.definition.kind
}));
</script>