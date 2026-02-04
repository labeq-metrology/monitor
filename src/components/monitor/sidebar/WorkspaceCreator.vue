<template>
  <CollapsibleTrigger class="group/workspace transition-colors duration-75 cursor-pointer w-full px-2 py-1 rounded-md">
    <div class="flex items-center space-x-2 w-full">
      <Icon
          icon="heroicons:chevron-down"
          class="text-lg self-end transition-transform duration-300 -ml-2 text-accent-foreground opacity-0"
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
            text-xs font-normal tracking-wide
            text-left max-w-8/12
            bg-background border border-border rounded px-1 py-0.5
            focus:outline-none focus:ring-1 focus:ring-ring
            text-foreground
            placeholder:text-muted-foreground
          "
      />
    </div>
  </CollapsibleTrigger>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {CollapsibleTrigger} from "reka-ui";
import {ref, onMounted} from "vue";

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