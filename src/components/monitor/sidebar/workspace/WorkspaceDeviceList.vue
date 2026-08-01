<template>
  <AlertDialog v-model:open="isAddingDevice">
    <AddDeviceDialog :workspace-id="workspaceId" @confirm="handleDeviceConfirm"/>
  </AlertDialog>
  <ContextMenu>
    <ContextMenuTrigger>
      <div class="text-xs font-semibold text-foreground px-2 pb-1">
        Devices
      </div>
      <div
          ref="rootDropEl"
          :class="{ 'ring-1 ring-ring ring-inset rounded-md': isOver }"
      >
        <div
            v-if="isCreatingFolder"
            class="flex items-center gap-2 w-full px-2 h-8 rounded-md"
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

        <template v-for="(item, index) in deviceFolders.items" :key="index">

          <DeviceFolder
              v-if="item.type === 'folder'"
              :folder="item.value"
              :workspace-id="workspaceId"
          />

          <div v-else>
            <DeviceButton
                :device="connectionStore.connections.get(item.value[0])!!.devices.find(d => d.uuid === item.value[1])!!"
                :connection="connectionStore.connections.get(item.value[0])!!"/>
          </div>
        </template>

        <div
            v-if="deviceFolders.items.length === 0 && !isCreatingFolder"
            class="flex items-center px-2 py-1"
        >
          <span class="text-muted-foreground text-xs select-none cursor-default">No devices</span>
        </div>
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent class="overflow-hidden">
      <ContextMenuSub>
        <ContextMenuSubTrigger class="text-xs py-1 px-2">Add</ContextMenuSubTrigger>
        <ContextMenuSubContent :align-offset="-5">
          <ContextMenuItem @click.prevent="startCreatingFolder">
            <Icon icon="heroicons:folder-16-solid"/>
            Folder
          </ContextMenuItem>
          <ContextMenuItem @click="isAddingDevice = true">
            <Icon icon="heroicons:cpu-chip-16-solid"/>
            Device
          </ContextMenuItem>
        </ContextMenuSubContent>
      </ContextMenuSub>
    </ContextMenuContent>
  </ContextMenu>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import DeviceFolder from "../device/DeviceFolder.vue";
import {Folder} from "@/lib/labeq.ts";
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
import AddDeviceDialog from "@/components/monitor/dialogs/AddDeviceDialog.vue";
import {useConnectionStore} from "@/store/connection.store.ts";
import DeviceButton from "@/components/monitor/sidebar/device/DeviceButton.vue";
import {useDragTarget} from "@/composables/useDrag.ts";

defineEmits<{
  createFolder: [workspaceId: string, parentId: string | undefined, folderName: string]
  renameFolder: [workspaceId: string, folderId: string, newName: string]
  deleteFolder: [workspaceId: string, folderId: string]
}>();

const props = defineProps<{
  workspaceId: string
  deviceFolders: Folder<[string, string]>
}>();

const connectionStore = useConnectionStore();
const store = useWorkspaceStore();
const rootDropEl = ref<HTMLElement | null>(null);
const { isOver, register } = useDragTarget(
    () => rootDropEl.value,
    'device',
    (deviceId) => store.moveDeviceToFolder(props.workspaceId, deviceId, undefined)
);
onMounted(register);

const isCreatingFolder = ref(false);
const isAddingDevice = ref(false);
const newFolderName = ref("");
const folderNameInput = ref<HTMLInputElement | null>(null);
const handleDeviceConfirm = async (connectionId: string, deviceId: string) => {
  isAddingDevice.value = false;

  if (!connectionId || !deviceId) return;
  const selectedConnection = connectionStore.connections.get(connectionId);
  const selectedDevice = selectedConnection?.devices.find(d => d.uuid === deviceId);

  if (!selectedConnection || !selectedDevice) return;
  await store.addConnectionToWorkspace(props.workspaceId, selectedConnection.id);
  await store.addDeviceToWorkspace(props.workspaceId, selectedConnection.id, selectedDevice.uuid);
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
    await store.createDeviceFolder(props.workspaceId, undefined, newFolderName.value.trim());
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

// const handleRemoveDevice = async (deviceId: string) => {
//   if (confirm('Remove this device from workspace?')) {
//     await store.removeDeviceFromWorkspace(props.workspaceId, deviceId);
//   }
// };
</script>