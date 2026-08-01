<script setup lang="ts">
import {useConnectionStore} from "@/store/connection.store.ts";
import {useRoute, useRouter} from "vue-router";
import {Button} from "@/components/ui/button";
import {Icon} from "@iconify/vue";
import {Separator} from "@/components/ui/separator";
import {computed, reactive, ref} from "vue";

const route = useRoute();
const router = useRouter();

if (!route.params.connectionId) {
  router.push("/");
}

const connectionStore = useConnectionStore();
const connection = computed(() => connectionStore.connections.get(route.params.connectionId as string));

if (!connection) {
  router.push("/");
}

</script>

<template>
  <div>
    <header>
      <h2 class="text-xl">{{ connection?.name }}</h2>
      <p class="text-muted-foreground" v-if="connection?.state === 'Disconnected'">{{ connection?.state }}</p>
      <p class="text-positive" v-if="connection?.state === 'Connected'">{{ connection?.state }}</p>
      <p class="text-attention" v-if="connection?.state === 'Connecting'">{{ connection?.state }}</p>
      <p class="text-destructive" v-if="connection?.state === 'ConnectionError'">Connection error</p>
<!--      <p class="text-xs text-muted-foreground">Loaded driver: {{ device?.driver }}</p>-->
      <ul class="mt-2 flex gap-2 w-full">

      </ul>
    </header>
    <Separator class="mt-2 mb-2"/>
  </div>
</template>

<style scoped lang="scss">

</style>