<script setup lang="ts">
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import {buttonVariants} from "@/components/ui/button";

defineProps<{
  open: boolean
  title: string
  description: string
  details?: string[]
  confirmLabel?: string
  destructive?: boolean
}>();

const emit = defineEmits<{
  "update:open": [open: boolean]
  confirm: []
  cancel: []
}>();

const confirm = () => emit("confirm");
const cancel = () => emit("cancel");
</script>

<template>
  <AlertDialog :open="open" @update:open="emit('update:open', $event)">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>{{ title }}</AlertDialogTitle>
        <AlertDialogDescription>{{ description }}</AlertDialogDescription>
        <div
            v-if="details?.length"
            class="mt-3 rounded-md border border-border bg-muted/40 p-3"
        >
          <p class="mb-2 text-xs font-medium text-foreground">Missing items</p>
          <ul class="space-y-1 text-xs text-muted-foreground">
            <li v-for="detail in details" :key="detail">{{ detail }}</li>
          </ul>
        </div>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel @click="cancel">Cancel</AlertDialogCancel>
        <AlertDialogAction
            :class="buttonVariants({variant: destructive ? 'destructive' : 'default'})"
            @click="confirm"
        >
          {{ confirmLabel ?? "Confirm" }}
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</template>
