<script setup lang="ts">
import {useConnectionStore} from "@/store/connection.store.ts";
import {useRoute, useRouter} from "vue-router";
import {Separator} from "@/components/ui/separator";
import {computed, onBeforeUnmount, onMounted, ref, watch} from "vue";
import {Tabs, TabsContent, TabsList, TabsTrigger} from "@/components/ui/tabs";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table";
import {Badge} from "@/components/ui/badge";
import {Icon} from "@iconify/vue"
import {Button} from "@/components/ui/button";
import {Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle} from "@/components/ui/dialog";
import {Eye, Play, Square} from "lucide-vue-next";

const route = useRoute();
const router = useRouter();

if (!route.params.connectionId) {
  router.push("/");
}

const connectionStore = useConnectionStore();
const connection = computed(() => connectionStore.connections.get(route.params.connectionId as string));

if (!connection) {
  router.push("/");
}

interface GatewayScript {
  name: string;
  description: string;
  file: string;
  runner: string;
}

interface ScriptExecution {
  executionId: string;
  script: GatewayScript;
  status: string;
  heartbeatId: number;
  lastHeartbeatAt: number | null;
}

interface GatewayRunner {
  name: string;
  extensions: string[];
  commandTemplate: string[];
  workingDirectory: string;
  env: Record<string, string>;
}

const scripts = ref<GatewayScript[]>([]);
const runners = ref<GatewayRunner[]>([]);
const executions = ref<ScriptExecution[]>([]);
const selectedExecution = ref<ScriptExecution | null>(null);
const isRunDialogOpen = ref(false);
const scriptsError = ref<string | null>(null);
let refreshTimer: ReturnType<typeof setInterval> | undefined;

const gatewayUrl = computed(() => connection.value?.apiUrl.replace(/\/+$/, ""));
const executionFor = (script: GatewayScript) => executions.value.find((execution) => execution.script.name === script.name);
const mqttStatusFor = (script: GatewayScript) => {
  const activeExecution = executionFor(script);
  return connection.value?.scripts
    .slice()
    .reverse()
    .find((execution) => activeExecution
      ? execution.executionId === activeExecution.executionId
      : execution.script === script.name);
};
const detailsFor = (script: GatewayScript): ScriptExecution => executionFor(script) ?? {
  executionId: mqttStatusFor(script)?.executionId ?? "—",
  script,
  status: mqttStatusFor(script)?.status ?? "IDLE",
  heartbeatId: mqttStatusFor(script)?.heartbeatId ?? 0,
  lastHeartbeatAt: mqttStatusFor(script)?.lastHeartbeatAt ?? null,
};
const displayStatusFor = (script: GatewayScript) => mqttStatusFor(script)?.status ?? executionFor(script)?.status ?? "IDLE";

const refreshScripts = async () => {
  if (!gatewayUrl.value) return;
  try {
    const [scriptResponse, runnerResponse, executionResponse] = await Promise.all([
      fetch(`${gatewayUrl.value}/scripts`),
      fetch(`${gatewayUrl.value}/runners`),
      fetch(`${gatewayUrl.value}/scripts/executions`),
    ]);
    if (!scriptResponse.ok || !runnerResponse.ok || !executionResponse.ok) {
      scriptsError.value = "Gateway did not return script data";
      return;
    }
    scripts.value = await scriptResponse.json() as GatewayScript[];
    runners.value = await runnerResponse.json() as GatewayRunner[];
    executions.value = await executionResponse.json() as ScriptExecution[];
    if (selectedExecution.value) {
      selectedExecution.value = executions.value.find((execution) => execution.executionId === selectedExecution.value?.executionId) ?? selectedExecution.value;
    }
    scriptsError.value = null;
  } catch (error) {
    scriptsError.value = error instanceof Error ? error.message : "Could not load scripts";
  }
};

const openExecution = (execution: ScriptExecution | undefined) => {
  if (!execution) return;
  selectedExecution.value = execution;
  isRunDialogOpen.value = true;
};

const startScript = async (script: GatewayScript) => {
  if (!gatewayUrl.value) return;
  try {
    const response = await fetch(`${gatewayUrl.value}/scripts/${encodeURIComponent(script.name)}/start`, {method: "POST"});
    if (!response.ok) {
      scriptsError.value = `Could not start ${script.name}`;
      return;
    }
    await refreshScripts();
  } catch (error) {
    scriptsError.value = error instanceof Error ? error.message : "Could not start script";
  }
};

const stopScript = async (execution: ScriptExecution) => {
  if (!gatewayUrl.value) return;
  try {
    const response = await fetch(`${gatewayUrl.value}/scripts/executions/${execution.executionId}/stop`, {method: "POST"});
    if (!response.ok) {
      scriptsError.value = `Could not stop ${execution.script.name}`;
      return;
    }
    await refreshScripts();
    openExecution(executionFor(execution.script));
  } catch (error) {
    scriptsError.value = error instanceof Error ? error.message : "Could not stop script";
  }
};

onMounted(async () => {
  await refreshScripts();
  refreshTimer = setInterval(refreshScripts, 2000);
});

watch(gatewayUrl, refreshScripts);
watch(
  () => connection.value?.scripts,
  (statuses) => {
    if (!selectedExecution.value || !statuses) return;
    const status = statuses.find((item) => item.executionId === selectedExecution.value?.executionId);
    if (status) {
      selectedExecution.value = {
        ...selectedExecution.value,
        status: status.status,
        heartbeatId: status.heartbeatId ?? selectedExecution.value.heartbeatId,
        lastHeartbeatAt: status.lastHeartbeatAt ?? selectedExecution.value.lastHeartbeatAt,
      };
    }
  },
  {deep: true},
);
onBeforeUnmount(() => {
  if (refreshTimer) clearInterval(refreshTimer);
});

