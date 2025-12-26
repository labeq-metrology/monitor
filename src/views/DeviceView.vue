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
        <div class="grid gap-4 self-start" :style="
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
import {invoke} from "@tauri-apps/api/core";
import {computed, onBeforeUnmount, onMounted, provide, reactive, ref} from "vue";
import {LabeqPluginManifest, LayoutSchema} from "@/lib/plugin.ts";
import DevicePanel from "@/components/monitor/device/DevicePanel.vue";
import {Label} from "@/components/ui/label";
import {Separator} from "@/components/ui/separator";
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {TooltipRoot} from "reka-ui";
import {ReadRequestResponse} from "@/lib/labeq.ts";
import {listen} from "@tauri-apps/api/event";
import usePlugin from "@/composables/usePlugin.ts";
import useDevice from "@/composables/useDevice.ts";

const route = useRoute();
const router = useRouter();

const device = useDevice();
let plugin = usePlugin(device.value!!);
let layout = ref<LayoutSchema | null>(null);

provide('device', device);
provide('plugin', plugin);

const parameters = reactive<Map<string, string>>(new Map());
const activeRequests = reactive<Map<string, string>>(new Map());

const getParameterValue = (parameter: string) => computed(() => {
  return parameters.has(parameter) ? parameters.get(parameter) : "N/A";
});

if (!device) {
  router.push("/");
}

const publishListener = await listen("mqtt-publish", (e: any) => {
  const paramRegex = /^device\/(?<device>.+)\/requests\/(?<request>.+)\/(?<action>.+)/;
  const match = paramRegex.exec(e.payload.event.message.topic);
  if (!match || !match.groups) return;
  if (match.groups.action !== 'resolve') return;

  const parsedResponse = JSON.parse(new TextDecoder().decode(new Uint8Array(e.payload.event.message.payload))) as ReadRequestResponse;

  parameters.set(parsedResponse.parameter, parsedResponse.error ? 'Error' : parsedResponse.resolvedValue!!);
});

onMounted(async () => {
  if (!device.value) return;

  layout.value = await invoke<LayoutSchema>('load_layout', {
    driver: device.value.driver,
    variant: device.value.model
  });

  layout.value.rows
      .forEach(row => row.columns
          .forEach(column => column.panels
              .forEach(panel => {
                const uuid = crypto.randomUUID();
                activeRequests.set(uuid, panel.parameter);
                parameters.set(panel.parameter, 'Retrieving...');
                invoke('gateway_publish', {
                  connId: route.params.connectionId as string,
                  topic: `device/${device.value!!.uuid}/parameters/${panel.parameter}/read`,
                  payload: JSON.stringify({
                    requestId: uuid,
                    returnType: 'string'
                  }),
                  retain: false
                });
              })
          )
      );
});

onBeforeUnmount(() => {
  publishListener();
});
</script>

<style lang="scss" scoped>

</style>`