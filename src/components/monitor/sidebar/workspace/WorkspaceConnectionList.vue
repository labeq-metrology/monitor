<template>
  <AlertDialog v-model:open="isAddingConnection">
    <AddConnectionDialog :workspace-id="workspaceId" @confirm="handleAddConnection"/>
  </AlertDialog>
  <ContextMenu>
    <ContextMenuTrigger as-child>
      <div>

        <div class="text-xs font-semibold text-foreground px-2 pb-1">
          Connections
        </div>
        <div
            ref="rootDropEl"
            :class="{ 'ring-1 ring-ring ring-inset rounded-md': isOver }"
        >
          <div
              v-if="isCreatingFolder"
              class="flex items-center space-x-2 w-full px-2 py-1 rounded-md"
          >
            <Icon
                icon="heroicons:chevron-right"
                width="16"
                height="16"
            />
            <Icon
                icon="heroicons:folder"
                class="text-muted-foreground"
                width="16"
                height="16"
            />
            <input
                ref="folderNameInput"
                v-model="newFolderName"
                @blur="handleFolderBlur"
                @keyup.enter="confirmCreateFolder"
                @keyup.esc="cancelCreateFolder"
                @click.stop
                placeholder="Folder name..."
                class="
            text-xs font-normal tracking-wide
            text-left
            bg-background border border-border rounded px-1
            focus:outline-none focus:ring-1 focus:ring-ring
            text-foreground
            placeholder:text-muted-foreground
            h-[1.125rem] min-w-32 max-w-32 w-32
          "
            />
          </div>

          <template v-for="(item, index) in connections.items" :key="index">
            <!-- Nested Folder -->
            <ConnectionFolder
                v-if="item.type === 'folder'"
                :folder="item.value"
                :workspace-id="workspaceId"
            />

            <!-- Connection Item -->
            <ContextMenu
                v-else-if="connectionStore.connections.get(item.value)"
            >
              <ContextMenuTrigger as-child>
                <div
                    class="flex items-center space-x-2 w-full px-2 py-1 rounded-md hover:bg-accent/50 transition-colors cursor-grab select-none touch-none"
                    @pointerdown="startDrag('connection', item.value, connectionStore.connections.get(item.value)!.name, $event)"
                >
                  <Icon
                      :icon="connectionStore.connections.get(item.value)!.state == 'Connected' ? 'heroicons:signal-16-solid' : 'heroicons:signal-slash-16-solid'"
                      :class="connectionStore.connections.get(item.value)!.state == 'Connected' ? 'text-positive' : 'text-muted-foreground'"
                      width="16"
                      height="16"
                  />
                  <span
                      class="
                        text-xs font-normal tracking-wide
                        text-left flex-1
                        whitespace-nowrap overflow-hidden overflow-ellipsis
                        text-foreground
                      "
                  >{{ connectionStore.connections.get(item.value)!.name }}</span>
                </div>
              </ContextMenuTrigger>
              <ContextMenuContent>
                <ContextMenuItem @click="connectionStore.activateConnection(item.value)" class="cursor-pointer">
                  Connect
                </ContextMenuItem>
                <ContextMenuItem @click="connectionStore.deactivateConnection(item.value)" class="cursor-pointer">
                  Disconnect
                </ContextMenuItem>
                <ContextMenuItem @click="connectionStore.refreshDeviceList(item.value)" class="cursor-pointer">
                  Refresh
                </ContextMenuItem>
                <ContextMenuItem @click="handleRemoveConnection(item.value)" variant="destructive" class="cursor-pointer">
                  Remove
                </ContextMenuItem>
              </ContextMenuContent>
            </ContextMenu>
          </template>

          <div
              v-if="connections.items.length === 0 && !isCreatingFolder"
              class="text-muted-foreground text-xs px-2 py-1 select-none cursor-default"
          >
            No connections
          </div>
        </div>
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent class="overflow-hidden">
      <ContextMenuSub>
        <ContextMenuSubTrigger class="text-xs py-1 px-2">Add</ContextMenuSubTrigger>
        <ContextMenuSubContent :align-offset="-5">
          <ContextMenuItem @click="startCreatingFolder">
            <Icon icon="heroicons:folder-16-solid"/>
            Folder
          </ContextMenuItem>
          <ContextMenuItem @click="isAddingConnection = true">
            <Icon icon="heroicons:signal-16-solid"/>
            Connection
          </ContextMenuItem>
        </ContextMenuSubContent>
      </ContextMenuSub>
    </ContextMenuContent>
  </ContextMenu>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import ConnectionFolder from "../connection/ConnectionFolder.vue";
import type {Folder} from "@/lib/labeq.ts";
import {nextTick, onMounted, ref} from "vue";
import {useWorkspaceStore} from "@/store/workspace.store.ts";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger
} from "@/components/ui/context-menu";
import {AlertDialog} from "@/components/ui/alert-dialog";
import AddConnectionDialog from "@/components/monitor/dialogs/AddConnectionDialog.vue";
import {useConnectionStore} from "@/store/connection.store.ts";
import {useDragSource, useDragTarget} from "@/composables/useDrag.ts";

const props = defineProps<{
  workspaceId: string
  connections: Folder<string>
}>();

const store = useWorkspaceStore();
const connectionStore = useConnectionStore();

const rootDropEl = ref<HTMLElement | null>(null);
const { startDrag } = useDragSource();
const { isOver, register } = useDragTarget(
    () => rootDropEl.value,
    'connection',
    (connectionId) => store.moveConnectionToFolder(props.workspaceId, connectionId, undefined)
);
onMounted(register);

const isCreatingFolder = ref(false);
const isAddingConnection = ref(false);
const newFolderName = ref("");
const folderNameInput = ref<HTMLInputElement | null>(null);


const handleAddConnection = async (selectedConnectionId: string) => {
  isAddingConnection.value = false;
  if (connectionStore.connections.has(selectedConnectionId)) return store.addConnectionToWorkspace(props.workspaceId, selectedConnectionId);
};

const startCreatingFolder = () => {
  isCreatingFolder.value = true;
  nextTick(() => {
    setTimeout(() => {
      folderNameInput.value?.focus();
    }, 200);
  });
};

const confirmCreateFolder = async () => {
  if (newFolderName.value.trim()) {
    await store.createConnectionFolder(props.workspaceId, undefined, newFolderName.value.trim());
    newFolderName.value = "";
    isCreatingFolder.value = false;
  } else {
    cancelCreateFolder();
  }
};

const handleFolderBlur = () => {
  setTimeout(() => {
    if (newFolderName.value.trim()) {
      confirmCreateFolder();
    } else {
      cancelCreateFolder();
    }
  }, 150);
};

const cancelCreateFolder = () => {
  newFolderName.value = "";
  isCreatingFolder.value = false;
};

const handleRemoveConnection = async (connectionId: string) => {
  if (confirm('Remove this connection from workspace?')) {
    await store.removeConnectionFromWorkspace(props.workspaceId, connectionId);
  }
};
</script>