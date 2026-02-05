<template>
  <AlertDialog v-model:open="isAddingDevice" v-model:device="selectedDevice" v-model:connection="selectedConnection"
               @confirm="handleDeviceConfirm">
    <AddDeviceDialog :workspace-id="workspaceId"/>
  </AlertDialog>
  <ContextMenu>
    <ContextMenuTrigger>
      <div class="text-xs font-semibold text-foreground px-2 pb-1">
        Devices
      </div>
      <div>
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

          <div
              v-else
              class="
          group flex items-center gap-2 w-full px-2 h-8
          rounded-md hover:bg-accent/50 transition-colors cursor-pointer
        "
          >
            <Icon
                icon="heroicons:cpu-chip"
                class="text-muted-foreground ml-4 shrink-0"
                width="16"
                height="16"
            />

            <div class="flex-1 min-w-0 flex items-center">
              <TooltipProvider>
                <TooltipRoot :delayDuration="3000">
                  <TooltipTrigger as-child>
                    <div
                        class="
                    text-xs font-normal truncate
                    text-left w-full
                    text-foreground
                  "
                    >
                      {{ item.value.name }}
                    </div>
                  </TooltipTrigger>
                  <TooltipContent>
                    <div class="space-y-1">
                      <p><span class="font-semibold">Name:</span> {{ item.value.name }}</p>
                      <p><span class="font-semibold">Model:</span> {{ item.value.model }}</p>
                      <p><span class="font-semibold">Manufacturer:</span> {{ item.value.manufacturer }}</p>
                    </div>
                  </TooltipContent>
                </TooltipRoot>
              </TooltipProvider>
            </div>

            <button
                @click.stop="handleRemoveDevice(item.value.uuid)"
                class="
            opacity-0 group-hover:opacity-100
            flex items-center justify-center
            h-6 w-6 shrink-0
            hover:bg-destructive/10 rounded transition-all
          "
            >
              <Icon icon="heroicons:trash" width="14" height="14" class="text-destructive shrink-0"/>
            </button>
          </div>
        </template>

        <div
            v-if="deviceFolders.items.length === 0 && !isCreatingFolder"
            class="flex items-center px-2 py-1"
        >
          <span class="text-muted-foreground text-xs">No devices</span>
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
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {TooltipRoot} from "reka-ui";
import DeviceFolder from "./DeviceFolder.vue";
import type {Folder, LabeqDevice} from "@/lib/labeq.ts";
import {computed, nextTick, ref} from "vue";
import {useWorkspaceSore} from "@/store/workspace.store.ts";
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

const props = defineProps<{
  workspaceId: string
  deviceFolders: Folder<LabeqDevice>
}>();

const connectionStore = useConnectionStore();
const store = useWorkspaceSore();

const isCreatingFolder = ref(false);
const isAddingDevice = ref(false);
const newFolderName = ref("");
const folderNameInput = ref<HTMLInputElement | null>(null);

const selectedConnectionId = ref<string>("");
const selectedDeviceId = ref<string>("");

const selectedConnection = computed(() => connectionStore.connections.get(selectedConnectionId.value));
const selectedDevice = computed(() => selectedConnection.value?.devices.find(d => d.uuid === selectedDeviceId.value));

const handleDeviceConfirm = async () => {
  isAddingDevice.value = false;
  if (!selectedConnection.value || !selectedDevice.value) return;
  await store.addConnectionToWorkspace(props.workspaceId, selectedConnection.value);
  await store.addDeviceToWorkspace(props.workspaceId, selectedDevice.value);
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

const handleRemoveDevice = async (deviceId: string) => {
  if (confirm('Remove this device from workspace?')) {
    await store.removeDeviceFromWorkspace(props.workspaceId, deviceId);
  }
};
</script>