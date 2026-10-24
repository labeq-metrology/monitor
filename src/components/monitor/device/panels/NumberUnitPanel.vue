<script setup lang="ts">
import {DevicePanelProps} from "@/lib/device.ts";
import {NumberField, NumberFieldContent, NumberFieldInput} from "@/components/ui/number-field";
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select";
import {Field} from "@/components/ui/field"
import usePanelConfig from "@/composables/usePanelConfig.ts";
import {z} from "zod";
import {ref, watch} from "vue";

const NumberUnitSchema = z.object({
  units: z.array(z.string())
});

type NumberUnitConfig = z.infer<typeof NumberUnitSchema>;

const props = defineProps<DevicePanelProps & { disabled?: boolean }>();
const config = usePanelConfig<NumberUnitConfig>(props, NumberUnitSchema);

const modelValue = defineModel<string>('value');
const error = defineModel<string>('error');

const value = ref<number>(0);
const unit = ref<string>('N/A');

watch(() => modelValue.value, (newVal) => {
  if (!newVal) return;
  const parts = newVal.split(',');
  const parsedNum = Number(parts[0]);
  const parsedUnit = parts[1] || 'N/A';

  if (value.value !== parsedNum) value.value = parsedNum;
  if (unit.value !== parsedUnit) unit.value = parsedUnit;
}, {immediate: true});

watch([value, unit], () => {
  if (props.disabled) return;
  const combined = `${value.value ?? 0},${unit.value}`;
  if (combined !== modelValue.value) {
    modelValue.value = combined;
  }
});
</script>

<template>
  <Field class="max-w-48 transition-opacity" :class="{ 'opacity-70': disabled }" :data-invalid="!!error" :aria-disabled="disabled">
    <div class="w-full flex flex-nowrap items-center">
      <NumberField
          type="number"
          v-model.number="value"
          class="flex-1"
          :disabled="disabled || readonly"
          :step="0.000001"
      >
        <NumberFieldContent>
          <NumberFieldInput
              :data-invalid="!!error"
              class="text-left text-xs rounded-r-none border-r-transparent h-7 py-1 px-2 w-full"
              :disabled="disabled || readonly"
          />
        </NumberFieldContent>
      </NumberField>
      <Select v-model="unit" :disabled="disabled || readonly">
        <SelectTrigger class="rounded-l-none h-7 max-h-7 py-1 px-2 w-auto min-w-[3.5rem] max-w-[5rem] lg:max-w-[6rem] flex-shrink-0">
          <SelectValue placeholder="N/A" class="text-xs whitespace-nowrap overflow-hidden overflow-ellipsis" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem class="text-xs" v-for="(u, i) in config?.units!!" :key="i" :value="u">
              {{ u }}
            </SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  </Field>
</template>
