<template>
  <div>
    <h2 class="heading-2">Welcome to LabEq Monitor.</h2>
    <VisualModelContainer
        model-id="smith-chart-a"
        :data="data"
        config=""
    />
    <button @click="invoke('open_settings')">s</button>
  </div>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {computed, onMounted, onUnmounted, reactive, ref, watch} from "vue";
import VisualModelContainer from "@/components/monitor/device/panels/VisualModelContainer.vue";
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";

const modelStore = useVisualModelsStore()

modelStore.autoDiscoverModels()

const data = [
  { name: 'Z1', real: 1.5, imag: 0.5 },
  { name: 'Z2', real: 0.8, imag: -0.3 }
];

onMounted(async () => {
  console.log(modelStore.models)
  invoke<string>('list_plugins').then((d: string) => console.log(JSON.parse(d))).catch(console.error)
});
</script>