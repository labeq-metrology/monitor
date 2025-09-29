<template>
  <div class="w-full h-full flex flex-col min-h-0">
    <header>
      <h2 class="text-xl">Installed plugins</h2>
    </header>

    <section class="grid grid-cols-5 gap-4 flex-1 h-full min-h-0 my-4">

      <article class="bg-sidebar rounded-sm p-1 border border-sidebar-border col-span-2 h-full overflow-y-auto min-h-0">
        <ul>
          <li v-for="plugin in plugins" :key="plugin.name" class="cursor-pointer">
            <SidebarMenuSubButton
                @click="selectedPlugin = plugin"
                :class="{ 'bg-sidebar-accent': selectedPlugin?.name === plugin.name }"
            >
              <span class="whitespace-nowrap overflow-hidden overflow-ellipsis">{{ plugin.name }}</span>
              <Badge variant="outline" class="whitespace-nowrap overflow-hidden overflow-ellipsis ml-auto">
                {{ plugin.version }}
              </Badge>
            </SidebarMenuSubButton>
          </li>
        </ul>
      </article>

      <div class="col-span-3 flex flex-col min-h-0">
        <article class="bg-sidebar rounded-sm p-1 border border-sidebar-border h-3/5 flex flex-col min-h-0">
          <div v-if="!selectedPlugin" class="flex flex-col justify-center h-full">
            <p class="text-muted-foreground mx-auto">Select a plugin to view details.</p>
          </div>

          <ScrollAreaRoot v-else class="flex-1 flex flex-col min-h-0">
            <ScrollAreaViewport class="flex-1 min-h-0 m-1">
              <div class="h-full flex flex-col">
                <h3 class="font-semibold">{{ selectedPlugin.name }}</h3>
                <div class="text-xs text-muted-foreground mt-1">
                  <p>Version: {{ selectedPlugin.version }}</p>
                  <p>Author: {{ selectedPlugin.author }}</p>
                  <div>
                    <p>Supported drivers:</p>
                    <ul class="list-disc list-inside">
                      <li v-for="driver in selectedPlugin.drivers" :key="driver">{{ driver }}</li>
                    </ul>
                  </div>
                </div>
                <article class="text-sm mt-4 p-3 rounded-md border border-sidebar-border bg-muted">
                  <h4 class="font-medium mb-2 text-foreground">Description</h4>
                  <p class="text-muted-foreground leading-relaxed whitespace-pre-line">
                    {{ selectedPlugin.description }}
                  </p>
                </article>
              </div>
            </ScrollAreaViewport>
            <ScrollAreaScrollbar
                orientation="vertical"
                class="flex w-1 bg-muted-background rounded-md transition-opacity"
            >
              <ScrollAreaThumb class="flex-1 bg-muted-foreground rounded-md hover:bg-foreground transition-opacity"/>
            </ScrollAreaScrollbar>
          </ScrollAreaRoot>
        </article>

        <Button :disabled="!selectedPlugin" class="mt-4 self-start">
          <Icon icon="heroicons:arrow-top-right-on-square" width="24" height="24"/>
          <p>More details</p>
        </Button>
      </div>

    </section>
  </div>
</template>

<script async setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {LabeqPluginManifest} from "@/lib/plugin.ts";
import {onMounted, ref} from "vue";
import {SidebarMenuSubButton} from "@/components/ui/sidebar";
import {Badge} from "@/components/ui/badge";
import {Button} from "@/components/ui/button";
import {Icon} from "@iconify/vue";
import {ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport} from "reka-ui";

let plugins = ref<LabeqPluginManifest[]>([]);
let selectedPlugin = ref<LabeqPluginManifest | null>(null);

onMounted(async () => {
  plugins.value = (await invoke<string[]>('list_plugins')).map(d => JSON.parse(d));
});
</script>

<style lang="scss" scoped>
/* Optional: ensure scrollbars don’t overflow */
section, article {
  min-height: 0;
}
</style>
