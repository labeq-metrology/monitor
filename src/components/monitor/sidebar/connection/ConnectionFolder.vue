<template>
  <ConfirmDialog
      :open="removingWorkspaceConnectionId !== null"
      @update:open="open => { if (!open) removingWorkspaceConnectionId = null }"
      title="Remove from workspace"
      :description="removingWorkspaceDeviceCount > 0
        ? `This will also remove ${removingWorkspaceDeviceCount} associated device${removingWorkspaceDeviceCount === 1 ? '' : 's'} from this workspace.`
        : 'Remove this connection from the workspace?'"
      confirm-label="Remove"
      destructive
      @confirm="confirmRemoveFromWorkspace"
  />
  <ConfirmDialog
      :open="removingConnectionId !== null"
      @update:open="open => { if (!open) removingConnectionId = null }"
      title="Remove connection"
      :description="`Remove connection '${removingConnectionId ? connectionStore.connections.get(removingConnectionId)?.name : ''}' globally?`"
      confirm-label="Remove"
      destructive
      @confirm="confirmRemoveConnection"
  />
  <AlertDialog v-model:open="isAddingConnection">
    <AddConnectionDialog :workspace-id="workspaceId" @confirm="handleAddConnection"/>
  </AlertDialog>
  <Collapsible v-model:open="isOpen" class="group/connection-folder">
    <div ref="triggerEl">
      <CollapsibleTrigger
          class="group/button group/folder-trigger h-6 w-full justify-start rounded-md px-2 font-normal transition-colors duration-75 aria-expanded:bg-transparent aria-expanded:hover:bg-muted dark:aria-expanded:hover:bg-muted/50"
          :class="{ 'outline outline-1 outline-ring bg-accent/30': isOver }">
        <ContextMenu>
          <ContextMenuTrigger as-child>
            <div class="flex w-full items-center gap-2">
              <Icon
                  icon="heroicons:chevron-right"
                  class="transition-transform group-data-[state=open]/button:rotate-90 text-accent-foreground"
                  width="16"
                  height="16"
              />
              <Icon
                  icon="heroicons:folder"
                  class="text-muted-foreground"
                  width="16"
                  height="16"
              />
              <span
                  v-if="!isEditing"
                  @dblclick="startEditing"
                  class="
                  text-xs font-normal
                  text-left flex-1
                  whitespace-nowrap overflow-hidden overflow-ellipsis
                  cursor-pointer transition-colors
                  text-foreground
                  group-data-[state=closed]/connection-folder:text-muted-foreground
                  group-data-[state=closed]/connection-folder:group-hover/folder-trigger:text-foreground
                "
              >
              {{ folder.name }}
            </span>
              <input
                  v-else
                  ref="nameInput"
                  v-model="editingName"
                  @keyup.enter="confirmEdit"
                  @keyup.esc="cancelEdit"
                  @click.stop
                  class="
                  text-xs font-normal
                  text-left w-32 shrink-0
                  bg-background border border-border rounded px-1
                  focus:outline-none focus:ring-1 focus:ring-ring
                  text-foreground
                  h-[1.125rem] leading-[1.125rem]
                "
              />
              <span class="text-xs text-muted-foreground">{{ itemCount }}</span>
            </div>
          </ContextMenuTrigger>
          <ContextMenuContent>
            <ContextMenuItem @click="isAddingConnection = true">
              <Icon icon="heroicons:signal"/>
              Add connection
            </ContextMenuItem>
            <ContextMenuItem @click="handleAddSubfolder">
              <Icon icon="heroicons:folder-plus"/>
              Add subfolder
            </ContextMenuItem>
            <ContextMenuItem @click="startEditing">
              <Icon icon="heroicons:pencil"/>
              Rename
            </ContextMenuItem>
            <ContextMenuItem @click="handleDelete" variant="destructive">
              <Icon icon="heroicons:trash"/>
              Delete
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      </CollapsibleTrigger>
    </div>
    <CollapsibleContent>
      <div class="ml-4 flex flex-col">
        <div
            v-if="isCreatingSubfolder"
            class="flex h-7 w-full items-center gap-2 rounded-md px-2 leading-none"
        >
          <Icon
              icon="heroicons:chevron-down"
              class="opacity-0"
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
              ref="subfolderNameInput"
              v-model="newSubfolderName"
              @keyup.enter="confirmCreateSubfolder"
              @keyup.esc="cancelCreateSubfolder"
              @click.stop
              placeholder="Folder name..."
              class="
                text-xs font-normal
                text-left w-32 shrink-0
                bg-background border border-border rounded px-1
                focus:outline-none focus:ring-1 focus:ring-ring
                text-foreground
                placeholder:text-muted-foreground
                h-[1.125rem] leading-[1.125rem]
              "
          />
        </div>

        <template v-for="(item, index) in folder.items" :key="index">
          <ConnectionFolder
              v-if="item.type === 'folder'"
              :folder="item.value"
              :workspace-id="workspaceId"
          />

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
              <ContextMenuItem @click="handleRemoveFromWorkspace(item.value)" variant="destructive">
                <Icon icon="heroicons:trash"/>
                Remove from workspace
              </ContextMenuItem>
              <ContextMenuItem @click="handleRemoveConnection(item.value)" variant="destructive">
                <Icon icon="heroicons:trash"/>
                Remove
              </ContextMenuItem>
            </ContextMenuContent>
          </ContextMenu>
        </template>
      </div>
    </CollapsibleContent>
  </Collapsible>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {Collapsible, CollapsibleContent, CollapsibleTrigger} from "@/components/ui/collapsible";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {AlertDialog} from "@/components/ui/alert-dialog";
