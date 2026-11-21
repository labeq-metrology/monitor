<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {onMounted} from "vue";

import {Toaster} from '@/components/ui/sonner'
import Navbar from "@/components/monitor/navbar/Navbar.vue";
import TabBar from "@/components/monitor/navbar/TabBar.vue";
import {SidebarProvider} from "@/components/ui/sidebar";
import AppSidebar from "@/components/monitor/sidebar/AppSidebar.vue";
import 'vue-sonner/style.css'
import {useWorkspaceStore} from "@/store/workspace.store.ts";
import DragGhost from "@/components/drag/DragGhost.vue";
import {useDragSource} from '@/composables/useDrag';
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";

const {ghost} = useDragSource();
useWorkspaceStore();
const visualModels = useVisualModelsStore();
onMounted(async () => {
  try {
    (window as any).Vue = await import('vue');
    (window as any).Pinia = await import('pinia');
    await invoke('init_plugins');
    const manifests = await invoke<string[]>('list_plugins');
    const plugins = manifests.map(manifest => JSON.parse(manifest));
    const loadPanels = () => {
      try {
        visualModels.loadPluginPanels(plugins);
      } catch (error) {
        console.error('Failed to load plugin panels:', error);
      }
    };
    if ('requestIdleCallback' in window) {
      window.requestIdleCallback(loadPanels);
    } else {
      setTimeout(loadPanels, 0);
    }
  } catch (error) {
    console.error('Failed to initialize plugins:', error);
  }
});
</script>

<template>
  <div>
    <Toaster :toast-options="{ style: { } }"/>
    <SidebarProvider class="flex">
      <AppSidebar/>
      <main class="w-full">
        <header>
          <Navbar/>
          <TabBar/>
        </header>
        <article class="p-5 justify-self-center flex w-full xl:w-3/4 justify-center">
          <Suspense>
            <router-view class="justify-self-center w-full" v-slot="{ Component, route }">
              <keep-alive>
                <component :is="Component" :key="route.fullPath" />
              </keep-alive>
            </router-view>
          </Suspense>
        </article>
      </main>
    </SidebarProvider>
    <DragGhost :ghost="ghost"/>
  </div>
</template>

<style scoped>
html, body {
  @apply h-full w-full m-[0] p-[0];
}
</style>

<style>
html, body {
  @apply h-full w-full m-[0] p-[0];
}
</style>
