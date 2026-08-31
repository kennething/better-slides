<template>
  <div class="flex flex-col h-screen w-screen items-center justify-center gap-8">
    <img src="/logo.svg" draggable="false" class="size-12 select-none fixed top-4 left-4" alt="" />

    <div class="flex items-center justify-center flex-col gap-1">
      <p class="text-2xl font-semibold">Status</p>
      <div
        class="px-10 select-none py-4 flex items-center justify-center rounded-xl text-xl"
        :class="{
          'bg-success': connectionState === ConnectionState.Connected,
          'bg-error': connectionState === ConnectionState.Disconnected,
          'bg-base-300 border-base-200 border-2': connectionState === ConnectionState.Init
        }"
      >
        {{ connectionState === ConnectionState.Connected ? "Connected!" : connectionState === ConnectionState.Disconnected ? "Disconnected" : "Idle..." }}
      </div>
    </div>

    <div class="group">
      <div
        class="absolute top-0 left-0 w-full h-full flex items-center justify-center transition-opacity duration-300"
        :class="connectionState === ConnectionState.Init ? 'opacity-0' : 'backdrop-blur-xl group-hover:opacity-0'"
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
</script>

<style scoped></style>
