<template>
  <CollapsibleTrigger class="hover:bg-sidebar-accent cursor-pointer w-full px-2 py-1 rounded-md">
    <ContextMenu>
      <ContextMenuTrigger asChild class="text-sm text-accent-background">
        <div class="flex items-center space-x-2 w-full">
          <Icon
              :icon="connection.state == 'Connected' ? 'heroicons:signal-20-solid' : 'heroicons:signal-slash-20-solid'"
              :class="connection.state == 'Connected' ? 'text-green-600 text-xl' : 'text-red-600 text-xl'"
          />
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger
                  class="text-left max-w-8/12 whitespace-nowrap overflow-hidden overflow-ellipsis cursor-pointer">
                {{ connection.id }}
              </TooltipTrigger>
              <TooltipContent>
                <p>{{ connection.id }}</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
          <Icon
              icon="heroicons:chevron-down"
              class="text-lg self-end ml-auto transition-transform duration-300 group-data-[state=open]/collapsible:rotate-180"
          />
        </div>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem @click="connectionStore.refreshDeviceList(connection)" class="cursor-pointer">
          Refresh
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  </CollapsibleTrigger>
</template>
<script setup lang="ts">
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {Icon} from "@iconify/vue";
import {CollapsibleTrigger} from "reka-ui";
import {LabeqConnection} from "@/lib/labeq.ts";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {useConnectionStore} from "@/store/connection.store.ts";

defineProps<{
  connection: LabeqConnection
}>()

const connectionStore = useConnectionStore()
</script>