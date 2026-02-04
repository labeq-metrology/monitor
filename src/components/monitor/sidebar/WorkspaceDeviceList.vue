<template>
  <div class="space-y-0.5">
    <div
        v-if="isCreatingFolder"
        class="flex items-center gap-2 w-full px-2 h-8 rounded-md"
    >
      <div class="w-4 h-4 shrink-0" />

      <Icon
          icon="heroicons:folder"
          class="text-muted-foreground shrink-0"
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
          flex-1 min-w-0 h-6
          text-xs font-normal
          bg-background border border-border rounded px-1.5
          focus:outline-none focus:ring-1 focus:ring-ring
          text-foreground placeholder:text-muted-foreground
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
</template>

<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { TooltipRoot } from "reka-ui";
import DeviceFolder from "./DeviceFolder.vue";
import type { Folder, LabeqDevice } from "@/lib/labeq.ts";
import { nextTick, ref } from "vue";
import { useWorkspaceSore } from "@/store/workspace.store.ts";

const props = defineProps<{
  workspaceId: string
  deviceFolders: Folder<LabeqDevice>
}>();

const store = useWorkspaceSore();

const isCreatingFolder = ref(false);
const newFolderName = ref("");
const folderNameInput = ref<HTMLInputElement | null>(null);

const startCreatingFolder = () => {
  isCreatingFolder.value = true;
  nextTick(() => {
    folderNameInput.value?.focus();
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