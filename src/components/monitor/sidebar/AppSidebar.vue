<script setup lang="ts">
import {Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader,} from "@/components/ui/sidebar"
import {Collapsible} from "@/components/ui/collapsible";
import {useRouter} from "vue-router";
import ConnectionMenu from "@/components/monitor/sidebar/connection/ConnectionMenu.vue";
import WorkspaceMenu from "@/components/monitor/sidebar/workspace/WorkspaceMenu.vue";
import {ScrollArea, ScrollBar} from "@/components/ui/scroll-area";

const router = useRouter();

</script>

<template>
  <Sidebar variant="sidebar">
    <SidebarHeader @click="router.push('/')" class="cursor-pointer border-border h-9 border-y-1">
      <div class="px-2 text-foreground font-bold tracking-wider text-sm">
        <h1>LabEq Monitor</h1>
      </div>
    </SidebarHeader>
    <SidebarContent>
      <ScrollArea class="h-full w-full border-b">
        <ScrollBar orientation="vertical" />
        <Collapsible defaultOpen class="group">
          <SidebarGroup>
            <ConnectionMenu/>
            <WorkspaceMenu/>
          </SidebarGroup>
        </Collapsible>
      </ScrollArea>
    </SidebarContent>
    <SidebarFooter class="flex flex-row text-sm bg-gradient-to-r from-blue-800 to-blue-950/50">
      <span class="uppercase font-semibold text-zinc-50 rounded-xl">beta</span>
      <span>0.0.1</span>
    </SidebarFooter>
  </Sidebar>
</template>
