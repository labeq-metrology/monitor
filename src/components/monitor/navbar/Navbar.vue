<script setup lang="ts">
import {Menubar,} from '@/components/ui/menubar'
import {SidebarTrigger} from "@/components/ui/sidebar";
import {onBeforeUnmount, onMounted, ref} from "vue";
import {applyAppearanceMode, getStoredAppearanceMode, watchActiveTheme, type AppearanceMode} from "@/lib/theme";
import {Button} from "@/components/ui/button";
import {Icon} from "@iconify/vue";
import {invoke} from "@tauri-apps/api/core";

const mode = ref<AppearanceMode>(getStoredAppearanceMode());
let observer: MutationObserver | undefined;
let stopThemeWatcher: (() => void) | undefined;

onMounted(() => {
  stopThemeWatcher = watchActiveTheme((_palette, themeMode) => {
    mode.value = themeMode;
  });
  observer = new MutationObserver(() => {
    mode.value = document.documentElement.classList.contains("dark") ? "dark" : "light";
  });
  observer.observe(document.documentElement, {attributes: true, attributeFilter: ["class"]});
});

onBeforeUnmount(() => {
  observer?.disconnect();
  stopThemeWatcher?.();
});

const toggleMode = () => {
  mode.value = mode.value === "light" ? "dark" : "light";
  applyAppearanceMode(mode.value);
};
</script>

<template>
  <Menubar class="h-9 border-l-0">
    <SidebarTrigger/>
    <Button variant="ghost" @click="toggleMode">
      <Icon v-if="mode == 'dark'" icon="heroicons-outline:sun"/>
      <Icon v-else icon="heroicons-outline:moon"/>
    </Button>
    <Button variant="ghost"
            @click="invoke('open_popup_webview', { title: 'Settings', label: 'settings', source: 'settings.html' })">
      <Icon icon="heroicons:cog-6-tooth" width="24" height="24"/>
    </Button>
  </Menubar>
</template>

<style lang="scss" scoped>

</style>