<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {SidebarMenuSubButton} from "@/components/ui/sidebar";
import {useRouter} from "vue-router";
import {LabeqConnection, LabeqDevice} from "@/lib/labeq.ts";
import {useDragSource} from "@/composables/useDrag.ts";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {useConnectionStore} from "@/store/connection.store.ts";
import {useWorkspaceStore} from "@/store/workspace.store.ts";

defineProps<{
  device: LabeqDevice,
  connection: LabeqConnection,
  workspaceId: string
}>();

const router = useRouter();
const {startDrag} = useDragSource();
const connectionStore = useConnectionStore();
const workspaceStore = useWorkspaceStore();
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger as-child>
      <div
          class="select-none touch-none"
          @pointerdown="startDrag('device', device.uuid, device.name, $event)"
          @click="router.push({ name: 'device', params: { connectionId: connection.id, id: device.uuid } })"
      >
        <SidebarMenuSubButton
            size="sm"
            class="h-6 w-full cursor-grab justify-start font-normal leading-none transition-colors">
          <Icon icon="heroicons:cpu-chip-20-solid"/>
          <span class="text-xs font-normal">{{ device.name }}</span>
        </SidebarMenuSubButton>
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent>
      <ContextMenuItem @click="router.push({ name: 'device', params: { connectionId: connection.id, id: device.uuid } })">
        <Icon icon="heroicons:arrow-top-right-on-square"/>
        Open
      </ContextMenuItem>
      <ContextMenuItem @click="connectionStore.refreshDeviceList(connection.id)">
        <Icon icon="heroicons:arrow-path"/>
        Refresh
      </ContextMenuItem>
      <ContextMenuItem variant="destructive" @click="workspaceStore.removeDeviceFromWorkspace(workspaceId, device.uuid)">
        <Icon icon="heroicons:trash"/>
        Remove
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>
</template>