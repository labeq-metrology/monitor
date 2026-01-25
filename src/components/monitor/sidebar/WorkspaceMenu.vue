<template>
  <Collapsible class="group/workspaces-section-label">
    <CollapsibleTrigger class="w-full" as-child>
      <SidebarGroupLabel class="flex items-center gap-2">
        <div class="flex items-center gap-2 cursor-pointer">
          <Icon
              icon="heroicons:chevron-down"
              class="transition-transform duration-300 group-data-[state=closed]/workspaces-section-label:rotate-180 -ml-2 text-accent-foreground self-end"
              width="16"
              height="16"
          />
          <p>Workspaces</p>
        </div>
        <div
            class="group-hover/workspaces-section-label:opacity-100 group-hover/workspaces-section-label:visible opacity-0 invisible transition-all duration-200 ml-auto">
          <RouterLink to="/new-connection">
            <Button variant="link" class="w-4 h-4 m-0 p-0 flex items-center justify-center text-accent-foreground/50">
              <Icon icon="heroicons:plus" width="16" height="16"/>
            </Button>
          </RouterLink>
        </div>
      </SidebarGroupLabel>
    </CollapsibleTrigger>
    <CollapsibleContent>
      <SidebarMenu>
        <div class="text-muted-foreground text-xs mx-2 px-2 py-2 rounded-md bg-background">
          <p>No workspaces present...</p>
        </div>
      </SidebarMenu>
    </CollapsibleContent>
  </Collapsible>
</template>

<script setup lang="ts">
import {Icon} from "@iconify/vue";
import {Button} from "@/components/ui/button";
import {Collapsible, CollapsibleTrigger, CollapsibleContent} from "@/components/ui/collapsible";
import {SidebarGroupLabel, SidebarMenu} from "@/components/ui/sidebar";
</script>

<style lang="scss" scoped>

</style>