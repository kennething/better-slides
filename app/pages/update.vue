<template>
  <div v-if="isChecking" class="flex items-center justify-center gap-2">
    <div class="h-4 w-4 animate-spin rounded-full border-2 border-sky-500 border-t-transparent"></div>
    <p class="text-sm text-neutral-700">Checking for updates...</p>
  </div>

  <div v-else-if="errorMessage">
    <p class="text-sm text-center text-neutral-700">{{ errorMessage }}</p>
  </div>

  <div v-else-if="!update" class="flex select-none flex-col items-center justify-center gap-2 p-10">
    <h1 class="text-xl font-semibold">Up to date!</h1>
  </div>

  <div v-else class="flex flex-col items-center justify-center p-10">
    <h1 class="text-xl font-semibold">Update available :3</h1>
    <p class="text-sm text-center text-neutral-700 dark:text-neutral-300">Version {{ update.version }}</p>

    <div class="my-4 pb-10" v-if="update.body">
      <h1 class="text-lg font-semibold">What's new:</h1>
      {{ update.body }}
    </div>

    <div class="fixed select-none bottom-10 left-1/2 -translate-x-1/2">
      <div v-if="!isInstalling" class="du-aura du-aura-sm du-aura-rainbow">
        <button class="bg-base-200 hover:bg-base-300 px-6 py-2 rounded-xl" @click="installUpdate">Install v{{ update.version }}</button>
      </div>

      <div v-else class="flex items-center justify-center gap-2 mt-2">
        <p class="shrink-0 font-light">{{ progress }}%</p>
        <div class="w-60 h-4 rounded-full bg-base-content overflow-hidden relative">
          <div class="h-full bg-sky-500 absolute top-0 left-0 transition duration-500" :style="{ width: `${progress}%` }"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

onMounted(() => document.documentElement.classList.add("bg-sky-200!", "dark:bg-sky-900!"));

const update = markRaw(shallowRef<Awaited<ReturnType<typeof check>>>(null));
const isChecking = ref(true);
const isInstalling = ref(false);
const progress = ref(0);
const errorMessage = ref("");

onMounted(async () => {
  try {
    const [updateResult, error] = await tryCatch(check());
    if (error) {
      errorMessage.value = error.message ?? "couldnt check for updates. womp womp";
      return console.error(error);
    }

    update.value = updateResult;
  } finally {
    isChecking.value = false;
  }
});

async function installUpdate() {
  if (!update.value) return;

  isInstalling.value = true;
  errorMessage.value = "";

  try {
    let downloaded = 0;
    let total = 0;

    await update.value.downloadAndInstall((event) => {
      switch (event.event) {
        case "Started":
          total = event.data.contentLength ?? 0;
          downloaded = 0;
          progress.value = 0;
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          if (total > 0) progress.value = Math.round((downloaded / total) * 100);
          break;
        case "Finished":
          progress.value = 100;
          break;
      }
    });

    await relaunch();
  } catch (error) {
    console.error(error);
    if (error instanceof Error) errorMessage.value = error.message ?? "couldnt install update. womp womp";
    else errorMessage.value = "couldnt install update. womp womp";
    isInstalling.value = false;
  }
}
</script>

<style scoped></style>
