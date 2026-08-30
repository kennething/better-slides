<template>
  <div class="flex flex-col h-screen w-screen items-center justify-center gap-8">
    <div class="flex items-center justify-center flex-col gap-2">
      <h1 class="text-3xl font-semibold mb-2">How to use:</h1>
      <ol class="list-decimal pl-5 *:text-lg">
        <li>Scan the QR code with a mobile device</li>
        <li>Minimize this app and return to your slideshow</li>
        <li>Profit :3</li>
      </ol>
    </div>

    <div class="flex items-center justify-center flex-col gap-1">
      <p class="text-lg font-semibold">Status</p>
      <div
        class="px-10 select-none py-4 flex items-center justify-center rounded-xl text-xl"
        :class="{
          'bg-success': connectionState === ConnectionState.Connected,
          'bg-error': connectionState === ConnectionState.Disconnected,
          'bg-base-300 border-base-200 border-2': connectionState === ConnectionState.Init
        }"
      >
        {{ connectionState === ConnectionState.Connected ? "Connected!" : connectionState === ConnectionState.Disconnected ? "Disconnected" : "Waiting..." }}
      </div>
    </div>

    <div class="fixed bottom-0 right-0 p-10 group">
      <div
        class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full h-full flex items-center justify-center transition-opacity duration-300"
        :class="connectionState === ConnectionState.Init ? 'opacity-0' : 'backdrop-blur-xl group-hover:opacity-0'"
      >
        <p class="text-xl font-medium text-white">Hover to reveal</p>
      </div>

      <QRCode />
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
