<template>
  <div class="bg-background rounded-md py-1 px-2">
    <CollapsibleTrigger
        class="group/workspace transition-colors duration-75 cursor-pointer w-full px-2 py-1 rounded-md align-text-top">
      <div class="flex items-end space-x-2 w-full">
        <Icon
            icon="heroicons:chevron-down"
            class="text-lg self-end transition-transform duration-300 text-accent-foreground -ml-2"
            width="16"
            height="16"
        />
        <Icon
            icon="heroicons:folder"
            class="text-muted-foreground text-lg"
        />
        <input
            ref="nameInput"
            v-model="workspaceName"
            @blur="handleBlur"
            @keyup.enter="confirmCreate"
            @keyup.esc="cancelCreate"
            @click.stop
            placeholder="Workspace name..."
            class="
            self-end text-xs font-normal tracking-wide
            text-left max-w-8/12
            bg-background border border-border rounded
            focus:outline-none focus:ring-1 focus:ring-ring
            text-foreground
            placeholder:text-muted-foreground
          "
        />
      </div>
    </CollapsibleTrigger>
  </div>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {CollapsibleTrigger} from "reka-ui";
import {onMounted, ref} from "vue";

const emit = defineEmits<{
  create: [name: string]
  cancel: []
}>();

const workspaceName = ref("");
const nameInput = ref<HTMLInputElement | null>(null);

onMounted(() => {
  nameInput.value?.focus();
});

const confirmCreate = () => {
  if (workspaceName.value.trim()) {
    emit('create', workspaceName.value.trim());
  } else {
    emit('cancel');
  }
};

const handleBlur = () => {
  // Small delay to allow click events to process
  setTimeout(() => {
    if (workspaceName.value.trim()) {
      emit('create', workspaceName.value.trim());
    } else {
      emit('cancel');
    }
  }, 150);
};

const cancelCreate = () => {
  emit('cancel');
};
</script>