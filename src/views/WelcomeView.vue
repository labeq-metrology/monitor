<template>
  <div>
    <h2 class="heading-2">Welcome to LabEq Monitor.</h2>
    <Button
        variant="outline" @click="() => invoke('init_plugins')"
    >
      Add to calendar
    </Button>
  </div>
</template>

<script setup lang="ts">
import {Button} from "@/components/ui/button";
import {invoke} from "@tauri-apps/api/core";
</script>