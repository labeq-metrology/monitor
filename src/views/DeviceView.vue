<template>
  <div>
    <header>
      <h2 class="text-xl">{{ device?.name }}</h2>
      <p class="text-muted-foreground">{{ device?.manufacturer }} &mdash; {{ device?.model }}</p>
      <p class="text-xs text-muted-foreground">Loaded driver: {{ device?.driver }}</p>
      <ul class="mt-2 flex gap-2">
        <li>
          <Button variant="default" class="h-6 text-xs disabled:[&_svg]:animate-bounce" @click="writeAllParameters"
                  :disabled="!layout || activeWriteRequests.size > 0">
            <Icon icon="heroicons:arrow-up-tray" width="24" height="24" v-if="activeWriteRequests.size === 0"/>
            <Icon icon="heroicons:arrow-up" width="24" height="24" v-else/>
            <span>Set all parameters</span>
          </Button>
        </li>
        <li>
          <Button variant="secondary" class="h-6 text-xs disabled:[&_svg]:animate-spin" @click="refreshParameters"
                  :disabled="!layout || activeReadRequests.size > 0">
            <Icon icon="heroicons:arrow-path" width="24" height="24"/>
            <span>Refresh parameters</span>
          </Button>
        </li>
        <li>
          <Button :variant="autoupdate ? 'secondary' : 'destructive'" class="h-6 text-xs"
                  @click.prevent="autoupdate = !autoupdate">
            <Icon v-if="autoupdate" icon="heroicons:bolt" width="24" height="24"/>
            <Icon v-else icon="heroicons:bolt-slash" width="24" height="24"/>
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
              <div class="flex items-center gap-2 mt-2 h-4">
                <Label
                    :for="`${device?.uuid},${panel.parameter}`"
                    class="whitespace-nowrap overflow-hidden text-ellipsis block max-w-full leading-none"
                >
                  {{ panel.displayName || panel.parameter }}
                </Label>

                <div class="w-4 h-4 flex items-center justify-center shrink-0">
                  <Tooltip v-if="!!errors[panel.parameter]">
                    <TooltipTrigger class="flex items-center">
                      <Icon
                          icon="heroicons:exclamation-triangle-16-solid"
                          class="text-destructive"
                          width="16"
                          height="16"
                      />
                    </TooltipTrigger>
                    <TooltipContent>
                      <p class="text-destructive">{{ errors[panel.parameter] }}</p>
                    </TooltipContent>
                  </Tooltip>
                </div>
              </div>
              <div class="mt-2">
                <DevicePanel :id="`${device?.uuid},${panel.parameter}`" :definition="panel"
                             v-model:data="parameters[panel.parameter]" v-model:error="errors[panel.parameter]"
                             :refreshing="refreshing"/>
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
import {Tooltip, TooltipContent, TooltipTrigger} from "@/components/ui/tooltip";
import {Icon} from "@iconify/vue";
import {ReadRequestResponse, UpdateMessage} from "@/lib/labeq.ts";
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
const refreshing = computed(() => activeReadRequests.size > 0);

const parameters = reactive<Record<string, string>>({});
const errors = reactive<Record<string, string>>({});

const activeReadRequests = reactive<Map<string, string>>(new Map());
const activeWriteRequests = reactive<Map<string, string>>(new Map());

if (!device) {
  router.push("/");
}

const publishListener = await listen("mqtt-publish", (e: any) => {
  const requestRegex = /^device\/(?<device>.+)\/requests\/(?<request>.+)\/(?<action>.+)/;
  const updateRegex = /^device\/(?<device>.+)\/parameters\/(?<parameter>.+)\/(?<action>.+)/;
  const requestMatch = requestRegex.exec(e.payload.event.topic);
  const updateMatch = updateRegex.exec(e.payload.event.topic);
  console.log(e.payload.event.topic);
  if (requestMatch?.groups) handleRequest(requestMatch, e);
  else if (updateMatch?.groups) handleUpdate(updateMatch, e);
});

const handleRequest = (match: RegExpExecArray, e: any) => {
  if (!match.groups || match.groups.action !== 'resolve') return;
  if (!activeReadRequests.has(match.groups?.request) && !activeWriteRequests.has(match.groups?.request)) return;

  const parsedResponse = JSON.parse(new TextDecoder().decode(new Uint8Array(e.payload.event.payload))) as ReadRequestResponse;
  activeReadRequests.delete(match.groups?.request);
  activeWriteRequests.delete(match.groups?.request);
  if (parsedResponse.error) {
    errors[parsedResponse.parameter] = parsedResponse.error;
    return;
  }
  parameters[parsedResponse.parameter] = parsedResponse.resolvedValue!!;
  delete errors[parsedResponse.parameter];
};

const handleUpdate = (match: RegExpExecArray, e: any) => {
  if (!autoupdate.value || match.groups?.action !== 'update') return;
  const parsedResponse = JSON.parse(new TextDecoder().decode(new Uint8Array(e.payload.event.payload))) as UpdateMessage;
  if (parsedResponse.error) {
    errors[match.groups.parameter] = parsedResponse.error;
    return;
  }
  parameters[match.groups.parameter] = parsedResponse.newValue!!;
  delete errors[match.groups.parameter];
};

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
                activeReadRequests.set(uuid, panel.parameter);
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

const writeAllParameters = async () => {
  if (!layout.value) return;
  layout.value.rows
      .forEach(row => row.columns
          .forEach(column => column.panels
              .forEach(panel => {
                writeParameter(panel.parameter);
              })
          )
      );
};

const writeParameter = async (parameter: string) => {
  if (!layout.value) return;
  const uuid = crypto.randomUUID();
  invoke('gateway_publish', {
    connId: route.params.connectionId as string,
    topic: `device/${device.value!!.uuid}/parameters/${parameter}/write`,
    payload: JSON.stringify({
      requestId: uuid,
      contentType: 'labeq/string',
      value: parameters[parameter]
    }),
    retain: false
  });
  activeWriteRequests.set(uuid, parameter);
};

onBeforeUnmount(() => {
  publishListener();
});
</script>

<style lang="scss" scoped>

</style>`