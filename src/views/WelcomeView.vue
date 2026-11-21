<template>
  <div class="welcome-view">
    <img
        class="welcome-logotype"
        :src="mode === 'dark' ? logotypeWhite : logotypeBlack"
        alt="LabEq Metrology"
    />
    <h2 class="heading-2 labeq-brand">Welcome to LabEq Monitor</h2>
    <p class="welcome-description">
      Connect metrology devices, organize them into workspaces, and monitor measurements in one place.
    </p>

    <div class="welcome-info" aria-label="Monitor overview">
      <div>
        <p class="info-title">Connections</p>
        <p class="info-description">Add and manage connections to your metrology gateway.</p>
      </div>
      <div>
        <p class="info-title">Workspaces</p>
        <p class="info-description">Arrange devices and data views around your workflow.</p>
      </div>
      <div>
        <p class="info-title">Monitoring</p>
        <p class="info-description">View live device information and measurements.</p>
      </div>
    </div>

    <Button variant="secondary" class="h-8" @click="router.push('/new-connection')">
      Add connection
    </Button>
  </div>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref} from "vue";
import {watchActiveTheme} from "@/lib/theme";
import {useRouter} from "vue-router";
import {Button} from "@/components/ui/button";
import logotypeBlack from "@/assets/img/logotype-black.svg";
import logotypeWhite from "@/assets/img/logotype-white.svg";

const router = useRouter();
const mode = ref(document.documentElement.classList.contains("dark") ? "dark" : "light");
let observer: MutationObserver | undefined;
let stopThemeWatcher: (() => void) | undefined;

onMounted(() => {
  stopThemeWatcher = watchActiveTheme((_palette, themeMode) => {
    mode.value = themeMode;
  });
  observer = new MutationObserver(() => {
    mode.value = document.documentElement.classList.contains("dark") ? "dark" : "light";
  });
  observer.observe(document.documentElement, {attributes: true, attributeFilter: ["class"]});
});

onBeforeUnmount(() => {
  observer?.disconnect();
  stopThemeWatcher?.();
});
</script>

<style scoped>
.welcome-view {
  max-width: 36rem;
  padding-top: 2rem;
}

.welcome-logotype {
  width: 440px;
  height: auto;
  margin-bottom: 1.5rem;
}

.welcome-description {
  margin: 0 0 1rem;
  color: var(--muted-foreground);
  font-size: 0.875rem;
  line-height: 1.5;
}

.info-title {
  position: relative;
  margin: 0 0 0.35rem;
  color: var(--foreground);
  font-size: 0.875rem;
  font-weight: 600;
}

.welcome-info > div::before {
  content: "";
  display: block;
  width: 1.5rem;
  height: 2px;
  margin-bottom: 0.65rem;
  background: var(--attention);
}

.welcome-info {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 1.25rem;
  margin: 1.5rem 0;
  padding: 1.25rem 0;
  border-top: 1px solid var(--border);
  border-bottom: 1px solid var(--border);
}

.info-title {
  margin: 0 0 0.35rem;
  color: var(--foreground);
  font-size: 0.875rem;
  font-weight: 600;
}

.info-description {
  margin: 0;
  color: var(--muted-foreground);
  font-size: 0.8125rem;
  line-height: 1.45;
}

@media (max-width: 640px) {
  .welcome-view {
    padding-top: 1rem;
  }

  .welcome-info {
    grid-template-columns: 1fr;
    gap: 0.9rem;
  }
}
</style>