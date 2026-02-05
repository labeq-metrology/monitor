<script setup lang="ts">

import {
  AlertDialogTitle,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogDescription, AlertDialogFooter, AlertDialogCancel
} from "@/components/ui/alert-dialog";
import {Input} from "@/components/ui/input";

const name = defineModel<string>('name');

defineEmits(['confirm']);
</script>

<template>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>Add Folder</AlertDialogTitle>
    </AlertDialogHeader>
    <AlertDialogDescription>
      <Input placeholder="Folder name" v-model:modelValue="name" />
    </AlertDialogDescription>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction @click="$emit('confirm')">Add</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</template>

<style scoped lang="scss">

</style>