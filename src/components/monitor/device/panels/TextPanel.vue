<script setup lang="ts">

import {Input} from "@/components/ui/input";
import {DevicePanelProps} from "@/lib/device.ts";

const props = defineProps<DevicePanelProps>();
const modelValue = defineModel<string>('value');
</script>

<template>
  <Input :modelValue="modelValue" :disabled="props.readonly" />
</template>

<style scoped lang="scss">

</style>