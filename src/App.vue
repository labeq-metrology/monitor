<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {computed, onMounted, ref} from "vue";
import {listen} from "@tauri-apps/api/event";
import {getCurrentWindow} from "@tauri-apps/api/window";
import ConfirmDialog from "@/components/monitor/dialogs/ConfirmDialog.vue";

import {Toaster} from '@/components/ui/sonner'
import Navbar from "@/components/monitor/navbar/Navbar.vue";
import TabBar from "@/components/monitor/navbar/TabBar.vue";
import {SidebarProvider} from "@/components/ui/sidebar";
import AppSidebar from "@/components/monitor/sidebar/AppSidebar.vue";
import 'vue-sonner/style.css'
import {useWorkspaceStore} from "@/store/workspace.store.ts";
import DragGhost from "@/components/drag/DragGhost.vue";
import {useDragSource} from '@/composables/useDrag';
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";

const {ghost} = useDragSource();
const visualModels = useVisualModelsStore();
const workspaceStore = useWorkspaceStore();
const unsavedWorkspaceIds = ref<string[]>([]);
const showCloseDialog = ref(false);
void listen<{workspaces: string[]}>("unsaved-work-warning", event => {
  unsavedWorkspaceIds.value = event.payload.workspaces;
  showCloseDialog.value = true;
});
const saveAndClose = async () => {
  for (const workspaceId of unsavedWorkspaceIds.value) {
    await invoke("save_workspace", {workspaceId});
  }
  showCloseDialog.value = false;
  await getCurrentWindow().close();
};
const closeWithoutSaving = async () => {
  await invoke("confirm_close");
  showCloseDialog.value = false;
  await getCurrentWindow().close();
};
const workspaceLoadReportDescription = computed(() => {
  const report = workspaceStore.loadReport;
  if (!report) return "";
  const connectionCount = report.missingConnections.length;
  const deviceCount = report.missingDevices.length;
  const parts = [];
  if (connectionCount > 0) {
    parts.push(`${connectionCount} connection${connectionCount === 1 ? "" : "s"}`);
  }
  if (deviceCount > 0) {
    parts.push(`${deviceCount} device${deviceCount === 1 ? "" : "s"}`);
  }
  if (parts.length === 0) {
    return `Workspace "${report.workspaceName}" was loaded successfully.`;
  }
  return `Workspace "${report.workspaceName}" was loaded, but ${parts.join(" and ")} could not be found.`;
});
const workspaceLoadReportDetails = computed(() => {
  const report = workspaceStore.loadReport;
  if (!report) return [];
  return [
    ...report.missingConnections.map(id => `Connection “${id}”`),
    ...report.missingDevices.map(device =>
      `Device “${device.deviceUuid}” from connection “${device.connectionId}”`
    ),
  ];
});
onMounted(async () => {
  try {
    (window as any).Vue = await import('vue');
    (window as any).Pinia = await import('pinia');
    await invoke('init_plugins');
    const manifests = await invoke<string[]>('list_plugins');
    const plugins = manifests.map(manifest => JSON.parse(manifest));
    const loadPanels = () => {
      try {
        visualModels.loadPluginPanels(plugins);
      } catch (error) {
        console.error('Failed to load plugin panels:', error);
      }
    };
    if ('requestIdleCallback' in window) {
      window.requestIdleCallback(loadPanels);
    } else {
      setTimeout(loadPanels, 0);
    }
  } catch (error) {
    console.error('Failed to initialize plugins:', error);
  }
});
</script>

<template>
  <div>
    <ConfirmDialog
        :open="workspaceStore.loadReport !== null"
        @update:open="open => { if (!open) workspaceStore.clearLoadReport() }"
        title="Workspace load report"
        :description="workspaceLoadReportDescription"
        :details="workspaceLoadReportDetails"
        confirm-label="Close"
        @confirm="workspaceStore.clearLoadReport"
    />
    <ConfirmDialog
        v-model:open="showCloseDialog"
        title="Unsaved workspace changes"
        :description="`${unsavedWorkspaceIds.length} workspace${unsavedWorkspaceIds.length === 1 ? '' : 's'} have unsaved changes. Save before closing?`"
        confirm-label="Save and close"
        @confirm="saveAndClose"
        @cancel="closeWithoutSaving"
    />
    <Toaster :toast-options="{ style: { } }"/>
    <SidebarProvider class="flex">
      <AppSidebar/>
      <main class="w-full">
        <header>
          <Navbar/>
          <TabBar/>
        </header>
        <article class="p-5 justify-self-center flex w-full xl:w-3/4 justify-center">
          <Suspense>
            <router-view class="justify-self-center w-full" v-slot="{ Component, route }">
              <keep-alive>
                <component :is="Component" :key="route.fullPath" />
              </keep-alive>
            </router-view>
          </Suspense>
        </article>
      </main>
    </SidebarProvider>
    <DragGhost :ghost="ghost"/>
  </div>
</template>

<style scoped>
html, body {
  @apply h-full w-full m-[0] p-[0];
}
</style>

<style>
html, body {
  @apply h-full w-full m-[0] p-[0];
}
</style>
