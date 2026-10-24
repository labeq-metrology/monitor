<template>
  <div class="w-full flex bg-muted/30 border-b border-border overflow-x-auto no-scrollbar h-9">
    <TabBarItem
        v-for="(tab, index) in tabsStore.tabs"
        :key="tab.id"
        :tab="tab"
        :index="index"
        :isActive="tabsStore.activeTabId === tab.id"
        @click="router.push(tab.path)"
        @close="tabsStore.removeTab(tab.id, router)"
        @drop="handleDrop"
    />

    <!-- Trailing drop zone -->
    <div
        ref="trailingZone"
        class="flex-1 min-w-[20px] h-full transition-colors"
        :class="{ 'bg-muted/50': isTrailingOver }"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { useTabsStore } from '@/store/tabs.store.ts';
import { useConnectionStore } from '@/store/connection.store.ts';
import { useRouter, useRoute } from 'vue-router';
import { watch, ref, onMounted } from 'vue';
import { useDragTarget } from '@/composables/useDrag';
import TabBarItem from './TabBarItem.vue';

const tabsStore = useTabsStore();
const connectionStore = useConnectionStore();
const router = useRouter();
const route = useRoute();

const handleDrop = ({ draggedId, targetIndex }: { draggedId: string, targetIndex: number }) => {
  const fromIndex = tabsStore.tabs.findIndex(t => t.id === draggedId);
  if (fromIndex !== -1 && fromIndex !== targetIndex) {
    // If moving left-to-right, adjust the target index because the array length shrinks when the item is spliced out
    const adjustedTarget = targetIndex > fromIndex ? targetIndex - 1 : targetIndex;
    tabsStore.reorderTabs(fromIndex, adjustedTarget);
  }
};

// Register the empty trailing space to drop items at the very end
const trailingZone = ref<HTMLElement | null>(null);
const { isOver: isTrailingOver, register: registerTrailing } = useDragTarget(
    () => trailingZone.value,
    'tab',
    (draggedId) => {
      handleDrop({ draggedId, targetIndex: tabsStore.tabs.length });
    },
    -1 // Lower priority so it doesn't accidentally override the last tab's hit area
);

onMounted(() => {
  registerTrailing();
});

watch(() => route.fullPath, () => {
  let title = "Tab";

  if (route.name === 'device' || route.name === 'device-terminal') {
    const conn = connectionStore.connections.get(route.params.connectionId as string);
    const dev = conn?.devices.find(d => d.uuid === route.params.id as string);
    title = dev ? dev.name : "Device";
    if (route.name === 'device-terminal') title += " (Terminal)";
  } else if (route.name === 'connection') {
    const conn = connectionStore.connections.get(route.params.connectionId as string);
    title = conn ? conn.name : "Connection";
  } else if (route.path === '/') {
    title = "Welcome";
  } else if (route.path === '/new-connection') {
    title = "New Connection";
  }

  tabsStore.addTab({
    id: route.fullPath,
    title,
    path: route.fullPath
  });
}, { immediate: true });
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
