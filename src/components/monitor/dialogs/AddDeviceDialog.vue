<script setup lang="ts">

import {
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogAction
} from "@/components/ui/alert-dialog";
import {useConnectionStore} from "@/store/connection.store.ts";
import {computed} from "vue";
import {Select, SelectTrigger, SelectContent, SelectItem, SelectValue, SelectLabel} from "@/components/ui/select";
import {useWorkspaceSore} from "@/store/workspace.store.ts";
import {Item, LabeqConnection} from "@/lib/labeq.ts";
import {Icon} from "@iconify/vue";

const props = defineProps<{ workspaceId: string }>();

const workspaceStore = useWorkspaceSore();
const workspace = computed(() => workspaceStore.workspaces.get(props.workspaceId));

const connectionStore = useConnectionStore();
const connection = computed(() => connectionStore.connections.get(selectedConnection.value ?? ""));

const selectedConnection = defineModel<string>('connection');
const device = defineModel<string>('device');

const allConnections = computed(() => {
  const flatten = (items: Item<LabeqConnection>[]): LabeqConnection[] => {
    return items.flatMap(item =>
        item.type === 'item' ? [item.value] : flatten(item.value.items)
    );
  };
  if (!workspace.value) return [];
  else return flatten(workspace.value.connections.items);
});

defineEmits(['confirm']);
</script>

<template>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>Add Device</AlertDialogTitle>
    </AlertDialogHeader>
    <AlertDialogDescription>
      <Select v-model="selectedConnection">
        <SelectLabel>Connection</SelectLabel>
        <SelectTrigger class="max-h-6 px-2 py-3 w-full">
          <SelectValue placeholder="Select connection"/>
        </SelectTrigger>
        <SelectContent>
          <SelectItem class="max-h-6 h-6" v-for="conn in allConnections" :key="conn.id" :value="conn.id">
            <Icon
                :icon="conn.state == 'Connected' ? 'heroicons:signal-16-solid' : 'heroicons:signal-slash-16-solid'"
                :class="conn.state == 'Connected' ? 'text-positive text-lg' : 'text-muted-foreground text-lg'"
            />
            {{ conn.name }}
          </SelectItem>
        </SelectContent>
      </Select>
      <Select v-model="device">
        <SelectLabel>Device</SelectLabel>
        <SelectTrigger class="max-h-6 px-2 py-3 w-full">
          <SelectValue placeholder="Select device"/>
        </SelectTrigger>
        <SelectContent>
          <SelectItem class="max-h-6 h-6" v-if="connection" v-for="device in connection.devices" :key="device.uuid" :value="device.uuid">
            {{ device.name ?? device.uuid }}
          </SelectItem>
        </SelectContent>
      </Select>
    </AlertDialogDescription>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction @click="$emit('confirm', selectedConnection, device)">Add</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</template>

<style scoped lang="scss">

</style>