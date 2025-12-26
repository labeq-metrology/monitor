<script setup lang="ts">
import {DevicePanelProps} from "@/lib/device.ts";
import {Input} from "@/components/ui/input";
import {Select, SelectTrigger, SelectValue, SelectItem, SelectContent, SelectGroup} from "@/components/ui/select";
import usePanelConfig from "@/composables/usePanelConfig.ts";
import {z} from "zod";
import {computed, ref, watch} from "vue";

const NumberUnitSchema = z.object({
  units: z.array(z.string())
});

type NumberUnitConfig = z.infer<typeof NumberUnitSchema>;

const props = defineProps<DevicePanelProps<string>>();
const config = usePanelConfig<NumberUnitConfig>(props, NumberUnitSchema);

const value = ref<number>(0);
const unit = ref<string>('N/A');

watch((props), async () => {
  value.value = Number(props.value.split(',')[0]);
  unit.value = props.value.split(',')[1];
});
</script>

<template>
  <div class="flex max-w-56">
    <Input class="w-3/4 max-w-3/4 rounded-r-none border-r-transparent max-h-6 px-2 py-3" v-model:value="value"/>
    <Select class="w-1/4 max-w-1/4 h-6 py-0" v-model="unit">
      <SelectTrigger class="rounded-l-none max-h-6 px-2 py-3">
        <SelectValue placeholder="N/A" class="text-xs"/>
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectItem class="text-xs" v-for="(unit, i) in config?.units!!" :key="i" :value="unit">{{ unit }}</SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  </div>
</template>

<style scoped lang="scss">

</style>