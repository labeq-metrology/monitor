<script setup lang="ts">

import {Sidebar, SidebarHeader} from "@/components/ui/sidebar";
</script>

<template>
  <div class="settings-overlay bg-background">
    <Sidebar>
      <SidebarHeader>
        s
      </SidebarHeader>
    </Sidebar>
  </div>
</template>

<style scoped lang="scss">
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
}
</style>
