<template>
  <div>
    <h2 class="heading-2">Welcome to LabEq Monitor.</h2>
  </div>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {onMounted} from "vue";
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";

const modelStore = useVisualModelsStore();

modelStore.autoDiscoverModels();

onMounted(async () => {
  invoke<string>('list_plugins').then((d: string) => console.log(JSON.parse(d))).catch(console.error);
});
</script>