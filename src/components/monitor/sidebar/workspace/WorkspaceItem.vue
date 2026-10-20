<template>
  <div class="bg-background rounded-md px-2">
    <CollapsibleTrigger
        class="h-9 min-h-9 max-h-9 group/workspace transition-colors duration-75 cursor-pointer w-full px-2 py-2 rounded-md">
      <ContextMenu>
        <ContextMenuTrigger asChild class="text-sm text-accent-background">
          <div class="flex items-center space-x-2 w-full">
            <Icon
                icon="heroicons:chevron-down"
                class="text-lg items-center transition-transform duration-300 group-data-[state=closed]/collapsible:rotate-180 -ml-2 text-accent-foreground"
                width="16"
                height="16"
            />
            <Icon
                icon="heroicons:folder"
                class="text-muted-foreground text-lg"
            />
            <TooltipProvider v-if="!isEditing">
              <TooltipRoot :delayDuration="3000">
                <TooltipTrigger
                    @dblclick="startEditing"
                    class="
                      text-xs font-normal tracking-wide
                      text-left max-w-8/12
                      whitespace-nowrap overflow-hidden overflow-ellipsis
                      cursor-pointer transition-colors
                      text-foreground
                      group-data-[state=closed]/collapsible:text-muted-foreground
                      group-data-[state=closed]/collapsible:group-hover/collapsible:text-foreground
                    "
                >
                  {{ workspace.name }}
                </TooltipTrigger>
                <TooltipContent>
                  <p>{{ workspace.name }}</p>
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
                  text-left max-w-8/12
                  bg-background border border-border rounded px-1
                  focus:outline-none focus:ring-1 focus:ring-ring
                  text-foreground
                  h-[1.125rem] leading-[1.125rem]
                "
            />
          </div>
        </ContextMenuTrigger>
        <ContextMenuContent>
          <ContextMenuItem @click="startEditing">
            <Icon icon="heroicons:pencil"/>
            Rename
          </ContextMenuItem>
          <ContextMenuItem @click="emit('delete', workspace)" variant="destructive">
            <Icon icon="heroicons:trash"/>
            Delete
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>
    </CollapsibleTrigger>

    <CollapsibleContent>
      <div class="ml-4 space-y-1 pt-0 pb-2 overflow-hidden">
        <div>
          <WorkspaceConnectionList :workspace-id="workspace.id" :connections="workspace.connections"/>
        </div>
        <!-- Devices Section -->
        <div>
          <WorkspaceDeviceList
              :workspace-id="workspace.id"
              :device-folders="workspace.devices"
              @create-folder="handleCreateFolder"
              @rename-folder="handleRenameFolder"
              @delete-folder="handleDeleteFolder"
          />
        </div>
      </div>
    </CollapsibleContent>
  </div>
</template>

<script setup lang="ts">
import {TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";
import {Icon} from "@iconify/vue";
import {CollapsibleContent, CollapsibleTrigger, TooltipRoot} from "reka-ui";
import {LabeqWorkspace} from "@/lib/labeq.ts";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {nextTick, ref} from "vue";
import WorkspaceDeviceList from "@/components/monitor/sidebar/workspace/WorkspaceDeviceList.vue";
import WorkspaceConnectionList from "@/components/monitor/sidebar/workspace/WorkspaceConnectionList.vue";

const props = defineProps<{
  workspace: LabeqWorkspace
}>();

const emit = defineEmits<{
  rename: [workspace: LabeqWorkspace, newName: string]
  delete: [workspace: LabeqWorkspace]
  createFolder: [workspaceId: string, parentId: string | undefined, folderName: string]
  renameFolder: [workspaceId: string, folderId: string, newName: string]
  deleteFolder: [workspaceId: string, folderId: string]
}>();

const isEditing = ref(false);
const editingName = ref(props.workspace.name);
const nameInput = ref<HTMLInputElement | null>(null);

const startEditing = () => {
  isEditing.value = true;
  editingName.value = props.workspace.name;
  nextTick(() => {
    nameInput.value?.focus();
    nameInput.value?.select();
  });
};

const confirmEdit = () => {
  if (editingName.value.trim() && editingName.value !== props.workspace.name) {
    emit('rename', props.workspace, editingName.value.trim());
  }
  isEditing.value = false;
};

const cancelEdit = () => {
  editingName.value = props.workspace.name;
  isEditing.value = false;
};

const handleCreateFolder = (workspaceId: string, parentId: string | undefined, folderName: string) => {
  emit('createFolder', workspaceId, parentId, folderName);
};

const handleRenameFolder = (workspaceId: string, folderId: string, newName: string) => {
  emit('renameFolder', workspaceId, folderId, newName);
};

const handleDeleteFolder = (workspaceId: string, folderId: string) => {
  emit('deleteFolder', workspaceId, folderId);
};
</script>