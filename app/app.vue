<template>
  <NuxtPage />
</template>

<script setup lang="ts">
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { check } from "@tauri-apps/plugin-updater";

onMounted(async () => {
  try {
    const update = await check();
    if (!update) return;

    const existing = await WebviewWindow.getByLabel("update");
    if (existing) return;

    new WebviewWindow("update", {
      url: "/update",
      title: `Update available: v${update.version}`,
      width: 600,
      height: 450,
      resizable: false,
      center: true
    });
  } catch (error) {
    console.error("Update check failed:", error);
  }
});
</script>

<style scoped></style>
