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
    <SidebarHeader @click="router.push('/')" class="cursor-pointer">
      <div class="px-2 text-muted-foreground text-sm">
        <h1>LabEq Monitor v0.0.1 <span class="bg-destructive text-foreground px-1 ml-1 font-semibold">ALPHA</span></h1>
      </div>
    </SidebarHeader>
    <SidebarContent>
      <ScrollArea class="h-full w-full border-t border-b">
        <ScrollBar orientation="vertical" />
        <Collapsible defaultOpen class="group">
          <SidebarGroup>
            <ConnectionMenu/>
            <WorkspaceMenu/>
          </SidebarGroup>
        </Collapsible>
      </ScrollArea>
    </SidebarContent>
    <SidebarFooter>
      s
    </SidebarFooter>
  </Sidebar>
</template>