</script>

<template>
  <div>
    <header>
      <h2 class="text-xl">{{ connection?.name }}</h2>
      <p class="text-muted-foreground" v-if="connection?.state === 'Disconnected'">{{ connection?.state }}</p>
      <p class="text-positive" v-if="connection?.state === 'Connected'">{{ connection?.state }}</p>
      <p class="text-attention" v-if="connection?.state === 'Connecting'">{{ connection?.state }}</p>
      <p class="text-destructive" v-if="connection?.state === 'ConnectionError'">Connection error</p>
    </header>
    <Separator class="mt-2 mb-2"/>
    <Tabs default-value="devices">
      <TabsList>
        <TabsTrigger value="devices">Devices</TabsTrigger>
        <TabsTrigger value="scripts">Scripts</TabsTrigger>
        <TabsTrigger value="runners">Runners</TabsTrigger>
      </TabsList>
      <TabsContent value="devices">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>Manufacturer</TableHead>
              <TableHead>Model</TableHead>
              <TableHead>Tags</TableHead>
              <TableHead>Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="device in connection?.devices" :key="device.uuid">
              <TableCell>{{ device.name }}</TableCell>
              <TableCell>{{ device.manufacturer }}</TableCell>
              <TableCell>{{ device.model }}</TableCell>
              <TableCell>
                <Badge v-for="tag in device.tags" :key="tag">{{ tag }}</Badge>
              </TableCell>
              <TableCell>
                <router-link :to="`/connection/${connection?.id}/devices/${device.uuid}`">
                  <Icon icon="heroicons:arrow-top-right-on-square" width="16" height="16"></Icon>
                </router-link>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </TabsContent>
      <TabsContent value="scripts">
        <div v-if="scriptsError" class="text-sm text-destructive">{{ scriptsError }}</div>
        <Table v-else>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>Description</TableHead>
              <TableHead>Runner</TableHead>
              <TableHead>Status</TableHead>
              <TableHead class="text-right">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="script in scripts" :key="script.name">
              <TableCell>{{ script.name }}</TableCell>
              <TableCell class="text-muted-foreground">{{ script.description || "—" }}</TableCell>
              <TableCell><Badge variant="outline">{{ script.runner || "—" }}</Badge></TableCell>
              <TableCell>
                <Badge :variant="displayStatusFor(script) === 'CRASHED' ? 'destructive' : displayStatusFor(script) === 'IDLE' ? 'outline' : 'default'">
                  {{ displayStatusFor(script) }}
                </Badge>
              </TableCell>
              <TableCell>
                <div class="flex justify-end gap-2">
                  <Button variant="outline" size="sm" @click="openExecution(detailsFor(script))">
                    <Eye/> Details
                  </Button>
                  <Button :disabled="!executionFor(script)" variant="destructive" size="sm" @click="stopScript(executionFor(script)!)">
                    <Square/> Stop
                  </Button>
                  <Button size="sm" @click="startScript(script)">
                    <Play/> Run
                  </Button>
                </div>
              </TableCell>
            </TableRow>
            <TableRow v-if="scripts.length === 0">
              <TableCell colspan="5" class="py-8 text-center text-muted-foreground">No scripts found in the gateway.</TableCell>
            </TableRow>
          </TableBody>
        </Table>

        <Dialog v-model:open="isRunDialogOpen">
          <DialogContent v-if="selectedExecution">
            <DialogHeader>
              <DialogTitle>{{ selectedExecution.script.name }}</DialogTitle>
              <DialogDescription>{{ selectedExecution.script.description || "Script execution details" }}</DialogDescription>
            </DialogHeader>
            <div class="grid gap-3 text-sm">
              <div class="flex justify-between border-b pb-2"><span class="text-muted-foreground">Status</span><Badge>{{ selectedExecution.status }}</Badge></div>
              <div class="flex justify-between border-b pb-2"><span class="text-muted-foreground">Execution ID</span><code class="max-w-[18rem] truncate">{{ selectedExecution.executionId }}</code></div>
              <div class="flex justify-between border-b pb-2"><span class="text-muted-foreground">Runner</span><span>{{ selectedExecution.script.runner }}</span></div>
              <div class="flex justify-between"><span class="text-muted-foreground">Heartbeat</span><span>{{ selectedExecution.heartbeatId }}</span></div>
              <div class="flex justify-between"><span class="text-muted-foreground">Last heartbeat</span><span>{{ selectedExecution.lastHeartbeatAt ? new Date(selectedExecution.lastHeartbeatAt).toLocaleString() : "—" }}</span></div>
            </div>
            <DialogFooter>
              <Button v-if="selectedExecution.status !== 'FINISHED' && selectedExecution.status !== 'CRASHED' && selectedExecution.status !== 'STOPPED'" variant="destructive" @click="stopScript(selectedExecution)">
                <Square/> Stop
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </TabsContent>
      <TabsContent value="runners">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>Extensions</TableHead>
              <TableHead>Status</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="runner in runners" :key="runner.name">
              <TableCell class="font-medium">{{ runner.name }}</TableCell>
              <TableCell>{{ runner.extensions.join(", ") || "—" }}</TableCell>
              <TableCell><Badge variant="outline">Available</Badge></TableCell>
            </TableRow>
            <TableRow v-if="runners.length === 0">
              <TableCell colspan="3" class="py-8 text-center text-muted-foreground">No runners found in the gateway.</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </TabsContent>
    </Tabs>
  </div>
</template>

<style scoped lang="scss">

</style>