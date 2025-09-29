<template>
  <div>
    <header>
      <h2>Installed plugins</h2>
    </header>
    <section class="grid grid-cols-2 gap-4">
      <article>
        <DataTable :columns="columns" :data="plugins"/>
      </article>
      <article></article>
    </section>
  </div>
</template>

<script async setup lang="ts"> import {invoke} from "@tauri-apps/api/core";
import {LabeqPluginManifest} from "@/lib/plugin.ts";
import {onMounted, ref} from "vue";
import {ColumnDef} from "@tanstack/vue-table";
import DataTable from "@/components/monitor/table/DataTable.vue";

let plugins = ref<LabeqPluginManifest[]>([]);

const columns: ColumnDef<LabeqPluginManifest>[] = [
  {
    accessorKey: 'name',
    header: 'Name',
    cell: ({row}) => row.getValue('name')
  },
  {
    accessorKey: 'version',
    header: 'Version',
    cell: ({row}) => row.getValue('version')
  }
]

onMounted(async () => {
  plugins.value = (await invoke<string[]>('list_plugins')).map(d => JSON.parse(d))
});
</script>
<style lang="scss" scoped>

</style>