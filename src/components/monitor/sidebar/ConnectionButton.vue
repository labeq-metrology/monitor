<template>
  <CollapsibleTrigger class="group/connection transition-colors duration-75 cursor-pointer w-full px-2 py-1 rounded-md">
    <ContextMenu>
      <ContextMenuTrigger asChild class="text-sm text-accent-background">
        <div class="flex items-center space-x-2 w-full ">
          <Icon
              icon="heroicons:chevron-down"
              class="text-lg self-end transition-transform duration-300 group-data-[state=closed]/collapsible:rotate-180 -ml-2 text-accent-foreground"
              width="16"
              height="16"
          />
          <Icon
              :icon="connection.state == 'Connected' ? 'heroicons:signal-16-solid' : 'heroicons:signal-slash-16-solid'"
              :class="connection.state == 'Connected' ? 'text-positive text-lg' : 'text-muted-foreground text-lg'"
          />
          <TooltipProvider>
            <TooltipRoot :delayDuration="3000">
              <TooltipTrigger
                  class="
                    text-xs font-normal tracking-wide
                    text-left max-w-8/12
                    whitespace-nowrap overflow-hidden overflow-ellipsis
                    cursor-pointer transition-colors
                    text-foreground
                    group-data-[state=closed]/collapsible:text-muted-foreground
                    group-data-[state=closed]/collapsible:group-hover/collapsible:text-foreground
                    group-data-[state=open]/collapsible:group-hover/collapsible:text-foreground/75
                  "
              >
                {{ connection.id }}
              </TooltipTrigger>
              <TooltipContent>
                <p>{{ connection.id }}</p>
              </TooltipContent>
            </TooltipRoot>
          </TooltipProvider>
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
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {Icon} from "@iconify/vue";
import {CollapsibleTrigger, TooltipRoot} from "reka-ui";
import {LabeqConnection} from "@/lib/labeq.ts";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {useConnectionStore} from "@/store/connection.store.ts";

defineProps<{
  connection: LabeqConnection
}>()

const connectionStore = useConnectionStore()
</script>