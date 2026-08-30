<template>
  <canvas ref="qrcode"></canvas>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import QRCode from "qrcode";

const qrcode = useTemplateRef("qrcode");

onMounted(async () => {
  if (!qrcode.value) return;

  const [serverUrl, error] = await tryCatch<string>(invoke("get_server_url"));
  if (error) {
    return console.error(error);
  }

  QRCode.toCanvas(qrcode.value, serverUrl, { color: { light: "#56b6ff", dark: "#294554" }, margin: 1, width: 300 });
});
</script>

<style scoped></style>
