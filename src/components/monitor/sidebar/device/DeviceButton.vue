<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {SidebarMenuSubButton} from "@/components/ui/sidebar";
import {useRouter} from "vue-router";
import {LabeqConnection, LabeqDevice} from "@/lib/labeq.ts";
import {useDragSource} from "@/composables/useDrag.ts";

defineProps<{
  device: LabeqDevice,
  connection: LabeqConnection
}>();

const router = useRouter();
const { startDrag } = useDragSource();
</script>

<template>
  <div
      class="select-none touch-none"
      @pointerdown="startDrag('device', device.uuid, device.name, $event)"
      @click="router.push({ name: 'device', params: { connectionId: connection.id, id: device.uuid } })"
  >
    <SidebarMenuSubButton class="cursor-grab hover:bg-accent/50 transition-colors">
      <Icon icon="heroicons:cpu-chip-20-solid"/>
      <span class="text-xs">{{ device.name }}</span>
    </SidebarMenuSubButton>
  </div>
</template>