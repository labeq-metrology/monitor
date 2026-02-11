<script setup lang="ts">

import {
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle
} from "@/components/ui/alert-dialog";
import {useConnectionStore} from "@/store/connection.store.ts";
import {computed, ref} from "vue";
import {Select, SelectContent, SelectItem, SelectLabel, SelectTrigger, SelectValue} from "@/components/ui/select";
import {useWorkspaceSore} from "@/store/workspace.store.ts";
import {Item, LabeqConnection} from "@/lib/labeq.ts";
import {Icon} from "@iconify/vue";

const props = defineProps<{ workspaceId: string }>();

const workspaceStore = useWorkspaceSore();
const workspace = computed(() => workspaceStore.workspaces.get(props.workspaceId));
const connectionStore = useConnectionStore();
const selectedConnection = ref<string>("");

const allConnections = computed(() => {
  const flatten = (items: Item<LabeqConnection>[]): LabeqConnection[] => {
    return items.flatMap(item =>
        item.type === 'item' ? [item.value] : flatten(item.value.items)
    );
  };
  console.log(workspace.value);
  if (!workspace.value) return [];
  else return flatten(workspace.value.connections.items)
});

const availableConnections = computed(() => {
  const ids = new Set(allConnections.value.map(c => c.id));
  return [...connectionStore.connections.values()].filter(conn => !ids.has(conn.id));
});

defineEmits(['confirm']);
</script>

<template>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>Add Connection</AlertDialogTitle>
    </AlertDialogHeader>
    <AlertDialogDescription>
      <Select v-model="selectedConnection">
        <SelectLabel>Connection</SelectLabel>
        <SelectTrigger class="max-h-6 px-2 py-3 w-full">
          <SelectValue placeholder="Select connection"/>
        </SelectTrigger>
        <SelectContent>
          <SelectItem class="max-h-6 h-6" v-for="conn in availableConnections" :key="conn.id" :value="conn.id">
            <Icon
                :icon="conn.state == 'Connected' ? 'heroicons:signal-16-solid' : 'heroicons:signal-slash-16-solid'"
                :class="conn.state == 'Connected' ? 'text-positive text-lg' : 'text-muted-foreground text-lg'"
            />
            {{ conn.name }}
          </SelectItem>
        </SelectContent>
      </Select>
    </AlertDialogDescription>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction @click="$emit('confirm', selectedConnection)">Add</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</template>

<style scoped lang="scss">

</style>