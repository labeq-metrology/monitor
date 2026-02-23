<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";

import {Toaster} from '@/components/ui/sonner'
import Navbar from "@/components/monitor/navbar/Navbar.vue";
import {SidebarProvider} from "@/components/ui/sidebar";
import AppSidebar from "@/components/monitor/sidebar/AppSidebar.vue";
import 'vue-sonner/style.css'
import {useWorkspaceStore} from "@/store/workspace.store.ts";
useWorkspaceStore();
invoke('init_plugins')
</script>

<template>
  <div>
    <Toaster :toast-options="{
      style: { }
    }" />
    <SidebarProvider class="flex">
      <AppSidebar/>
      <main class="w-full">
        <header>
          <Navbar/>
        </header>
        <article class="p-5 justify-self-center flex w-full lg:w-3/4 justify-center">
          <Suspense>
            <router-view :key="$route.path" class="justify-self-center w-full"/>
          </Suspense>
        </article>
      </main>
    </SidebarProvider>
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