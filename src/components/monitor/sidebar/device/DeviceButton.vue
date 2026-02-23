<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {SidebarMenuSubButton} from "@/components/ui/sidebar";
import {useRouter} from "vue-router";
import {LabeqConnection, LabeqDevice} from "@/lib/labeq.ts";

defineProps<{
  device: LabeqDevice,
  connection: LabeqConnection
}>();

const router = useRouter();
</script>

<template>
  <SidebarMenuSubButton class="select-none cursor-pointer"
                        @click.prevent="router.push({
                              name: 'device',
                              params: { connectionId: connection.id, id: device.uuid }
                            })"
  >
    <Icon icon="heroicons:cpu-chip-20-solid"/>
    <span class="text-xs">{{ device.name }}</span>
  </SidebarMenuSubButton>
</template>

<style scoped lang="scss">

</style>