<template>
  <div class="space-y-1">
    <div
        v-if="isCreatingFolder"
        class="flex items-center space-x-2 w-full px-2 py-1 rounded-md"
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
          ref="folderNameInput"
          v-model="newFolderName"
          @blur="handleFolderBlur"
          @keyup.enter="confirmCreateFolder"
          @keyup.esc="cancelCreateFolder"
          @click.stop
          placeholder="Folder name..."
          class="
            text-xs font-normal tracking-wide
            text-left flex-1
            bg-background border border-border rounded px-1
            focus:outline-none focus:ring-1 focus:ring-ring
            text-foreground
            placeholder:text-muted-foreground
            h-[1.125rem] leading-[1.125rem]
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
      <div
          v-else
          class="flex items-center w-full px-2 py-1 rounded-md hover:bg-accent/50 transition-colors cursor-pointer group"
      >
        <Icon
            icon="heroicons:link"
            class="text-muted-foreground ml-4"
            width="16"
            height="16"
        />
        <TooltipProvider>
          <TooltipRoot :delayDuration="3000">
            <TooltipTrigger
                class="
                  text-xs font-normal tracking-wide
                  text-left flex-1
                  whitespace-nowrap overflow-hidden overflow-ellipsis
                  cursor-pointer transition-colors
                  text-foreground
                "
            >
              {{ item.value.id }}
            </TooltipTrigger>
            <TooltipContent>
              {{ item.value.id }}
            </TooltipContent>
          </TooltipRoot>
        </TooltipProvider>
        <button
            @click="handleRemoveConnection(item.value.id)"
            class="opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-destructive/10 rounded"
        >
          <Icon icon="heroicons:trash" width="14" height="14" class="text-destructive"/>
        </button>
      </div>
    </template>

    <div
        v-if="connections.items.length === 0 && !isCreatingFolder"
        class="text-muted-foreground text-xs px-2 py-1"
    >
      No connections
    </div>
  </div>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {TooltipRoot} from "reka-ui";
import ConnectionFolder from "./ConnectionFolder.vue";
import type {LabeqConnection, Folder} from "@/lib/labeq.ts";
import {ref, nextTick} from "vue";
import {useWorkspaceSore} from "@/store/workspace.store.ts";

const props = defineProps<{
  workspaceId: string
  connections: Folder<LabeqConnection>
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
    await store.createConnectionFolder(props.workspaceId, null, newFolderName.value.trim());
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
