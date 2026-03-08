<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import useDevice from "@/composables/useDevice.ts";
import usePlugin from "@/composables/usePlugin.ts";
import {computed, ref} from "vue";
import {LayoutSchema} from "@/lib/plugin.ts";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card";
import {useConnectionStore} from "@/store/connection.store.ts";
import {Table, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table";

const route = useRoute();

const device = useDevice();
const connectionStore = useConnectionStore();
const connection = computed(() => connectionStore.connections.get(route.params.connectionId as string));

let plugin = usePlugin(device.value!!);
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
        <Table class="text-xs w-full ">
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>Type</TableHead>
              <TableHead>Value</TableHead>
            </TableRow>
          </TableHeader>
          <template v-for="[name, parameter] in Object.entries(device?.parameters ?? {}).sort()">
            <TableRow>
              <TableCell class="text-muted-foreground">{{ name }}</TableCell>
              <TableCell>{{ parameter.paramType }}</TableCell>
              <TableCell>{{ parameter.value }}</TableCell>
            </TableRow>
          </template>
        </Table>
      </CardContent>
    </Card>
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