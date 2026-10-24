<script setup lang="ts">
import { DevicePanelProps } from "@/lib/device.ts";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import usePanelConfig from "@/composables/usePanelConfig.ts";
import { z } from "zod";

const ActionSchema = z.object({
  label: z.string().optional().default("Execute"),
  value: z.string().optional().default("1")
});

type ActionConfig = z.infer<typeof ActionSchema>;

const props = defineProps<DevicePanelProps & { disabled?: boolean }>();
const config = usePanelConfig<ActionConfig>(props, ActionSchema);

const error = defineModel<string>('error');
const emit = defineEmits(['execute']);

const executeAction = () => {
  if (props.disabled) return;
  emit('execute', config.value?.value ?? "1");
};
</script>

<template>
  <Field class="max-w-48 transition-opacity" :class="{ 'opacity-70': disabled }" :data-invalid="!!error">
    <div class="w-full flex flex-nowrap items-center">
      <Button
          variant="secondary"
          class="w-full h-7 py-1 px-2 text-xs"
          :disabled="disabled"
          @click="executeAction"
      >
        {{ config?.label ?? 'Execute' }}
      </Button>
    </div>
  </Field>
</template>
