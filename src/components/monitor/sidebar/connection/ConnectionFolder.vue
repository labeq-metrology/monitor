<template>
  <Collapsible class="group/connection-folder">
    <div ref="triggerEl">
      <CollapsibleTrigger
          class="group/folder-trigger transition-colors duration-75 cursor-pointer w-full px-2 py-1 rounded-md hover:bg-accent/50"
          :class="{ 'outline outline-1 outline-ring bg-accent/30': isOver }">
        <ContextMenu>
          <ContextMenuTrigger as-child>
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
              <span
                  v-if="!isEditing"
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
                  text-xs font-normal tracking-wide
                  text-left flex-1
                  bg-background border border-border rounded px-1
                  focus:outline-none focus:ring-1 focus:ring-ring
                  text-foreground
                  h-[1.125rem] leading-[1.125rem]
                "
              />
              <span class="text-xs text-muted-foreground h-[1.125rem] leading-[1.125rem]">{{ itemCount }}</span>
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
    </div>
    <CollapsibleContent>
      <div class="ml-4">
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
      </div>
    </CollapsibleContent>
  </Collapsible>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {Collapsible, CollapsibleContent, CollapsibleTrigger} from "@/components/ui/collapsible";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import type {NestedFolder} from "@/lib/labeq.js";
import {useConnectionStore} from "@/store/connection.store.ts";
import {computed, nextTick, onMounted, ref, watchEffect} from "vue";
import {useWorkspaceStore} from "@/store/workspace.store.js";
import {useDragSource, useDragTarget} from "@/composables/useDrag.ts";

const props = defineProps<{
  folder: NestedFolder<string>
  workspaceId: string
}>();

const store = useWorkspaceStore();
const connectionStore = useConnectionStore();

const { startDrag } = useDragSource();

const triggerEl = ref<HTMLElement | null>(null);
const { isOver, register } = useDragTarget(
    () => triggerEl.value,
    'connection',
    (connectionId) => store.moveConnectionToFolder(props.workspaceId, connectionId, props.folder.id)
);
onMounted(register);
watchEffect(register);

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
  await store.deleteConnectionFolder(props.workspaceId, props.folder.id);
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