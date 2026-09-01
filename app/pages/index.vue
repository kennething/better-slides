<template>
  <div class="flex flex-col h-screen w-screen items-center justify-center gap-8">
    <img src="/logo.svg" draggable="false" class="size-12 select-none fixed top-4 left-4" alt="" />

    <div class="flex items-center justify-center flex-col gap-1">
      <p class="text-2xl font-semibold">Status</p>
      <div
        class="px-10 select-none py-4 flex items-center justify-center rounded-xl text-xl"
        :class="{
          'bg-success/75': connectionState === ConnectionState.Connected,
          'bg-error/75': connectionState === ConnectionState.Disconnected,
          'bg-base-300 border-base-200 dark:bg-neutral-600 dark:border-neutral-700 border-2': connectionState === ConnectionState.Init
        }"
      >
        {{ connectionState === ConnectionState.Connected ? "Connected!" : connectionState === ConnectionState.Disconnected ? "Disconnected" : "Idle..." }}
      </div>
    </div>

    <div class="group relative rounded-xl overflow-hidden">
      <div
        class="absolute top-0 left-0 z-1 w-full h-full flex items-center justify-center transition-opacity duration-300"
        :class="connectionState === ConnectionState.Init ? 'opacity-0' : 'backdrop-blur-md group-hover:opacity-0'"
      >
        <p class="text-xl font-medium text-white">Hover to reveal</p>
      </div>

      <QRCode />
    </div>

    <div class="flex items-center justify-center flex-col gap-2">
      <h1 class="text-2xl font-semibold">How to use:</h1>
      <ol class="list-decimal pl-5 *:text-lg">
        <li>Scan the QR code with a mobile device</li>
        <li>Keep this app in the background</li>
        <li>Return to your presentation</li>
        <li>profit :3</li>
      </ol>
    </div>
  </div>
</template>

<script setup lang="ts">
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { check } from "@tauri-apps/plugin-updater";
import { listen } from "@tauri-apps/api/event";

enum ConnectionState {
  Init,
  Connected,
  Disconnected
}
const connectionState = ref<ConnectionState>(ConnectionState.Init);

onMounted(() => {
  listen("connected", () => (connectionState.value = ConnectionState.Connected));
  listen("disconnected", () => (connectionState.value = ConnectionState.Disconnected));
});

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
