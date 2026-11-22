<template>
  <ContextMenu>
    <ContextMenuTrigger as-child>
      <div class="w-full">
        <Collapsible class="group/workspaces-section">
          <SidebarGroupLabel class="flex items-center gap-2 group/workspaces-section-label">
            <div class="flex items-center gap-2 cursor-default">
              <CollapsibleTrigger class="w-full cursor-pointer" as-child>
                <Icon
                    icon="heroicons:chevron-down"
                    class="transition-transform duration-300 group-data-[state=closed]/workspaces-section:rotate-180 -ml-2 text-accent-foreground self-end"
                    width="16"
                    height="16"
                />
              </CollapsibleTrigger>
              <p>Workspaces</p>
            </div>
          </SidebarGroupLabel>
          <CollapsibleContent>
            <SidebarMenu>
              <Empty
                  v-if="!workspacesPresent && !isCreating"
                  class="h-9 min-h-9 max-h-9 p-0 md:p-0 text-muted-foreground text-xs rounded-md bg-card cursor-default select-none"
              >
                <EmptyHeader>No workspaces present...</EmptyHeader>
              </Empty>

              <Collapsible
                  v-if="isCreating"
                  :open="true"
                  class="group/collapsible"
              >
                <WorkspaceCreator
                    @create="handleCreateWorkspace"
                    @cancel="cancelCreatingWorkspace"
                />
              </Collapsible>

              <Collapsible
                  v-for="workspace in workspaceStore.workspaces.values()"
                  :key="workspace.id"
                  class="group/collapsible"
              >
                <WorkspaceItem
                    :workspace="workspace"
                    @rename="handleRenameWorkspace"
                    @delete="handleDeleteWorkspace"
                    @create-folder="handleCreateFolder"
                    @rename-folder="handleRenameFolder"
                    @delete-folder="handleDeleteFolder"
                />
              </Collapsible>
            </SidebarMenu>
          </CollapsibleContent>
        </Collapsible>
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent>
      <ContextMenuItem @click="startCreatingWorkspace">
        <Icon icon="heroicons:plus"/>
        New workspace
      </ContextMenuItem>
      <ContextMenuItem @click="openWorkspace">
        <Icon icon="heroicons:folder-open"/>
        Open workspace...
      </ContextMenuItem>
      <ContextMenuItem
          v-if="workspaceStore.dirtyWorkspaceIds.size > 0"
          @click="saveAllWorkspaces"
      >
        <Icon icon="heroicons:document-arrow-down"/>
        Save all
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {Collapsible, CollapsibleContent, CollapsibleTrigger} from "@/components/ui/collapsible";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {SidebarGroupLabel, SidebarMenu} from "@/components/ui/sidebar";
import {useWorkspaceStore} from "@/store/workspace.store.ts";
import {computed, nextTick, ref} from "vue";
import type {LabeqWorkspace} from "@/lib/labeq.ts";
import WorkspaceCreator from "@/components/monitor/sidebar/workspace/WorkspaceCreator.vue";
import WorkspaceItem from "@/components/monitor/sidebar/workspace/WorkspaceItem.vue";
import {Empty, EmptyHeader} from "@/components/ui/empty";

const workspacesPresent = computed(() => workspaceStore.workspaces.size > 0);
const workspaceStore = useWorkspaceStore();
const isCreating = ref(false);
const openWorkspace = () => workspaceStore.openWorkspace();

const saveAllWorkspaces = async () => {
  for (const workspaceId of workspaceStore.dirtyWorkspaceIds) {
    await workspaceStore.saveWorkspace(workspaceId);
  }
};

const startCreatingWorkspace = () => {
  isCreating.value = true;
  nextTick(() => {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        const input = document.querySelector<HTMLInputElement>(
            '[placeholder="Workspace name..."]',
        );
        input?.focus();
        input?.select();
      });
    });
  });
};

const cancelCreatingWorkspace = () => {
  isCreating.value = false;
};

const handleCreateWorkspace = (name: string) => {
  workspaceStore.createWorkspace(name);
  isCreating.value = false;
};

const handleRenameWorkspace = (workspace: LabeqWorkspace, newName: string) => {
  workspaceStore.renameWorkspace(workspace.id, newName);
};

const handleDeleteWorkspace = (workspace: LabeqWorkspace) => {
  workspaceStore.deleteWorkspace(workspace.id);
};

const handleCreateFolder = (workspaceId: string, parentId: string | undefined, folderName: string) => {
  workspaceStore.createDeviceFolder(workspaceId, parentId, folderName);
};

const handleRenameFolder = (workspaceId: string, folderId: string, newName: string) => {
  workspaceStore.renameDeviceFolder(workspaceId, folderId, newName);
};

const handleDeleteFolder = (workspaceId: string, folderId: string) => {
  workspaceStore.deleteDeviceFolder(workspaceId, folderId);
};
</script>

<style lang="scss" scoped>

</style>