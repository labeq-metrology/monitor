<template>
  <div>
    {{ props.definition }}
  </div>
</template>

<script setup lang="ts">
import {LayoutPanel} from "@/lib/plugin.ts";

const props = defineProps<{
  definition: LayoutPanel,
  data: any
}>()
</script>

<style lang="scss" scoped>

</style>