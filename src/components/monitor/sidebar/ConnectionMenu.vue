<template>
  <Collapsible class="group/connections-section">
    <SidebarGroupLabel class="flex items-center gap-2 group/connections-section-label">
      <div class="flex items-center gap-2 cursor-default">
        <CollapsibleTrigger class="w-full cursor-pointer" as-child>
          <Icon
              icon="heroicons:chevron-down"
              class="transition-transform duration-300 group-data-[state=closed]/connections-section:rotate-180 -ml-2 text-accent-foreground self-end"
              width="16"
              height="16"
          />
        </CollapsibleTrigger>
        <p>Connections</p>
      </div>
      <div
          class="group-hover/connections-section-label:opacity-100 group-hover/connections-section-label:visible opacity-0 invisible transition-all duration-200 ml-auto">
        <RouterLink to="/new-connection">
          <Button variant="link" class="w-4 h-4 m-0 p-0 flex items-center justify-center text-accent-foreground/50">
            <Icon icon="heroicons:plus" width="16" height="16"/>
          </Button>
        </RouterLink>
      </div>
    </SidebarGroupLabel>
    <CollapsibleContent>
      <SidebarMenu class="">
        <Collapsible v-if="connectionStore.connections.size > 0"
                     v-for="connection in connectionStore.connections.values()"
                     :key="connection.id"
                     class="group/collapsible">
          <SidebarGroup class="py-1 bg-background rounded-md">
            <SidebarGroupLabel asChild>
              <ConnectionButton :connection="connection"/>
            </SidebarGroupLabel>
            <CollapsibleContent
                class="mx-2 mb-2 text-muted-foreground border-l-sidebar-accent-foreground/20 border-l-2">
              <TooltipProvider v-for="device in connection.devices" :key="device.uuid">
                <TooltipRoot :delayDuration="5000">
                  <TooltipTrigger asChild class="mx-2 w-full">
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
        <div class="text-muted-foreground text-xs mx-2 px-2 py-2 rounded-md bg-background" v-else>
          <p>No connections present...</p>
        </div>
      </SidebarMenu>
    </CollapsibleContent>
  </Collapsible>
</template>

<script setup lang="ts">
import {SidebarGroup, SidebarGroupLabel, SidebarMenu, SidebarMenuSubButton} from "@/components/ui/sidebar/index.ts";
import {Button} from "@/components/ui/button/index.ts";
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip/index.ts";
import {Collapsible, CollapsibleContent} from "@/components/ui/collapsible/index.ts";
import {Icon} from "@iconify/vue";
import ConnectionButton from "@/components/monitor/sidebar/ConnectionButton.vue";
import {useConnectionStore} from "@/store/connection.store.ts";
import {useRouter} from "vue-router";
import {TooltipRoot} from "reka-ui";
import {CollapsibleTrigger} from "@/components/ui/collapsible";

const connectionStore = useConnectionStore();
const router = useRouter();
</script>

<style lang="scss" scoped>

</style>