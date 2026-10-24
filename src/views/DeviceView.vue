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
                              params: { connectionId: props.connectionId, id: device?.uuid }
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

    <article class="flex flex-col">
      <div v-for="(row, i) in layout?.rows ?? []" :key="i" class="my-2">

        <!-- Group Header with group update & refresh icons -->
        <div class="flex items-center gap-2 mb-1" v-if="row.groupName">
          <h3 class="text-lg">{{ row.groupName }}</h3>
          <div class="flex items-center gap-1">
            <Button
                v-if="isGroupDirty(row)"
                variant="ghost"
                class="h-6 w-6 p-0 text-primary hover:text-primary/80 transition-colors animate-in fade-in"
                @click="writeGroupParameters(row)"
                title="Write group parameters to device"
            >
              <Icon icon="heroicons:arrow-up-tray" width="18" height="18"/>
            </Button>
            <Button
                variant="ghost"
                class="h-6 w-6 p-0 text-muted-foreground hover:text-foreground transition-colors"
                @click="readGroupParameters(row)"
                title="Refresh group parameters"
            >
              <Icon icon="heroicons:arrow-path" width="18" height="18"/>
            </Button>
          </div>
        </div>

        <div class="grid gap-4 place-self-start"
             :style="{ gridTemplateColumns: `repeat(${row.columns.length || 1}, minmax(0, 1fr))` }">
          <div v-for="(column, j) in row.columns" :key="j" class="flex flex-col">
            <div v-for="(panel, k) in column.panels" :key="k" class="min-w-0">
              <div class="flex items-center gap-2 mt-2 h-4">
                <Label
                    :for="`${device?.uuid},${panel.parameter}`"
                    class="whitespace-nowrap overflow-hidden text-ellipsis block max-w-full leading-none"
                >
                  {{ panel.displayName || panel.parameter }}
                </Label>

                <div class="flex items-center justify-center shrink-0 gap-1 h-4">
                  <Button
                      v-if="panel.kind !== 'action'"
                      variant="ghost"
                      class="h-5 w-5 p-0 text-muted-foreground hover:text-foreground transition-colors"
                      @click="readParameter(panel.parameter)"
                      title="Refresh parameter"
                      :disabled="isParameterRefreshing(panel.parameter).value"
                  >
                    <Icon
                        icon="heroicons:arrow-path"
                        width="14"
                        height="14"
                        :class="{ 'animate-spin opacity-50': isParameterRefreshing(panel.parameter).value }"
                    />
                  </Button>

                  <Button
                      v-if="dirtyParameters[panel.parameter] && panel.kind !== 'action'"
                      variant="ghost"
                      class="h-5 w-5 p-0 text-primary hover:text-primary/80 transition-colors animate-in fade-in"
                      @click="writeParameter(panel.parameter)"
                      title="Write to device"
                  >
                    <Icon icon="heroicons:arrow-up-tray" width="16" height="16"/>
                  </Button>

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
                    v-if="parameters[panel.parameter]"
                    :id="`${device?.uuid},${panel.parameter}`"
                    :definition="panel"
                    v-model:data="parameters[panel.parameter].value"
                    v-model:error="errors[panel.parameter]"
                    :processing="isParameterProcessing(panel.parameter).value"
                    :refreshing="isParameterRefreshing(panel.parameter).value"
                    @dirty="dirtyParameters[panel.parameter] = true"
                    @write="writeParameter(panel.parameter)"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </article>
  </div>
</template>

<script setup lang="ts">
import {useRouter} from "vue-router";
import {invoke} from "@tauri-apps/api/core";
import {computed, onBeforeUnmount, onMounted, provide, ref, watch} from "vue";
import {LayoutRow, LayoutSchema} from "@/lib/plugin.ts";
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

const props = defineProps<{ connectionId: string, id: string }>();
const router = useRouter();
let layout = ref<LayoutSchema | null>(null);

const connectionStore = useConnectionStore();
const device = useDevice(props.connectionId, props.id);

const {
  parameters,
  errors,
  autoupdate,
  refreshParameters,
  readParameter,
  writeParameter,
  writeAllParameters,
  activeWriteRequests,
  activeReadRequests,
  publishListener,
  isParameterProcessing,
  isParameterRefreshing,
  dirtyParameters
} = useDeviceParameters(props.connectionId, device, layout);

const connectionActive = computed(() => connectionStore.connections.get(props.connectionId)?.state === 'Connected');

provide('device', device);
provide('plugin', device.value ? usePlugin(device.value) : null);

if (!device) router.push("/");

const isGroupDirty = (row: LayoutRow) => {
  return row.columns.some(col =>
      col.panels.some(panel => dirtyParameters[panel.parameter])
  );
};

const writeGroupParameters = (row: LayoutRow) => {
  row.columns.forEach(col => {
    col.panels.forEach(panel => {
      if (dirtyParameters[panel.parameter] && panel.kind !== 'action') {
        writeParameter(panel.parameter);
      }
    });
  });
};

const readGroupParameters = (row: LayoutRow) => {
  row.columns.forEach(col => {
    col.panels.forEach(panel => {
      if (panel.kind !== 'action') {
        readParameter(panel.parameter);
      }
    });
  });
};

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
              parameters[panel.parameter] = {paramType: '', value: panel.default};
            }
          }
        }
      }
    }
  } catch (error) {
    console.error("Failed to load layout configuration:", error);
  }
});

const hasInitiallyFetched = ref(false);

const fetchDeviceData = async () => {
  if (!device.value) return;
  try {
    await invoke('update_device_parameters', {
      connId: props.connectionId,
      deviceId: device.value.uuid
    });
    await refreshParameters();
  } catch (error) {
    console.error("Failed to sync device parameters over active connection:", error);
  }
};

watch(connectionActive, async (isActive) => {
  if (isActive && !hasInitiallyFetched.value) {
    hasInitiallyFetched.value = true;
    await fetchDeviceData();
  }
}, {immediate: true});

onBeforeUnmount(async () => {
  (await publishListener)();
});
</script>

<style lang="scss" scoped></style>
