<template>
  <div class="max-w-xl w-full pt-2">
    <h2 class="heading-2">Appearance</h2>

    <div class="flex items-center justify-between border-b border-border py-4">
      <div>
        <p class="text-sm font-medium">Color palette</p>
        <p class="text-xs text-muted-foreground mt-1">
          Choose the application color palette.
        </p>
      </div>

      <Select
          :model-value="colorPalette"
          @update:model-value="value => selectColorPalette(value as ColorPalette)"
      >
        <SelectTrigger class="w-32">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem
              v-for="palette in colorPalettes"
              :key="palette.value"
              :value="palette.value"
          >
            {{ palette.label }}
          </SelectItem>
        </SelectContent>
      </Select>
    </div>

    <div class="flex items-center justify-between border-b border-border py-4">
      <div>
        <p class="text-sm font-medium">Appearance</p>
        <p class="text-xs text-muted-foreground mt-1">
          Choose the light or dark variant.
        </p>
      </div>

      <Select
          :model-value="appearanceMode"
          @update:model-value="value => selectAppearanceMode(value as AppearanceMode)"
      >
        <SelectTrigger class="w-32">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="light">Light</SelectItem>
          <SelectItem value="dark">Dark</SelectItem>
        </SelectContent>
      </Select>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref} from "vue";
import {
  applyColorPalette,
  applyAppearanceMode,
  colorPalettes,
  getStoredAppearanceMode,
  initColorPalette,
  watchActiveTheme,
  type ColorPalette,
  type AppearanceMode,
} from "@/lib/theme";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

const colorPalette = ref<ColorPalette>("default");
const appearanceMode = ref<AppearanceMode>("light");
let stopThemeWatcher: (() => void) | undefined;

onMounted(() => {
  colorPalette.value = initColorPalette();
  appearanceMode.value = getStoredAppearanceMode();
  stopThemeWatcher = watchActiveTheme((palette, mode) => {
    colorPalette.value = palette;
    appearanceMode.value = mode;
  });
});

onBeforeUnmount(() => stopThemeWatcher?.());

const selectColorPalette = (palette: ColorPalette) => {
  colorPalette.value = palette;
  applyColorPalette(palette);
};

const selectAppearanceMode = (mode: AppearanceMode) => {
  appearanceMode.value = mode;
  applyAppearanceMode(mode);
};
</script>