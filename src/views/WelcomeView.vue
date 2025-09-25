<template>
  <div>
    <h2 class="heading-2">Welcome to LabEq Monitor.</h2>
    <button @click="invoke('open_settings')">s</button>
  </div>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {onMounted} from "vue";

onMounted(async () => {
  invoke<string>('list_plugins').then((d: string) => console.log(JSON.parse(d))).catch(console.error)
});
</script>