<script setup lang="ts">
import {DevicePanelProps} from "@/lib/device.ts";
import {NumberField, NumberFieldContent, NumberFieldInput} from "@/components/ui/number-field";
import {Select, SelectTrigger, SelectValue, SelectItem, SelectContent, SelectGroup} from "@/components/ui/select";
import usePanelConfig from "@/composables/usePanelConfig.ts";
import {z} from "zod";
import {ref, watch, watchEffect} from "vue";

const NumberUnitSchema = z.object({
  units: z.array(z.string())
});

type NumberUnitConfig = z.infer<typeof NumberUnitSchema>;

const props = defineProps<DevicePanelProps>();
const config = usePanelConfig<NumberUnitConfig>(props, NumberUnitSchema);

const modelValue = defineModel<string>('value');

const value = ref<number>(0);
const unit = ref<string>('N/A');

watch(modelValue, (newVal) => {
  if (!newVal) return;
  const parts = newVal.split(',');
  value.value = Number(parts[0]);
  unit.value = parts[1] || 'N/A';
}, { immediate: true });

watchEffect(() => {
  const combined = `${value.value},${unit.value}`;

  if (combined !== modelValue.value) {
    console.log('aaa', combined)
    modelValue.value = combined;
  }
});
</script>

<template>
  <div class="flex max-w-48">
    <NumberField type="number" v-model.number="value">
      <NumberFieldContent>
        <NumberFieldInput class="text-left rounded-r-none border-r-transparent max-h-6 px-2 py-3" />
      </NumberFieldContent>
    </NumberField>
    <Select class="w-1/3 max-w-1/3 h-6 py-0" v-model="unit">
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