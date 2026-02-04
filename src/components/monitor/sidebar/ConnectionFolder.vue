<template>
  <Collapsible class="group/connection-folder">
    <CollapsibleTrigger
        class="group/folder-trigger transition-colors duration-75 cursor-pointer w-full px-2 py-1 rounded-md hover:bg-accent/50">
      <ContextMenu>
        <ContextMenuTrigger asChild>
          <div class="flex items-center space-x-2 w-full">
            <Icon
                icon="heroicons:chevron-down"
                class="transition-transform duration-300 group-data-[state=closed]/connection-folder:-rotate-90 text-accent-foreground"
                width="16"
                height="16"
            />
            <Icon
                icon="heroicons:folder"
                class="text-muted-foreground"
                width="16"
                height="16"
            />
            <TooltipProvider v-if="!isEditing">
              <TooltipRoot :delayDuration="3000">
                <TooltipTrigger
                    @dblclick="startEditing"
                    class="
                      text-xs font-normal tracking-wide
                      text-left flex-1
                      whitespace-nowrap overflow-hidden overflow-ellipsis
                      cursor-pointer transition-colors
                      text-foreground
                      group-data-[state=closed]/connection-folder:text-muted-foreground
                      group-data-[state=closed]/connection-folder:group-hover/folder-trigger:text-foreground
                    "
                >
                  {{ folder.name }}
                </TooltipTrigger>
                <TooltipContent>
                  <p>{{ folder.name }}</p>
                </TooltipContent>
              </TooltipRoot>
            </TooltipProvider>
            <input
                v-else
                ref="nameInput"
                v-model="editingName"
                @blur="confirmEdit"
                @keyup.enter="confirmEdit"
                @keyup.esc="cancelEdit"
                @click.stop
                class="
                  text-xs font-normal tracking-wide
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
          <ContextMenuItem @click="handleAddSubfolder" class="cursor-pointer">
            <Icon icon="heroicons:folder-plus" width="16" height="16" class="mr-2"/>
            Add subfolder
          </ContextMenuItem>
          <ContextMenuItem @click="startEditing" class="cursor-pointer">
            <Icon icon="heroicons:pencil" width="16" height="16" class="mr-2"/>
            Rename
          </ContextMenuItem>
          <ContextMenuItem @click="handleDelete" class="cursor-pointer text-destructive">
            <Icon icon="heroicons:trash" width="16" height="16" class="mr-2"/>
            Delete
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>
    </CollapsibleTrigger>
    <CollapsibleContent>
      <div class="ml-4 space-y-1">
        <div
            v-if="isCreatingSubfolder"
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
              ref="subfolderNameInput"
              v-model="newSubfolderName"
              @blur="handleSubfolderBlur"
              @keyup.enter="confirmCreateSubfolder"
              @keyup.esc="cancelCreateSubfolder"
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

        <template v-for="(item, index) in folder.items" :key="index">
          <ConnectionFolder
              v-if="item.type === 'folder'"
              :folder="item.value"
              :workspace-id="workspaceId"
          />

          <div
              v-else
              class="flex items-center space-x-2 w-full px-2 py-1 rounded-md hover:bg-accent/50 transition-colors cursor-pointer group"
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
                  {{ item.value.name }}
                </TooltipTrigger>
                <TooltipContent>
                  <div class="space-y-1">
                    <p><span class="font-semibold">Name:</span> {{ item.value.name }}</p>
                    <p><span class="font-semibold">Type:</span> {{ item.value.type }}</p>
                    <p v-if="item.value.description"><span class="font-semibold">Description:</span>
                      {{ item.value.description }}</p>
                  </div>
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
      </div>
    </CollapsibleContent>
  </Collapsible>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {Collapsible, CollapsibleContent, CollapsibleTrigger} from "@/components/ui/collapsible";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {TooltipRoot} from "reka-ui";
import type {NestedFolder} from "@/lib/labeq.ts";
import {computed, nextTick, ref} from "vue";
import {useWorkspaceSore} from "@/store/workspace.store.ts";

const props = defineProps<{
  folder: NestedFolder<any>
  workspaceId: string
}>()

const store = useWorkspaceSore();

const isEditing = ref(false);
const editingName = ref(props.folder.name);
const nameInput = ref<HTMLInputElement | null>(null);

const isCreatingSubfolder = ref(false);
const newSubfolderName = ref("");
const subfolderNameInput = ref<HTMLInputElement | null>(null);

const itemCount = computed(() => {
  return props.folder.items.filter(item => item.type === 'item').length;
});

// Folder editing
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
    await store.renameConnectionFolder(props.workspaceId, props.folder.id, editingName.value.trim());
  }
  isEditing.value = false;
};

const cancelEdit = () => {
  editingName.value = props.folder.name;
  isEditing.value = false;
};

const handleDelete = async () => {
  if (confirm(`Delete folder "${props.folder.name}" and all its contents?`)) {
    await store.deleteConnectionFolder(props.workspaceId, props.folder.id);
  }
};

// Subfolder creation
const handleAddSubfolder = () => {
  isCreatingSubfolder.value = true;
  nextTick(() => {
    subfolderNameInput.value?.focus();
  });
};

const confirmCreateSubfolder = async () => {
  if (newSubfolderName.value.trim()) {
    await store.createConnectionFolder(props.workspaceId, props.folder.id, newSubfolderName.value.trim());
    newSubfolderName.value = "";
    isCreatingSubfolder.value = false;
  } else {
    cancelCreateSubfolder();
  }
};

const handleSubfolderBlur = () => {
  setTimeout(() => {
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

const handleRemoveConnection = async (connectionId: string) => {
  if (confirm('Remove this connection from workspace?')) {
    await store.removeConnectionFromWorkspace(props.workspaceId, connectionId);
  }
};
</script>