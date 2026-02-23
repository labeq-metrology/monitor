<template>
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
        <div
            class="group-hover/workspaces-section-label:opacity-100 group-hover/workspaces-section-label:visible opacity-0 invisible transition-all duration-200 ml-auto">
          <Button
              @click.stop="startCreatingWorkspace"
              variant="link"
              class="w-4 h-4 m-0 p-0 flex items-center justify-center text-accent-foreground/50"
          >
            <Icon icon="heroicons:plus" width="16" height="16"/>
          </Button>
        </div>
      </SidebarGroupLabel>
    <CollapsibleContent>
      <SidebarMenu>
        <div class="text-muted-foreground text-xs mx-2 px-2 py-2 rounded-md bg-background" v-if="!workspacesPresent && !isCreating">
          <p>No workspaces present...</p>
        </div>

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
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {Button} from "@/components/ui/button";
import {Collapsible, CollapsibleContent, CollapsibleTrigger} from "@/components/ui/collapsible";
import {SidebarGroupLabel, SidebarMenu} from "@/components/ui/sidebar";
import {useWorkspaceStore} from "@/store/workspace.store.ts";
import {computed, ref} from "vue";
import type {LabeqWorkspace} from "@/lib/labeq.ts";
import WorkspaceCreator from "@/components/monitor/sidebar/workspace/WorkspaceCreator.vue";
import WorkspaceItem from "@/components/monitor/sidebar/workspace/WorkspaceItem.vue";

const workspacesPresent = computed(() => workspaceStore.workspaces.size > 0);
const workspaceStore = useWorkspaceStore();
const isCreating = ref(false);

const startCreatingWorkspace = () => {
  isCreating.value = true;
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