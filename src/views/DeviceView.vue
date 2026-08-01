<template>
  <div>
    <header>
      <h2 class="text-xl">{{ device?.name }}</h2>
      <p class="text-muted-foreground">{{ device?.manufacturer }} &mdash; {{ device?.model }}</p>
      <p class="text-xs text-muted-foreground">Loaded driver: {{ device?.driver }}</p>
      <ul class="mt-2 flex gap-2 w-full">
        <li>
          <Button variant="default" class="h-6 text-xs disabled:[&_svg]:animate-bounce" @click="writeAllParameters"
                  :disabled="!layout || activeWriteRequests.size > 0 || !connectionActive">
            <Icon icon="heroicons:arrow-up-tray" width="24" height="24" v-if="activeWriteRequests.size === 0"/>
            <Icon icon="heroicons:arrow-up" width="24" height="24" v-else/>
            <span>Set all parameters</span>
          </Button>
        </li>
        <li>
          <Button variant="secondary" class="h-6 text-xs disabled:[&_svg]:animate-spin" @click="refreshParameters"
                  :disabled="!layout || activeReadRequests.size > 0 || !connectionActive">
            <Icon icon="heroicons:arrow-path" width="24" height="24"/>
            <span>Refresh parameters</span>
          </Button>
        </li>
        <li>
          <Button :variant="autoupdate ? 'secondary' : 'destructive'" class="h-6 text-xs"
                  @click.prevent="autoupdate = !autoupdate">
            <Icon v-if="autoupdate" icon="heroicons:bolt" width="24" height="24"/>
            <Icon v-else icon="heroicons:bolt-slash" width="24" height="24"/>
            <p>Auto-update</p>
          </Button>
        </li>
        <li class="ml-auto">
          <Button :variant="'ghost'" class="h-6 mr-0"
                  @click.prevent="router.push({
                              name: 'device-terminal',
                              params: { connectionId: route.params.connectionId as string, id: device?.uuid }
                            })"
          >
            <Icon icon="heroicons:command-line" class="scale-150 px-0" width="24" height="24"/>
          </Button>
        </li>
      </ul>
    </header>
    <Separator class="mt-2 mb-2"/>
    <Alert variant="destructive" class="mt-4 mb-2" v-if="!connectionActive">
      <Icon
          icon="heroicons:exclamation-triangle-16-solid"
          class="text-destructive p-0"
          width="16"
          height="16"
      />
      <AlertTitle>Connection inactive!</AlertTitle>
      <AlertDescription>Activate the connection in order to use this panel.</AlertDescription>
    </Alert>
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
                <DevicePanel
                    :id="`${device?.uuid},${panel.parameter}`"
                    :definition="panel"
                    v-model:data="parameters[panel.parameter].value"
                    v-model:error="errors[panel.parameter]"
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
import {computed, onBeforeUnmount, onMounted, provide, ref, watch} from "vue";
import {LayoutSchema} from "@/lib/plugin.ts";
import DevicePanel from "@/components/monitor/device/DevicePanel.vue";
import {Label} from "@/components/ui/label";
import {Separator} from "@/components/ui/separator";
import {Tooltip, TooltipContent, TooltipTrigger} from "@/components/ui/tooltip";
import {Icon} from "@iconify/vue";
import usePlugin from "@/composables/usePlugin.ts";
import useDevice from "@/composables/useDevice.ts";
import {Button} from "@/components/ui/button";
import useDeviceParameters from "@/composables/useDeviceParameters.ts";
import {useConnectionStore} from "@/store/connection.store.ts";
import {Alert, AlertDescription, AlertTitle} from "@/components/ui/alert";

const route = useRoute();
const router = useRouter();

let layout = ref<LayoutSchema | null>(null);

const connectionStore = useConnectionStore()
const device = useDevice();
const {
  parameters,
  errors,
  refreshing,
  autoupdate,
  refreshParameters,
  writeAllParameters,
  activeWriteRequests,
  activeReadRequests,
  publishListener
} = useDeviceParameters(device, layout);

const connectionActive = computed(() => connectionStore.connections.get(route.params.connectionId as string)?.state === 'Connected');

provide('device', device);
provide('plugin', device.value ? usePlugin(device.value) : null);

if (!device) {
  router.push("/");
}

onMounted(async () => {
  if (!device.value) return;

  try {
    layout.value = await invoke<LayoutSchema>('load_layout', {
      driver: device.value.driver,
      variant: device.value.model
    });

    if (layout.value?.rows) {
      for (const row of layout.value.rows) {
        for (const column of row.columns) {
          for (const panel of column.panels) {
            if (!parameters[panel.parameter]) {
              parameters[panel.parameter] = {
                paramType: '',
                value: panel.default
              };
            }
          }
        }
      }
    }
  } catch (error) {
    console.error("Failed to load layout configuration:", error);
  }
});

const fetchDeviceData = async () => {
  if (!device.value) return;

  try {
    await invoke('update_device_parameters', {
      connId: route.params.connectionId,
      deviceId: device.value.uuid
    });

    await refreshParameters();
  } catch (error) {
    console.error("Failed to sync device parameters over active connection:", error);
  }
};

watch(connectionActive, async (isActive) => {
  if (isActive) {
    await fetchDeviceData();
  }
}, {immediate: true});

onBeforeUnmount(async () => {
  (await publishListener)();
});
</script>

<style lang="scss" scoped>

</style>`