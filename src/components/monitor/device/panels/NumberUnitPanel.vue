<script setup lang="ts">
import {DevicePanelProps} from "@/lib/device.ts";
import {NumberField, NumberFieldContent, NumberFieldInput} from "@/components/ui/number-field";
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select";
import {Field, FieldError} from "@/components/ui/field"
import usePanelConfig from "@/composables/usePanelConfig.ts";
import {z} from "zod";
import {ref, watch, watchEffect} from "vue";
import {Tooltip, TooltipContent, TooltipTrigger} from "@/components/ui/tooltip";

const NumberUnitSchema = z.object({
  units: z.array(z.string())
});

type NumberUnitConfig = z.infer<typeof NumberUnitSchema>;

const props = defineProps<DevicePanelProps>();
const config = usePanelConfig<NumberUnitConfig>(props, NumberUnitSchema);

const modelValue = defineModel<string>('value');
const error = defineModel<string>('error');

const value = ref<number>(0);
const unit = ref<string>('N/A');

watch(modelValue, (newVal) => {
  if (!newVal) return;
  const parts = newVal.split(',');
  value.value = Number(parts[0]);
  unit.value = parts[1] || 'N/A';
}, {immediate: true});

watchEffect(() => {
  const combined = `${value.value.toPrecision(6)},${unit.value}`;

  if (combined !== modelValue.value) {
    modelValue.value = combined;
  }
});
</script>

<template>
  <Tooltip>
    <TooltipTrigger>
      <Field class="max-w-48" :data-invalid="!!error">
        <div class="w-full flex flex-nowrap items-center">
          <NumberField
              type="number"
              v-model.number="value"
              class="flex-1"
              :format-options="{
                minimumFractionDigits: 1,
              }"
              :step="0.000001"
          >
            <NumberFieldContent>
              <NumberFieldInput
                  data-invalid
                  class="text-left text-xs rounded-r-none border-r-transparent max-h-6 px-2 py-3 w-full invalid:border-destructive"
              />
            </NumberFieldContent>
          </NumberField>
          <Select v-model="unit">
            <SelectTrigger
                class="rounded-l-none max-h-6 px-2 py-3 w-auto min-w-[3.5rem] max-w-[5rem] lg:max-w-[6rem] flex-shrink-0"
            >
              <SelectValue
                  placeholder="N/A"
                  class="text-xs whitespace-nowrap overflow-hidden overflow-ellipsis"
              />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem
                    class="text-xs"
                    v-for="(unit, i) in config?.units!!"
                    :key="i"
                    :value="unit"
                >
                  {{ unit }}
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      </Field>
    </TooltipTrigger>
    <TooltipContent v-if="!!error" side="bottom">
      <FieldError :errors="[error]" />
    </TooltipContent>
  </Tooltip>
</template>

<style scoped lang="scss">

</style>