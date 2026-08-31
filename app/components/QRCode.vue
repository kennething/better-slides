<template>
  <div class="relative rounded-xl overflow-hidden">
    <div v-if="errorMessage" class="size-75 absolute bottom-0 right-0 bg-red-800 flex items-center justify-center p-10 text-center text-lg text-white">{{ errorMessage }}</div>
    <canvas ref="qrcode"></canvas>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import QRCode from "qrcode";

const qrcode = useTemplateRef("qrcode");

const errorMessage = ref("");
async function generateQRCode() {
  if (!qrcode.value) return;

  const [serverUrl, error] = await tryCatch<string>(invoke("get_server_url"));
  if (error) {
    errorMessage.value = error.message ?? "couldnt generate QR code. womp womp";
    return console.error(error);
  }

  const width = Math.min(window.innerWidth, window.innerHeight) * 0.35;
  QRCode.toCanvas(qrcode.value, serverUrl, { color: { light: "#f5fcff", dark: "#294554" }, margin: 1, width });
}

onMounted(() => {
  generateQRCode();
  window.addEventListener("resize", () => generateQRCode());
});
</script>

<style scoped></style>
