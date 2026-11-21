<template>
  <div class="w-full h-full flex flex-col min-h-0 gap-4">
    <header class="flex items-start justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold tracking-tight">Installed plugins</h2>
        <p class="text-sm text-muted-foreground mt-1">
          Extend Monitor with panels and device integrations.
        </p>
      </div>
      <Button variant="outline" :disabled="isReloading" @click="reloadPlugins">
        <Icon icon="heroicons:arrow-path" width="16" height="16" :class="{ 'animate-spin': isReloading }"/>
        <span>{{ isReloading ? 'Reloading...' : 'Reload' }}</span>
      </Button>
    </header>

    <section class="grid grid-cols-1 lg:grid-cols-[minmax(13rem,0.8fr)_minmax(0,1.7fr)] gap-4 flex-1 min-h-0">
      <article class="bg-sidebar rounded-md border border-sidebar-border flex flex-col min-h-0 overflow-hidden">
        <div class="flex items-center justify-between px-3 py-1.5 border-b border-sidebar-border">
          <h3 class="text-sm font-medium">Plugins</h3>
          <Badge variant="secondary">{{ plugins.length }}</Badge>
        </div>
        <ScrollAreaRoot class="flex-1 min-h-0">
          <ScrollAreaViewport class="h-full p-2">
            <div v-if="!plugins.length" class="p-4 text-sm text-muted-foreground text-center">
              No plugins installed.
            </div>
            <div v-else class="space-y-1">
              <button
                  v-for="plugin in plugins"
                  :key="plugin.id || plugin.name"
                  type="button"
                  class="w-full text-left rounded-sm px-2 py-1.5 transition-colors hover:bg-sidebar-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                  :class="{ 'bg-sidebar-accent': selectedPlugin?.id === plugin.id }"
                  :aria-current="selectedPlugin?.id === plugin.id ? 'true' : undefined"
                  @click="selectedPlugin = plugin"
              >
                <span class="flex items-baseline gap-2 min-w-0 leading-4">
                  <span class="truncate text-xs font-medium leading-4">{{ plugin.name }}</span>
                  <span class="shrink-0 text-[11px] leading-4 text-muted-foreground">v{{ plugin.version }}</span>
                </span>
              </button>
            </div>
          </ScrollAreaViewport>
        </ScrollAreaRoot>
      </article>

      <article class="bg-sidebar rounded-md border border-sidebar-border flex flex-col min-h-0 overflow-hidden">
        <div v-if="!selectedPlugin" class="flex flex-1 flex-col items-center justify-center p-8 text-center">
          <Icon icon="heroicons:cube-transparent" width="32" height="32" class="text-muted-foreground/60 mb-3"/>
          <h3 class="text-sm font-medium">Select a plugin</h3>
          <p class="text-sm text-muted-foreground mt-1">Choose a plugin to view its capabilities.</p>
        </div>
        <ScrollAreaRoot v-else class="flex-1 min-h-0">
          <ScrollAreaViewport class="h-full">
            <div class="p-4 space-y-4">
              <div class="flex items-start justify-between gap-4">
                <div class="min-w-0">
                  <h3 class="text-xl font-semibold break-words">{{ selectedPlugin.name }}</h3>
                  <p class="text-sm text-muted-foreground mt-1 break-all">
                    {{ selectedPlugin.id || 'Plugin identifier unavailable' }}
                  </p>
                </div>
                <Badge variant="outline" class="shrink-0">v{{ selectedPlugin.version }}</Badge>
              </div>

              <div class="grid grid-cols-1 gap-4 text-sm">
                <div>
                  <h4 class="font-medium mb-1">Panels</h4>
                  <ul v-if="selectedPlugin.panels?.length" class="text-muted-foreground space-y-0.5">
                    <li v-for="panel in selectedPlugin.panels" :key="panel.id" class="break-all">{{ panel.id }}</li>
                  </ul>
                  <p v-else class="text-muted-foreground">None declared</p>
                </div>
                <div>
                  <h4 class="font-medium mb-1">Supported drivers</h4>
                  <ul v-if="selectedPlugin.drivers?.length" class="space-y-1">
                    <li
                        v-for="driver in selectedPlugin.drivers"
                        :key="driver"
                        class="border-l-2 border-muted-foreground/30 pl-2 text-xs leading-4 text-muted-foreground break-all select-text"
                    >
                      {{ driver }}
                    </li>
                  </ul>
                  <p v-else class="text-muted-foreground">None declared</p>
                </div>
              </div>

              <div class="border-t border-sidebar-border pt-4 text-sm">
                <h4 class="font-medium mb-1">Description</h4>
                <p class="text-muted-foreground leading-relaxed whitespace-pre-line break-words">
                  {{ selectedPlugin.description || 'No description provided.' }}
                </p>
              </div>

              <dl class="border-t border-sidebar-border pt-4 text-sm space-y-1.5">
                <div class="flex justify-between gap-4">
                  <dt class="text-muted-foreground">Author</dt>
                  <dd class="text-right break-all">{{ selectedPlugin.author || 'Unknown' }}</dd>
                </div>
              </dl>
            </div>
          </ScrollAreaViewport>
          <ScrollAreaScrollbar orientation="vertical" class="flex w-1 bg-muted-background">
            <ScrollAreaThumb class="flex-1 bg-muted-foreground rounded-md"/>
          </ScrollAreaScrollbar>
        </ScrollAreaRoot>
      </article>
    </section>
  </div>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {LabeqPluginManifest} from "@/lib/plugin.ts";
import {onMounted, ref, watch} from "vue";
import {Badge} from "@/components/ui/badge";
import {Button} from "@/components/ui/button";
import {Icon} from "@iconify/vue";
import {ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport} from "reka-ui";
import {toast} from "vue-sonner";
import {useVisualModelsStore} from "@/store/visual-modules.store.ts";

let plugins = ref<LabeqPluginManifest[]>([]);
let selectedPlugin = ref<LabeqPluginManifest | null>(null);
const isReloading = ref(false);
const visualModels = useVisualModelsStore();

onMounted(() => {
  reloadList(false).catch(error => {
    console.error('Failed to load plugins:', error);
    toast.error('Failed to load plugins.');
  });
});

watch(plugins, () => {
  selectedPlugin.value = null;
});

const reloadList = async (notify = true) => {
  const manifests = await invoke<string[]>('list_plugins');
  plugins.value = manifests.map(d => JSON.parse(d));
  visualModels.clearModels();
  visualModels.loadPluginPanels(plugins.value);
  if (notify) toast.success(`Loaded ${plugins.value.length} plugin${plugins.value.length === 1 ? '' : 's'}.`);
};

const reloadPlugins = async () => {
  if (isReloading.value) return;
  isReloading.value = true;
  const toastId = toast.loading('Reloading plugins...');
  try {
    await invoke('init_plugins');
    await reloadList(false);
    toast.success(`Loaded ${plugins.value.length} plugin${plugins.value.length === 1 ? '' : 's'}.`, {id: toastId});
  } catch (error) {
    console.error('Failed to reload plugins:', error);
    toast.error('Failed to reload plugins.', {id: toastId});
  } finally {
    isReloading.value = false;
  }
};
</script>

<style lang="scss" scoped>
/* Optional: ensure scrollbars don’t overflow */
section, article {
  min-height: 0;
}
</style>
