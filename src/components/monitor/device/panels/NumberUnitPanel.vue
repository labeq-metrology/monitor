<script setup lang="ts">
import {DevicePanelProps} from "@/lib/device.ts";
import {Input} from "@/components/ui/input";
import {Select, SelectTrigger, SelectValue} from "@/components/ui/select";

const props = defineProps<DevicePanelProps<number>>();
</script>

<template>
  <div class="flex max-w-56">
    <Input class="w-3/4 max-w-3/4 rounded-r-none border-r-transparent max-h-6 px-2 py-3"/>
    <Select class="w-1/4 max-w-1/4 h-6 py-0">
      <SelectTrigger class="rounded-l-none max-h-6 px-2 py-3">
        <SelectValue placeholder="N/A" class="text-xs"/>
      </SelectTrigger>
    </Select>
  </div>
</template>

<style scoped lang="scss">

</style>