import AddConnectionDialog from "@/components/monitor/dialogs/AddConnectionDialog.vue";
import type {Item, NestedFolder} from "@/lib/labeq.js";
import {useConnectionStore} from "@/store/connection.store.ts";
import {computed, nextTick, onMounted, ref, watchEffect} from "vue";
import {useWorkspaceStore} from "@/store/workspace.store.js";
import {useDragSource, useDragTarget} from "@/composables/useDrag.ts";
import ConfirmDialog from "@/components/monitor/dialogs/ConfirmDialog.vue";

const props = defineProps<{
  folder: NestedFolder<string>
  workspaceId: string
}>();

const store = useWorkspaceStore();
const connectionStore = useConnectionStore();

const {startDrag} = useDragSource();

const triggerEl = ref<HTMLElement | null>(null);
const {isOver, register} = useDragTarget(
    () => triggerEl.value,
    'connection',
    (connectionId) => store.moveConnectionToFolder(props.workspaceId, connectionId, props.folder.id),
    10
);
onMounted(register);
watchEffect(register);

const isEditing = ref(false);
const isOpen = ref(false);
const editingName = ref(props.folder.name);
const nameInput = ref<HTMLInputElement | null>(null);

const isCreatingSubfolder = ref(false);
const newSubfolderName = ref("");
const subfolderNameInput = ref<HTMLInputElement | null>(null);
const isAddingConnection = ref(false);
const removingConnectionId = ref<string | null>(null);
const removingWorkspaceConnectionId = ref<string | null>(null);
const removingWorkspaceDeviceCount = ref(0);

const itemCount = computed(() => {
  return props.folder.items.filter(item => item.type === 'item').length;
});

const countAssociatedDevices = (connectionId: string): number => {
  const workspace = store.workspaces.get(props.workspaceId);
  if (!workspace) return 0;

  let count = 0;
  const walk = (items: Item<[string, string]>[]) => {
    for (const item of items) {
      if (item.type === 'item') {
        if (item.value[0] === connectionId) count++;
      } else {
        walk(item.value.items);
      }
    }
  };
  walk(workspace.devices.items);
  return count;
};

// Folder editing
const startEditing = () => {
  isEditing.value = true;
  editingName.value = props.folder.name;
  nextTick(() => {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        nameInput.value?.focus();
        nameInput.value?.select();
      });
    });
  });
};

const confirmEdit = async () => {
  if (editingName.value.trim() && editingName.value !== props.folder.name) {
    await store.renameConnectionFolder(props.workspaceId, props.folder.id, editingName.value.trim());
  }
  isEditing.value = false;
};

const cancelEdit = () => {
  editingName.value = props.folder.name;
  isEditing.value = false;
};

const handleDelete = async () => {
  await store.deleteConnectionFolder(props.workspaceId, props.folder.id);
};

// Subfolder creation
const handleAddSubfolder = () => {
  isOpen.value = true;
  isCreatingSubfolder.value = true;
  nextTick(() => {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        subfolderNameInput.value?.focus();
      });
    });
  });
};

const handleAddConnection = async (connectionId: string) => {
  isAddingConnection.value = false;
  if (!connectionId || store.workspaceHasConnection(props.workspaceId, connectionId)) return;

  await store.addConnectionToWorkspace(props.workspaceId, connectionId);
  await store.moveConnectionToFolder(props.workspaceId, connectionId, props.folder.id);
};

const confirmCreateSubfolder = async () => {
  if (newSubfolderName.value.trim()) {
    isOpen.value = true;
    await store.createConnectionFolder(props.workspaceId, props.folder.id, newSubfolderName.value.trim());
    newSubfolderName.value = "";
    isCreatingSubfolder.value = false;
  } else {
    cancelCreateSubfolder();
  }
};

const cancelCreateSubfolder = () => {
  newSubfolderName.value = "";
  isCreatingSubfolder.value = false;
};

const handleRemoveFromWorkspace = (connectionId: string) => {
  removingWorkspaceDeviceCount.value = countAssociatedDevices(connectionId);
  removingWorkspaceConnectionId.value = connectionId;
};

const confirmRemoveFromWorkspace = async () => {
  if (!removingWorkspaceConnectionId.value) return;
  await store.removeConnectionFromWorkspace(props.workspaceId, removingWorkspaceConnectionId.value);
  removingWorkspaceConnectionId.value = null;
  removingWorkspaceDeviceCount.value = 0;
};

const handleRemoveConnection = (connectionId: string) => {
  removingConnectionId.value = connectionId;
};

const confirmRemoveConnection = async () => {
  if (!removingConnectionId.value) return;
  await connectionStore.removeConnection(removingConnectionId.value);
  removingConnectionId.value = null;
};
</script>
