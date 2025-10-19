<script setup lang="ts">

import {Input} from "@/components/ui/input";
import {DevicePanelProps} from "@/lib/device.ts";

const props = defineProps<DevicePanelProps<string>>();
</script>

<template>
  <Input :modelValue="props.value" :disabled="props.readonly" />
</template>

<style scoped lang="scss">

</style>