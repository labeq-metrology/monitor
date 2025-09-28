<template>
  <div>
    {{ route.name }} {{ route.fullPath }}
  </div>
</template>

<script setup lang="ts">
import {useRoute} from "vue-router";

const route = useRoute()
</script>

<style lang="scss" scoped>

</style>