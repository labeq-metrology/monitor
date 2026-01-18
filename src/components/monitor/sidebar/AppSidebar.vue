<script setup lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuSubButton,
} from "@/components/ui/sidebar"
import {TooltipRoot} from "reka-ui";
import {Collapsible, CollapsibleContent} from "@/components/ui/collapsible";
import {Icon} from "@iconify/vue";
import {Button} from "@/components/ui/button";
import {useConnectionStore} from "@/store/connection.store.ts";
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import ConnectionButton from "@/components/monitor/sidebar/ConnectionButton.vue";
import {useRouter} from "vue-router";

const connectionStore = useConnectionStore();
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
          <SidebarGroupLabel>Connections</SidebarGroupLabel>
          <SidebarMenu>
            <SidebarMenuButton asChild class="bg-background justify-center text-muted-foreground">
              <RouterLink to="/new-connection">
                <Button variant="ghost">
                  <Icon icon="heroicons:plus"/>
                  <span>New connection</span>
                </Button>
              </RouterLink>
            </SidebarMenuButton>

            <Collapsible v-for="connection in connectionStore.connections.values()" :key="connection.id"
                         class="group/collapsible">
              <SidebarGroup class="py-0">
                <SidebarGroupLabel asChild>
                  <ConnectionButton :connection="connection"/>
                </SidebarGroupLabel>
                <CollapsibleContent
                    class="ml-4 text-muted-foreground mt-1 border-l-sidebar-accent-foreground/20 border-l-2">
                  <TooltipProvider v-for="device in connection.devices" :key="device.uuid">
                    <TooltipRoot :delayDuration="2000">
                      <TooltipTrigger asChild class="mx-2 mb-0.5 w-full">
                        <SidebarMenuSubButton class="select-none cursor-pointer"
                          @click.prevent="router.push({
                            name: 'device',
                            params: { connectionId: connection.id, id: device.uuid }
                          })"
                        >
                          <Icon icon="heroicons:cpu-chip-20-solid"/>
                          <span class="text-xs">{{ device.name }}</span>
                        </SidebarMenuSubButton>
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>{{ device.manufacturer }} {{ device.model }} &mdash; {{ device.name }}</p>
                      </TooltipContent>
                    </TooltipRoot>
                  </TooltipProvider>
                </CollapsibleContent>
              </SidebarGroup>
            </Collapsible>
          </SidebarMenu>
        </SidebarGroup>
      </Collapsible>
    </SidebarContent>
    <SidebarFooter></SidebarFooter>
  </Sidebar>
</template>
