<template>
  <div class="flex select-none flex-col items-center justify-center gap-2 px-7 py-3 rounded-xl">
    <p class="text-xl font-semibold">Update available :3</p>
    <p class="text-sm text-center text-neutral-700">{{ current }} -> {{ latest }}</p>

    <!-- <a :href="newRelease?.html_url" target="_blank" rel="noopener noreferrer" class="bg-sky-100 hover:bg-sky-300 duration-75 rounded-xl p-2 w-full text-center text-lg">Update</a> -->
  </div>
</template>

<script setup lang="ts">
const route = useRoute();

const current = computed(() => route.query.current);
const latest = computed(() => route.query.latest);

onMounted(() => {
  document.documentElement.classList.add("bg-sky-200!", "dark:bg-sky-900!");
});
</script>

<style scoped></style>
