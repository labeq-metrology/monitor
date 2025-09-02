<script setup lang="ts">
import Navbar from "@/components/monitor/navbar/Navbar.vue";
import {SidebarProvider} from "@/components/ui/sidebar";
import AppSidebar from "@/components/monitor/sidebar/AppSidebar.vue";
</script>

<template>
  <div>
    <SidebarProvider class="flex">
      <AppSidebar/>
      <main class="w-full">
        <header>
          <Navbar/>
        </header>
        <article class="p-5 justify-self-center flex w-full lg:w-3/4 justify-center">
          <Suspense>
            <router-view class="justify-self-center w-full"/>
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