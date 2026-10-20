<template>
  <AlertDialog v-model:open="isAddingDevice">
    <AddDeviceDialog :workspace-id="workspaceId" @confirm="handleAddDevice"/>
  </AlertDialog>
  <Collapsible v-model:open="isOpen" class="group/device-folder">
    <div ref="triggerEl">
      <CollapsibleTrigger
          class="group/button group/folder-trigger h-6 w-full justify-start rounded-md px-2 font-normal transition-colors duration-75 aria-expanded:bg-transparent aria-expanded:hover:bg-muted dark:aria-expanded:hover:bg-muted/50"
          :class="{ 'outline outline-1 outline-ring bg-accent/30': isOver }"
      >
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
                  group-data-[state=closed]/device-folder:text-muted-foreground
                  group-data-[state=closed]/device-folder:group-hover/folder-trigger:text-foreground
                "
              >
              {{ folder.name }}
            </span>
              <input
                  v-else
                  ref="nameInput"
                  v-model="editingName"
                  @blur="confirmEdit"
                  @keyup.enter="confirmEdit"
                  @keyup.esc="cancelEdit"
                  @click.stop
                  class="
                  text-xs font-normal
                  text-left flex-1
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
            <ContextMenuItem @click="isAddingDevice = true">
              <Icon icon="heroicons:cpu-chip"/>
              Add device
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
            class="flex h-7 w-full items-center gap-2 rounded-md px-2"
        >
          <Icon icon="heroicons:chevron-down" class="opacity-0" width="16" height="16"/>
          <Icon icon="heroicons:folder" class="text-muted-foreground" width="16" height="16"/>
          <input
              ref="subfolderNameInput"
              v-model="newSubfolderName"
              @blur="handleSubfolderBlur"
              @keyup.enter="confirmCreateSubfolder"
              @keyup.esc="cancelCreateSubfolder"
              @click.stop
              placeholder="Folder name..."
              class="
                text-xs font-normal
                text-left flex-1
                bg-background border border-border rounded px-1
                focus:outline-none focus:ring-1 focus:ring-ring
                text-foreground
                placeholder:text-muted-foreground
                h-[1.125rem] leading-[1.125rem]
              "
          />
        </div>

        <template v-for="(item, index) in folder.items" :key="index">
          <DeviceFolder
              v-if="item.type === 'folder'"
              :folder="item.value"
              :workspace-id="workspaceId"
          />

          <DeviceButton
              v-else-if="getResolved(item.value[1])"
              :connection="getResolved(item.value[1])!.connection"
              :device="getResolved(item.value[1])!.device"
              :workspace-id="workspaceId"
              :name="getResolved(item.value[1])!.device.name"
          />
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
import AddDeviceDialog from "@/components/monitor/dialogs/AddDeviceDialog.vue";
import type {Item, LabeqConnection, LabeqDevice, NestedFolder} from "@/lib/labeq.ts";
import {computed, nextTick, onMounted, ref, watchEffect} from "vue";
import {useWorkspaceStore} from "@/store/workspace.store.ts";
import {useConnectionStore} from "@/store/connection.store"; // <-- required
import {useDragTarget} from "@/composables/useDrag.ts";
import DeviceButton from "@/components/monitor/sidebar/device/DeviceButton.vue";

const props = defineProps<{
  folder: NestedFolder<string[]>
  workspaceId: string
}>();

const store = useWorkspaceStore();
const connectionStore = useConnectionStore();

const triggerEl = ref<HTMLElement | null>(null);
const {isOver, register} = useDragTarget(
    () => triggerEl.value,
    'device',
    (deviceId) => store.moveDeviceToFolder(props.workspaceId, deviceId, props.folder.id),
    10
);
onMounted(register);
watchEffect(register);

/* ------------------ helpers ------------------ */

function flattenFolder<T>(folder: NestedFolder<T>): T[] {
  const result: T[] = [];

  const walk = (items: Item<T>[]) => {
    for (const item of items) {
      if (item.type === 'item') {
        result.push(item.value);
      } else {
        walk(item.value.items);
      }
    }
  };

  walk(folder.items);
  return result;
}

/* ------------------ computed ------------------ */

type ResolvedDevice = {
  connection: LabeqConnection;
  device: LabeqDevice;
};

const resolvedDeviceMap = computed(() => {
  const map = new Map<string, ResolvedDevice>();

  const tuples = flattenFolder(props.folder) as [string, string][];

  for (const [connectionId, deviceId] of tuples) {
    const connection = connectionStore.connections.get(connectionId);
    if (!connection) continue;

    const device = connection.devices.find(d => d.uuid === deviceId);
    if (!device) continue;

    map.set(deviceId, {connection, device});
  }

  return map;
});

function getResolved(deviceId: string) {
  return resolvedDeviceMap.value.get(deviceId);
}

const itemCount = computed(() =>
    props.folder.items.filter(item => item.type === 'item').length
);

/* ------------------ UI state ------------------ */

const isEditing = ref(false);
const isOpen = ref(false);
const editingName = ref(props.folder.name);
const nameInput = ref<HTMLInputElement | null>(null);

const isCreatingSubfolder = ref(false);
const newSubfolderName = ref("");
const subfolderNameInput = ref<HTMLInputElement | null>(null);
const isAddingDevice = ref(false);

/* ------------------ actions ------------------ */

const startEditing = () => {
  isEditing.value = true;
  editingName.value = props.folder.name;
  nextTick(() => {
    nameInput.value?.focus();
    nameInput.value?.select();
  });
};

const confirmEdit = async () => {
  if (editingName.value.trim() && editingName.value !== props.folder.name) {
    await store.renameDeviceFolder(props.workspaceId, props.folder.id, editingName.value.trim());
  }
  isEditing.value = false;
};

const cancelEdit = () => {
  editingName.value = props.folder.name;
  isEditing.value = false;
};

const handleDelete = async () => {
  await store.deleteDeviceFolder(props.workspaceId, props.folder.id);
};

const handleAddSubfolder = () => {
  window.setTimeout(() => {
    isOpen.value = true;
    isCreatingSubfolder.value = true;
    nextTick(() => subfolderNameInput.value?.focus());
  }, 0);
};

const handleAddDevice = async (connectionId?: string, deviceId?: string) => {
  isAddingDevice.value = false;
  if (!connectionId || !deviceId) return;

  await store.addConnectionToWorkspace(props.workspaceId, connectionId);
  await store.addDeviceToWorkspace(props.workspaceId, connectionId, deviceId);
  await store.moveDeviceToFolder(props.workspaceId, deviceId, props.folder.id);
};

const confirmCreateSubfolder = async () => {
  if (newSubfolderName.value.trim()) {
    isOpen.value = true;
    await store.createDeviceFolder(props.workspaceId, props.folder.id, newSubfolderName.value.trim());
    newSubfolderName.value = "";
    isCreatingSubfolder.value = false;
  } else {
    cancelCreateSubfolder();
  }
};

const handleSubfolderBlur = () => {
  window.setTimeout(() => {
    if (newSubfolderName.value.trim()) {
      confirmCreateSubfolder();
    } else {
      cancelCreateSubfolder();
    }
  }, 150);
};

const cancelCreateSubfolder = () => {
  newSubfolderName.value = "";
  isCreatingSubfolder.value = false;
};

// const handleRemoveDevice = async (deviceId: string) => {
//   if (confirm('Remove this device from workspace?')) {
//     await store.removeDeviceFromWorkspace(props.workspaceId, deviceId);
//   }
// };
</script>