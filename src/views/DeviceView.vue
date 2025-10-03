<template>
  <div>
    <header>
      <h2 class="text-xl">{{ device.name }}</h2>
      <p class="text-muted-foreground">{{ device?.manufacturer }} &mdash; {{ device.model }}</p>
      <p class="text-xs text-muted-foreground font-semibold">Loaded driver: {{ device.driver }}</p>
    </header>
    <article
        class="grid"
        :style="{
          gridTemplateRows: `repeat(${layout?.rows.length || 1}, 1fr)`
        }"
    >
      <div v-for="(row, i) in layout?.rows ?? []" :key="i" class="grid" :style="{
        gridTemplateColumns: `repeat(${row.columns.length || 1}, 1fr)`
      }">
        <div v-for="(column, j) in row.columns" :key="j" class="flex flex-col">
          <div v-for="(panel, i) in column.panels" :key="i">
            <DevicePanel :definition="panel" :data="123"/>
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
import {onMounted, ref} from "vue";
import {LabeqPluginManifest, LayoutSchema} from "@/lib/plugin.ts";
import DevicePanel from "@/components/monitor/device/DevicePanel.vue";

const connections = useConnectionStore()
const route = useRoute()
const router = useRouter()

const device = connections.connections
    .get(route.params.connectionId as string)?.devices
    .find(d => d.uuid === route.params.id)!!

if (!device) {
  router.push("/");
}

let plugin = ref<LabeqPluginManifest | null>(null);
let layout = ref<LayoutSchema | null>(null);

onMounted(async () => {
  plugin.value = (await invoke<string[]>('list_plugins')).map(d => JSON.parse(d)).find(plugin => plugin.drivers.includes(device.driver));
  layout.value = await invoke<LayoutSchema>('load_layout', {driver: device.driver, variant: device.model})
});
</script>

<style lang="scss" scoped>

</style>`