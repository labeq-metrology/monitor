<template>
  <div>
    <header>
      <h2 class="text-xl">{{ device.name }}</h2>
      <p class="text-muted-foreground">{{ device?.manufacturer }} &mdash; {{ device.model }}</p>
      <p class="text-xs text-muted-foreground">Loaded driver: {{ device.driver }}</p>
    </header>
    <Separator class="mt-4 mb-2"/>
    <article
        class="grid"
        :style="{
          gridTemplateRows: `repeat(${layout?.rows.length || 1}, 1fr)`
        }"
    >
      <div v-for="(row, i) in layout?.rows ?? []" :key="i">
        <h3 v-if="row.groupName" class="text-lg my-1">{{ row.groupName }}</h3>
        <div class="grid gap-2" :style="
          {
            gridTemplateColumns: `repeat(${row.columns.length || 1}, minmax(0, 1fr))`
          }
        ">
          <div v-for="(column, j) in row.columns" :key="j" class="flex flex-col gap-2">
            <div v-for="(panel, i) in column.panels" :key="i" class="min-w-0">
              <TooltipProvider>
                <TooltipRoot :delayDuration="2000">
                  <TooltipTrigger as-child>
                    <Label class="pb-1 mt-2 whitespace-nowrap overflow-hidden text-ellipsis block w-fit max-w-full">{{
                        panel.displayName || panel.parameter
                      }}</Label>
                  </TooltipTrigger>
                  <TooltipContent>
                    {{ panel.displayName || panel.parameter }}
                  </TooltipContent>
                </TooltipRoot>
              </TooltipProvider>
              <div class="mt-1">
                <DevicePanel :definition="panel" :data="getParameterValue(panel.parameter)"/>
              </div>
            </div>
          </div>
        </div>
      </div>
    </article>
  </div>
</template>

<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {useConnectionStore} from "@/store/connection.store.ts";
import {invoke} from "@tauri-apps/api/core";
import {computed, onMounted, reactive, ref} from "vue";
import {LabeqPluginManifest, LayoutSchema} from "@/lib/plugin.ts";
import DevicePanel from "@/components/monitor/device/DevicePanel.vue";
import {Label} from "@/components/ui/label";
import {Separator} from "@/components/ui/separator";
import {TooltipTrigger, TooltipContent, TooltipProvider} from "@/components/ui/tooltip";
import {TooltipRoot} from "reka-ui";

const connections = useConnectionStore();
const route = useRoute();
const router = useRouter();

const device = connections.connections
    .get(route.params.connectionId as string)?.devices
    .find(d => d.uuid === route.params.id)!!;

const parameters = reactive<Map<string, string>>(new Map());

const getParameterValue = (parameter: string) => computed(() => {
  return parameters.has(parameter) ? parameters.get(parameter) : "N/A";
});

if (!device) {
  router.push("/");
}

let plugin = ref<LabeqPluginManifest | null>(null);
let layout = ref<LayoutSchema | null>(null);

onMounted(async () => {
  plugin.value = (await invoke<string[]>('list_plugins')).map(d => JSON.parse(d)).find(plugin => plugin.drivers.includes(device.driver));
  layout.value = await invoke<LayoutSchema>('load_layout', {driver: device.driver, variant: device.model});
  console.log(layout.value);
});
</script>

<style lang="scss" scoped>

</style>`