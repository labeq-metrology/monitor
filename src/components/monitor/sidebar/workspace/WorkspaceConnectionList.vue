<template>
  <AlertDialog v-model:open="isAddingConnection">
    <AddConnectionDialog :workspace-id="workspaceId" @confirm="handleAddConnection"/>
  </AlertDialog>
  <ContextMenu>
    <ContextMenuTrigger as-child>
      <div>
        <div class="flex h-7 items-center rounded-md px-2 text-xs font-medium text-sidebar-foreground/70">
          Connections
        </div>
        <div
            ref="rootDropEl"
            class="rounded-md"
            :class="{ 'ring-1 ring-ring ring-inset': isOver }"
        >
          <FileTree>
            <div
                v-if="isCreatingFolder"
                class="flex h-7 w-full items-center gap-2 rounded-md px-2 leading-none"
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
            text-xs font-normal
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
                      class="flex h-6 w-full cursor-grab select-none touch-none items-center gap-2 rounded-md px-2 font-normal leading-none transition-colors hover:bg-muted"
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
                        text-xs font-normal
                        text-left flex-1
                        whitespace-nowrap overflow-hidden overflow-ellipsis
                        text-foreground
                      "
                    >{{ connectionStore.connections.get(item.value)!.name }}</span>
                  </div>
                </ContextMenuTrigger>
                <ContextMenuContent>
                  <ContextMenuItem @click="connectionStore.activateConnection(item.value)">
                    <Icon icon="heroicons:signal"/>
                    Connect
                  </ContextMenuItem>
                  <ContextMenuItem @click="connectionStore.deactivateConnection(item.value)">
                    <Icon icon="heroicons:signal-slash"/>
                    Disconnect
                  </ContextMenuItem>
                  <ContextMenuItem @click="connectionStore.refreshDeviceList(item.value)">
                    <Icon icon="heroicons:arrow-path"/>
                    Refresh
                  </ContextMenuItem>
                  <ContextMenuItem @click="handleRemoveConnection(item.value)" variant="destructive">
                    <Icon icon="heroicons:trash"/>
                    Remove
                  </ContextMenuItem>
                </ContextMenuContent>
              </ContextMenu>
            </template>

            <div
                v-if="connections.items.length === 0 && !isCreatingFolder"
                class="flex h-7 items-center px-2 leading-none"
            >
              <span class="select-none cursor-default text-xs text-muted-foreground">No connections</span>
            </div>
          </FileTree>
        </div>
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent class="overflow-hidden">
      <ContextMenuSub>
        <ContextMenuSubTrigger>
          <Icon icon="heroicons:plus"/>
          Add
        </ContextMenuSubTrigger>
        <ContextMenuSubContent :align-offset="-5">
          <ContextMenuItem @click="startCreatingFolder">
            <Icon icon="heroicons:folder-16-solid"/>
            Folder
          </ContextMenuItem>
          <ContextMenuItem @click="isAddingConnection = true">
            <Icon icon="heroicons:signal"/>
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
import {FileTree} from "@/components/ui/file-tree";

const props = defineProps<{
  workspaceId: string
  connections: Folder<string>
}>();

const store = useWorkspaceStore();
const connectionStore = useConnectionStore();

const rootDropEl = ref<HTMLElement | null>(null);
const {startDrag} = useDragSource();
const {isOver, register} = useDragTarget(
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
  if (
      connectionStore.connections.has(selectedConnectionId)
      && !store.workspaceHasConnection(props.workspaceId, selectedConnectionId)
  ) {
    await store.addConnectionToWorkspace(props.workspaceId, selectedConnectionId);
  }
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