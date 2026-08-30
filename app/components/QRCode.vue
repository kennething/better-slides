<template>
  <div class="rounded-xl overflow-hidden">
    <canvas v-if="!errorMessage" ref="qrcode"></canvas>
    <div v-else class="size-75 bg-red-800 flex items-center justify-center p-10 text-center text-lg text-white">{{ errorMessage }}</div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import QRCode from "qrcode";

const qrcode = useTemplateRef("qrcode");

const errorMessage = ref("");
onMounted(async () => {
  if (!qrcode.value) return;

  const [serverUrl, error] = await tryCatch<string>(invoke("get_server_url"));
  if (error) {
    errorMessage.value = error.message ?? "couldnt generate QR code. womp womp";
    return console.error(error);
  }

  QRCode.toCanvas(qrcode.value, serverUrl, { color: { light: "#56b6ff", dark: "#294554" }, margin: 1, width: 300 });
});
</script>

<style scoped></style>
