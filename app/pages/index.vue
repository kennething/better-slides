<template>
  <div class="flex flex-col h-screen w-screen items-center justify-center">
    <h1 class="text-3xl font-semibold mb-2">How to use:</h1>
    <ol class="list-decimal pl-5 *:text-lg">
      <li>Scan the QR code with a mobile device</li>
      <li>Minimize this app and return to your slideshow</li>
      <li>Profit</li>
    </ol>

    <div class="fixed bottom-0 right-0 p-10 overflow-hidden rounded-lg group">
      <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full h-full backdrop-blur-lg group-hover:opacity-0 select-none flex items-center justify-center">
        <p class="text-xl font-medium text-white">Hover to reveal</p>
      </div>

      <QRCode />
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped></style>
