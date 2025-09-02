<script setup lang="ts">
import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from '@/components/ui/menubar'
import {SidebarTrigger} from "@/components/ui/sidebar";
import {useColorMode} from "@vueuse/core";
import {Button} from "@/components/ui/button";
import {Icon} from "@iconify/vue";

const mode = useColorMode()
</script>

<template>
  <Menubar>
    <SidebarTrigger />
    <Button variant="ghost" @click="mode == 'light' ? mode = 'dark' : mode = 'light'">
      <Icon v-if="mode == 'dark'" icon="heroicons-outline:sun" />
      <Icon v-else icon="heroicons-outline:moon" />
    </Button>
    <MenubarMenu>
      <MenubarTrigger>File</MenubarTrigger>
      <MenubarContent>
        <MenubarItem>
          New Tab <MenubarShortcut>⌘T</MenubarShortcut>
        </MenubarItem>
        <MenubarItem>New Window</MenubarItem>
        <MenubarSeparator />
        <MenubarItem>Share</MenubarItem>
        <MenubarSeparator />
        <MenubarItem>Print</MenubarItem>
      </MenubarContent>
    </MenubarMenu>
  </Menubar>
</template>

<style lang="scss" scoped>

</style>