<script setup lang="ts">
import {Menubar,} from '@/components/ui/menubar'
import {SidebarTrigger} from "@/components/ui/sidebar";
import {useColorMode} from "@vueuse/core";
import {Button} from "@/components/ui/button";
import {Icon} from "@iconify/vue";
import {invoke} from "@tauri-apps/api/core";

const mode = useColorMode()
</script>

<template>
  <Menubar class="h-9 border-l-0">
    <SidebarTrigger/>
    <Button variant="ghost" @click="mode == 'light' ? mode = 'dark' : mode = 'light'">
      <Icon v-if="mode == 'dark'" icon="heroicons-outline:sun"/>
      <Icon v-else icon="heroicons-outline:moon"/>
    </Button>
    <Button variant="ghost"
            @click="invoke('open_popup_webview', { title: 'Settings', label: 'settings', source: 'settings.html' })">
      <Icon icon="heroicons:cog-6-tooth" width="24" height="24"/>
    </Button>
    <Button variant="ghost"
            @click="invoke('open_popup_webview', { title: 'Settings', label: 'settings1', source: 'new-connection' })">
      <Icon icon="heroicons:cog-6-tooth" width="24" height="24"/>
    </Button>
  </Menubar>
</template>

<style lang="scss" scoped>

</style>