<script setup lang="ts">
import { computed, ref, onBeforeUnmount } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import useDevice from "@/composables/useDevice.ts";
import usePlugin from "@/composables/usePlugin.ts";
import useDeviceParameters from "@/composables/useDeviceParameters.ts";
import { useConnectionStore } from "@/store/connection.store.ts";
import { toast } from "vue-sonner";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Table, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Icon } from "@iconify/vue";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";

const props = defineProps<{ connectionId: string, id: string }>();
const router = useRouter();

const device = useDevice(props.connectionId, props.id);

if (!device.value) {
  router.push("/");
}

const {
  parameters,
  errors,
  readParameter,
  writeParameter,
  isParameterRefreshing,
  isParameterProcessing,
  publishListener
} = useDeviceParameters(props.connectionId, device, ref(null));

const connectionStore = useConnectionStore();
const connection = computed(() => connectionStore.connections.get(props.connectionId));

let plugin = usePlugin(device.value!!);

const isEditDialogOpen = ref(false);
const editingParamName = ref("");
const editingParamValue = ref<string | number>("");

const openEditDialog = (name: string, currentValue: any) => {
  editingParamName.value = name;
  editingParamValue.value = currentValue;
  isEditDialogOpen.value = true;
};

const saveParameter = async () => {
  if (editingParamName.value && parameters[editingParamName.value]) {
    parameters[editingParamName.value].value = String(editingParamValue.value);

    await writeParameter(editingParamName.value);

    toast.info(`Dispatched update for ${editingParamName.value}`);
    isEditDialogOpen.value = false;
  }
};

// Cleanup MQTT listener when leaving the terminal
onBeforeUnmount(async () => {
  (await publishListener)();
});
</script>

<template>
  <div class="grid gap-y-4">
    <header>
      <h2 class="text-xl">{{ device?.name }}</h2>
      <p class="text-muted-foreground">{{ device?.manufacturer }} &mdash; {{ device?.model }}</p>
      <p class="text-xs text-muted-foreground">Loaded driver: {{ device?.driver }}</p>
    </header>

    <div class="grid grid-cols-2 gap-4">
      <Card class="py-4 gap-2">
        <CardHeader>
          <CardTitle>Device information</CardTitle>
        </CardHeader>
        <CardContent>
          <dl class="param-container">
            <dt>Connection</dt>
            <dd>{{ connection?.name }}</dd>

            <dt>Name</dt>
            <dd>{{ device?.name }}</dd>

            <dt>Manufacturer</dt>
            <dd>{{ device?.manufacturer }}</dd>

            <dt>Model</dt>
            <dd>{{ device?.model }}</dd>

            <dt>UUID</dt>
            <dd>{{ device?.uuid }}</dd>
          </dl>
        </CardContent>
      </Card>
      <Card class="py-4 gap-2">
        <CardHeader>
          <CardTitle>Driver information</CardTitle>
        </CardHeader>
        <CardContent>
          <dl class="param-container">
            <dt>Driver</dt>
            <dd>{{ device?.driver }}</dd>

            <dt>Plugin</dt>
            <dd>{{ plugin?.name }}</dd>

            <dt>Plugin ID</dt>
            <dd>{{ plugin?.id }}</dd>

            <dt>Plugin Version</dt>
            <dd>{{ plugin?.version }}</dd>

            <dt>Plugin Author</dt>
            <dd>{{ plugin?.author }}</dd>
          </dl>
        </CardContent>
      </Card>
    </div>

    <Card class="gap-2">
      <CardHeader>
        <CardTitle>Parameters</CardTitle>
      </CardHeader>
      <CardContent>
        <Table class="text-xs w-full">
          <TableHeader>
            <TableRow>
              <TableHead class="w-[80px] lg:table-cell hidden" />
              <TableHead>Name</TableHead>
              <TableHead>Type</TableHead>
              <TableHead>Value</TableHead>
              <TableHead>Status</TableHead>
            </TableRow>
          </TableHeader>
          <template v-for="[name, parameter] in Object.entries(parameters).sort()" :key="name">
            <TableRow>
              <TableCell class="lg:table-cell hidden">
                <div class="flex gap-1 items-center">
                  <Button
                      @click.prevent="readParameter(name)"
                      variant="ghost"
                      class="p-0 w-8 h-8 text-muted-foreground hover:text-foreground transition-colors"
                      :disabled="isParameterProcessing(name).value"
                  >
                    <Icon
                        icon="heroicons:arrow-path"
                        width="18"
                        height="18"
                        :class="{ 'animate-spin opacity-50': isParameterRefreshing(name).value }"
                    />
                  </Button>
                  <Button
                      v-if="parameter"
                      @click.prevent="openEditDialog(name, parameter.value)"
                      variant="ghost"
                      class="p-0 w-8 h-8 text-muted-foreground hover:text-foreground transition-colors"
                      :disabled="isParameterProcessing(name).value"
                  >
                    <Icon icon="heroicons:pencil" width="18" height="18" />
                  </Button>
                </div>
              </TableCell>
              <TableCell class="text-muted-foreground font-mono">{{ name }}</TableCell>
              <TableCell>{{ parameter.paramType }}</TableCell>
              <TableCell>
                <span :class="{ 'opacity-50 transition-opacity': isParameterProcessing(name).value }">
                  {{ parameter.value }}
                </span>
              </TableCell>
              <TableCell>
                <!-- Real-time parameter state tracking explicitly showing the error content -->
                <span v-if="isParameterProcessing(name).value" class="text-muted-foreground flex items-center gap-1 text-xs">
                  <Icon icon="heroicons:arrow-path" class="animate-spin" width="14" height="14" /> Processing
                </span>
                <span v-else-if="errors[name]" class="text-destructive flex flex-col gap-0.5 text-xs">
                  <div class="flex items-center gap-1">
                    <Icon icon="heroicons:exclamation-triangle" width="14" height="14" /> Error
                  </div>
                  <span class="text-[10px] opacity-80 whitespace-normal">{{ errors[name] }}</span>
                </span>
                <span v-else class="text-green-500/80 flex items-center gap-1 text-xs">
                  <Icon icon="heroicons:check-circle" width="14" height="14" /> OK
                </span>
              </TableCell>
            </TableRow>
          </template>
        </Table>
      </CardContent>
    </Card>

    <!-- Edit Parameter Dialog -->
    <Dialog v-model:open="isEditDialogOpen">
      <DialogContent class="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Edit Parameter</DialogTitle>
        </DialogHeader>
        <div class="grid gap-4 py-4">
          <div class="flex flex-col gap-2">
            <Label for="param-value" class="text-xs text-muted-foreground font-mono">
              {{ editingParamName }}
            </Label>
            <Input
                id="param-value"
                v-model="editingParamValue"
                @keyup.enter="saveParameter"
                class="w-full"
                autocomplete="off"
            />
          </div>
        </div>
        <DialogFooter>
          <Button variant="secondary" @click="isEditDialogOpen = false">Cancel</Button>
          <Button @click="saveParameter">Dispatch Update</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<style scoped>
@reference "@/index.css";

dl {
  @apply grid grid-cols-[auto_1fr] gap-x-4 gap-y-1.5 text-xs;
}

dt {
  @apply text-muted-foreground;
}

dd {
  @apply text-foreground whitespace-nowrap overflow-hidden overflow-ellipsis;
}
</style>
