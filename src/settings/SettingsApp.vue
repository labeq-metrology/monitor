<template>
  <div class="bg-background w-full h-screen">
    <SidebarProvider class="flex h-full">
      <Sidebar variant="sidebar" class="h-full">
        <SidebarHeader>
          <div class="px-2 text-muted-foreground text-sm">
            <h1>Settings</h1>
          </div>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup v-for="group in Object.keys(settingsStructure)" :key="group">
            <SidebarGroupLabel class="capitalize">{{ group }}</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenuSubButton
                  v-for="setting in settingsStructure[group]"
                  :key="setting"
                  class="capitalize cursor-pointer"
                  :class="{ 'bg-sidebar-accent': route.name === setting }"
                  @click="router.push(setting)"
              >
                {{ setting }}
              </SidebarMenuSubButton>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>

      <!-- flex + h-full to stretch -->
      <article class="p-5 flex w-full lg:w-3/5 justify-center h-full overflow-y-auto mx-auto">
          <router-view class="h-full" />
      </article>
    </SidebarProvider>
  </div>
</template>

<script setup lang="ts">
import {useColorMode} from "@vueuse/core";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenuSubButton,
  SidebarProvider
} from "@/components/ui/sidebar";
import {useRoute, useRouter} from "vue-router";

const settingsStructure: { [key: string]: string[] } = {
  general: ['appearance', 'plugins']
};

const route = useRoute();
const router = useRouter();

useColorMode();
</script>

<style lang="scss" scoped>

</style>