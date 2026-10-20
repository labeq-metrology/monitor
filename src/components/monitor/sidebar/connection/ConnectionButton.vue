<template>
  <CollapsibleTrigger class="group/connection cursor-pointer w-full rounded-md">
    <ContextMenu>
      <ContextMenuTrigger as-child class="text-sm text-accent-background">
        <SidebarMenuSubButton
            class="flex w-full translate-x-0"
            @click="router.push({ name: 'connection', params: { connectionId: connection.id } })"
        >
          <Icon
              icon="heroicons:chevron-down"
              class="shrink-0 transition-transform duration-300 group-data-[state=closed]/collapsible:-rotate-90 text-accent-foreground"
              width="16"
              height="16"
          />
          <Icon
              :icon="connection.state == 'Connected' ? 'heroicons:signal-16-solid' : 'heroicons:signal-slash-16-solid'"
              :class="[
                connection.state == 'Connected' ? '!text-positive' : '!text-muted-foreground',
                'shrink-0',
              ]"
              width="16"
              height="16"
          />
          <span
              class="
                min-w-0 flex-1
                text-xs font-normal tracking-wide
                text-left
                whitespace-nowrap overflow-hidden overflow-ellipsis
                transition-colors
                text-foreground
                group-data-[state=closed]/collapsible:text-muted-foreground
                group-data-[state=closed]/collapsible:group-hover/connection:text-foreground
              "
          >{{ connection.name }}</span>
        </SidebarMenuSubButton>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem @click="connectionStore.activateConnection(connection.id)">
          <Icon icon="heroicons:signal"/>
          Connect
        </ContextMenuItem>
        <ContextMenuItem @click="connectionStore.deactivateConnection(connection.id)">
          <Icon icon="heroicons:signal-slash"/>
          Disconnect
        </ContextMenuItem>
        <ContextMenuItem @click="connectionStore.refreshDeviceList(connection.id)">
          <Icon icon="heroicons:arrow-path"/>
          Refresh
        </ContextMenuItem>
        <ContextMenuItem variant="destructive">
          <Icon icon="heroicons:trash"/>
          Remove
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  </CollapsibleTrigger>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {CollapsibleTrigger} from "reka-ui";
import {LabeqConnection} from "@/lib/labeq.ts";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {useConnectionStore} from "@/store/connection.store.ts";
import {SidebarMenuSubButton} from "@/components/ui/sidebar";
import {useRouter} from "vue-router";

defineProps<{
  connection: LabeqConnection
}>();

const connectionStore = useConnectionStore();
const router = useRouter();
</script>