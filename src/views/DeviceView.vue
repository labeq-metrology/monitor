<template>
  <div>
    <header>
      <h2 class="text-xl">{{ device?.name }}</h2>
      <p class="text-muted-foreground">{{ device?.manufacturer }} &mdash; {{ device?.model }}</p>
      <p class="text-xs text-muted-foreground">Loaded driver: {{ device?.driver }}</p>
      <ul class="mt-2 flex gap-2">
        <li>
          <Button variant="default" class="h-6 text-xs">
            <Icon icon="heroicons:arrow-up-tray" width="24" height="24" />
            <span>Set all parameters</span>
          </Button>
        </li>
        <li>
          <Button variant="secondary" class="h-6 text-xs disabled:[&_svg]:animate-spin" @click="refreshParameters" :disabled="!layout || activeRequests.size > 0">
            <Icon icon="heroicons:arrow-path" width="24" height="24" />
            <span>Refresh parameters</span>
          </Button>
        </li>
        <li>
          <Button :variant="autoupdate ? 'secondary' : 'destructive'" class="h-6 text-xs" @click.prevent="autoupdate = !autoupdate">
            <Icon v-if="autoupdate" icon="heroicons:bolt" width="24" height="24" />
            <Icon v-else icon="heroicons:bolt-slash" width="24" height="24" />
            <p>Auto-update: {{ autoupdate ? 'on' : 'off' }}</p>
          </Button>
        </li>
      </ul>
    </header>
    <Separator class="mt-2 mb-2"/>
    <article
        class="grid"
        :style="{
          gridTemplateRows: `repeat(${layout?.rows.length || 1}, 1fr)`
        }"
    >
      <div v-for="(row, i) in layout?.rows ?? []" :key="i">
        <h3 v-if="row.groupName" class="text-lg my-1">{{ row.groupName }}</h3>
        <div class="grid gap-4 place-self-start" :style="
          {
            gridTemplateColumns: `repeat(${row.columns.length || 1}, minmax(0, 1fr))`
          }
        ">
          <div v-for="(column, j) in row.columns" :key="j" class="flex flex-col">
            <div v-for="(panel, i) in column.panels" :key="i" class="min-w-0">
              <TooltipProvider>
                <TooltipRoot :delayDuration="2000">
                  <TooltipTrigger as-child>
                    <Label :for="`${device?.uuid},${panel.parameter}`" class="pb-1 mt-2 whitespace-nowrap overflow-hidden text-ellipsis block w-fit max-w-full">{{
                        panel.displayName || panel.parameter
                      }}</Label>
                  </TooltipTrigger>
                  <TooltipContent>
                    {{ panel.displayName || panel.parameter }}
                  </TooltipContent>
                </TooltipRoot>
              </TooltipProvider>
              <div class="mt-1">
                <DevicePanel :id="`${device?.uuid},${panel.parameter}`" :definition="panel" v-model:data="parameters[panel.parameter]" :refreshing="refreshing"/>
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
import {LayoutSchema} from "@/lib/plugin.ts";
import DevicePanel from "@/components/monitor/device/DevicePanel.vue";
import {Label} from "@/components/ui/label";
import {Separator} from "@/components/ui/separator";
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {TooltipRoot} from "reka-ui";
import {Icon} from "@iconify/vue";
import {ReadRequestResponse} from "@/lib/labeq.ts";
import {listen} from "@tauri-apps/api/event";
import usePlugin from "@/composables/usePlugin.ts";
import useDevice from "@/composables/useDevice.ts";
import {Button} from "@/components/ui/button";

const route = useRoute();
const router = useRouter();

const device = useDevice();
let plugin = usePlugin(device.value!!);
let layout = ref<LayoutSchema | null>(null);

provide('device', device);
provide('plugin', plugin);

const autoupdate = ref(true);
const refreshing = computed(() => activeRequests.size > 0);

const parameters = reactive<Record<string, string>>({});
const activeRequests = reactive<Map<string, string>>(new Map());

if (!device) {
  router.push("/");
}

const publishListener = await listen("mqtt-publish", (e: any) => {
  if (!autoupdate.value) return;
  const paramRegex = /^device\/(?<device>.+)\/requests\/(?<request>.+)\/(?<action>.+)/;
  const match = paramRegex.exec(e.payload.event.message.topic);
  if (!match || !match.groups || match.groups.action !== 'resolve') return;
  if (!activeRequests.has(match.groups?.request)) return;

  const parsedResponse = JSON.parse(new TextDecoder().decode(new Uint8Array(e.payload.event.message.payload))) as ReadRequestResponse;
  activeRequests.delete(match.groups?.request);
  parameters[parsedResponse.parameter] = parsedResponse.error ? 'Error' : parsedResponse.resolvedValue!!;
});

onMounted(async () => {
  if (!device.value) return;

  layout.value = await invoke<LayoutSchema>('load_layout', {
    driver: device.value.driver,
    variant: device.value.model
  });

  await refreshParameters();
});

const refreshParameters = async () => {
  if (!layout.value) return;
  layout.value.rows
      .forEach(row => row.columns
          .forEach(column => column.panels
              .forEach(panel => {
                const uuid = crypto.randomUUID();
                activeRequests.set(uuid, panel.parameter);
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
}

onBeforeUnmount(() => {
  publishListener();
});
</script>

<style lang="scss" scoped>

</style>`