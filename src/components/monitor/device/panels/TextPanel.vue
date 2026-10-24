<script setup lang="ts">
import {Input} from "@/components/ui/input";
import {DevicePanelProps} from "@/lib/device.ts";

const props = defineProps<DevicePanelProps & { disabled?: boolean }>();
const modelValue = defineModel<string>('value');
</script>

<template>
  <Input
      class="h-7 py-1 px-2 text-xs transition-opacity disabled:opacity-70"
      :class="{ 'opacity-70': disabled }"
      v-model="modelValue"
      :disabled="disabled || readonly"
  />
</template>
