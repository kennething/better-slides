<template>
  <NuxtPage />
</template>

<script setup lang="ts">
onMounted(() => {
  document.addEventListener("contextmenu", (event) => event.preventDefault());
});
</script>

<style scoped></style>
