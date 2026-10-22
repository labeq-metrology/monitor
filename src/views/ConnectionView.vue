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
import {ScrollArea} from "@/components/ui/scroll-area";
import {
  Trace,
  TraceDescription,
  TraceIcon,
  TraceLine,
  TraceTitle,
} from "@/components/ui/trace";
import {AlertCircle, CheckCircle2, CircleDashed, Info, LoaderCircle, TriangleAlert} from "lucide-vue-next";

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
  startedAt: number;
  log: string[];
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
const runsFor = (script: GatewayScript) => executions.value
  .filter((execution) => execution.script.name === script.name)
  .sort((left, right) => right.startedAt - left.startedAt);
const executionFor = (script: GatewayScript) => runsFor(script)
  .find((execution) => !["FINISHED", "CRASHED", "STOPPED"].includes(execution.status));
const latestRunFor = (script: GatewayScript) => runsFor(script)[0];
const allRuns = computed(() => executions.value.slice().sort((left, right) => right.startedAt - left.startedAt));
const terminalStatuses = ["FINISHED", "CRASHED", "STOPPED"];
interface FormattedLogEntry {
  timestamp: string;
  thread: string;
  level: string;
  message: string;
}
const formatLogEntry = (entry: string): FormattedLogEntry => {
  const match = entry.match(/^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3})(?: \[([^\]]+)])? (\w+)\s+-\s?([\s\S]*)$/);
  if (!match) {
    return {timestamp: "", thread: "", level: "", message: entry};
  }
  return {
    timestamp: match[1],
    thread: match[2] ?? "",
    level: match[3],
    message: match[4],
  };
};
const formattedLogEntries = computed(() => {
  if (!selectedExecution.value) return [];
  return selectedExecution.value.log.map(formatLogEntry);
});
const logStatusFor = (entry: FormattedLogEntry): "running" | "error" | "pending" | undefined => {
  const level = entry.level;
  if (level === "ERROR") return "error";
  return undefined;
};
const logIconFor = (entry: FormattedLogEntry) => {
  const level = entry.level;
  if (level === "ERROR") return AlertCircle;
  if (level === "WARN") return TriangleAlert;
  return Info;
};
const logClassFor = (entry: FormattedLogEntry) => {
  if (entry.level === "ERROR") {
    return "rounded-md border border-destructive/30 bg-destructive/10 px-3 text-destructive";
  }
  if (entry.level === "WARN") {
    return "rounded-md border border-attention/40 bg-attention/10 px-3 text-attention";
  }
  return "rounded-md px-3";
};
const statusIconFor = (status: string) => {
  if (status === "FINISHED") return CheckCircle2;
  if (status === "CRASHED") return AlertCircle;
  if (terminalStatuses.includes(status)) return CircleDashed;
  return LoaderCircle;
};
const mqttStatusFor = (script: GatewayScript) => {
  const activeExecution = executionFor(script);
  return connection.value?.scripts
    .slice()
    .reverse()
    .find((execution) => activeExecution
      ? execution.executionId === activeExecution.executionId
      : execution.script === script.name);
};
const detailsFor = (script: GatewayScript): ScriptExecution => latestRunFor(script) ?? {
  executionId: mqttStatusFor(script)?.executionId ?? "—",
  script,
  status: mqttStatusFor(script)?.status ?? "IDLE",
  heartbeatId: mqttStatusFor(script)?.heartbeatId ?? 0,
  lastHeartbeatAt: mqttStatusFor(script)?.lastHeartbeatAt ?? null,
  startedAt: Date.now(),
  log: [],
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
    openExecution(executions.value.find((item) => item.executionId === execution.executionId));
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
              <TableHead>Runs</TableHead>
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
              <TableCell>{{ runsFor(script).length }}</TableCell>
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
              <TableCell colspan="6" class="py-8 text-center text-muted-foreground">No scripts found in the gateway.</TableCell>
            </TableRow>
          </TableBody>
        </Table>

        <div class="mt-6 space-y-3">
          <div>
            <h3 class="font-medium">All runs</h3>
            <p class="text-sm text-muted-foreground">Every execution started for the scripts on this gateway.</p>
          </div>
          <Table v-if="allRuns.length">
            <TableHeader>
              <TableRow>
                <TableHead>Script</TableHead>
                <TableHead>Started</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Run ID</TableHead>
                <TableHead class="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow v-for="run in allRuns" :key="run.executionId">
                <TableCell class="font-medium">{{ run.script.name }}</TableCell>
                <TableCell>{{ new Date(run.startedAt).toLocaleString() }}</TableCell>
                <TableCell>
                  <Badge :variant="run.status === 'CRASHED' ? 'destructive' : run.status === 'IDLE' ? 'outline' : 'default'">
                    <component :is="statusIconFor(run.status)" class="mr-1 size-3.5"/>
                    {{ run.status }}
                  </Badge>
                </TableCell>
                <TableCell><code class="text-xs">{{ run.executionId.slice(0, 8) }}</code></TableCell>
                <TableCell class="text-right">
                  <Button variant="outline" size="sm" @click="openExecution(run)">
                    <Eye/> View run
                  </Button>
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
          <div v-else class="rounded-md border py-8 text-center text-sm text-muted-foreground">
            No runs have been started yet.
          </div>
        </div>

        <Dialog v-model:open="isRunDialogOpen">
          <DialogContent v-if="selectedExecution" class="w-[min(90vw,56rem)] max-w-none max-h-[90vh] overflow-hidden">
            <DialogHeader>
              <DialogTitle>{{ selectedExecution.script.name }}</DialogTitle>
              <DialogDescription>
                Run {{ selectedExecution.executionId }} · {{ selectedExecution.script.description || "Script execution details" }}
              </DialogDescription>
            </DialogHeader>
            <Tabs default-value="details" class="min-w-0">
              <TabsList>
                <TabsTrigger value="details">Details</TabsTrigger>
                <TabsTrigger value="log">Log ({{ selectedExecution.log.length }})</TabsTrigger>
              </TabsList>
              <TabsContent value="details" class="grid min-w-0 gap-3 text-sm">
                <div class="flex justify-between border-b pb-2"><span class="text-muted-foreground">Status</span><Badge>{{ selectedExecution.status }}</Badge></div>
                <div class="flex min-w-0 justify-between gap-4 border-b pb-2"><span class="text-muted-foreground">Execution ID</span><code class="min-w-0 break-all text-right">{{ selectedExecution.executionId }}</code></div>
                <div class="flex justify-between border-b pb-2"><span class="text-muted-foreground">Started</span><span>{{ new Date(selectedExecution.startedAt).toLocaleString() }}</span></div>
                <div class="flex min-w-0 justify-between gap-4 border-b pb-2"><span class="text-muted-foreground">Runner</span><span class="min-w-0 break-all text-right">{{ selectedExecution.script.runner }}</span></div>
                <div class="flex justify-between"><span class="text-muted-foreground">Heartbeat</span><span>{{ selectedExecution.heartbeatId }}</span></div>
                <div class="flex justify-between"><span class="text-muted-foreground">Last heartbeat</span><span>{{ selectedExecution.lastHeartbeatAt ? new Date(selectedExecution.lastHeartbeatAt).toLocaleString() : "—" }}</span></div>
              </TabsContent>
              <TabsContent value="log">
                <ScrollArea class="h-[min(55vh,28rem)] min-w-0 rounded-md border bg-muted/30 p-3">
                  <Trace v-if="selectedExecution.log.length" class="gap-1">
                    <TraceLine v-for="(entry, index) in formattedLogEntries" :key="`${selectedExecution.executionId}-${index}`" :status="logStatusFor(entry)" :class="[logClassFor(entry), 'items-start gap-3 py-2 font-mono text-xs']">
                      <TraceIcon class="mt-0.5"><component :is="logIconFor(entry)" class="size-3.5"/></TraceIcon>
                      <div class="min-w-0 flex-1 space-y-1">
                        <div class="flex flex-wrap items-center gap-2">
                          <TraceDescription v-if="entry.timestamp" class="text-[11px]">{{ entry.timestamp }}</TraceDescription>
                          <Badge v-if="entry.level" variant="outline" :class="entry.level === 'ERROR' ? 'border-destructive/50 text-destructive' : entry.level === 'WARN' ? 'border-attention/60 text-attention' : ''" class="text-[10px]">{{ entry.level }}</Badge>
                          <span v-if="entry.thread" class="truncate text-[11px] opacity-70">{{ entry.thread }}</span>
                        </div>
                        <TraceTitle class="!block !whitespace-pre-wrap !overflow-visible !text-clip break-all text-foreground">{{ entry.message }}</TraceTitle>
                      </div>
                    </TraceLine>
                  </Trace>
                  <p v-else class="text-sm text-muted-foreground">No log output.</p>
                </ScrollArea>
              </TabsContent>
            </Tabs>
            <DialogFooter>
              <Button v-if="!terminalStatuses.includes(selectedExecution.status)" variant="destructive" @click="stopScript(selectedExecution)">
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