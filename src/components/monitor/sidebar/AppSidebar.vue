<script setup lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
} from "@/components/ui/sidebar"
import {Collapsible} from "@/components/ui/collapsible";
import {useRouter} from "vue-router";
import ConnectionMenu from "@/components/monitor/sidebar/ConnectionMenu.vue";
import WorkspaceMenu from "@/components/monitor/sidebar/WorkspaceMenu.vue";
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
      <Collapsible defaultOpen class="group">
        <SidebarGroup>
          <ConnectionMenu />
          <WorkspaceMenu />
        </SidebarGroup>
      </Collapsible>
    </SidebarContent>
    <SidebarFooter></SidebarFooter>
  </Sidebar>
</template>
