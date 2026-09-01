<template>
  <div v-if="isChecking" class="flex items-center justify-center gap-2">
    <div class="h-4 w-4 animate-spin rounded-full border-2 border-sky-500 border-t-transparent"></div>
    <p class="text-sm text-neutral-700">Checking for updates...</p>
  </div>

  <div v-else-if="errorMessage">
    <p class="text-sm text-center text-neutral-700">{{ errorMessage }}</p>
  </div>

  <div v-else-if="!update">
    <p>u good</p>
  </div>

  <div v-else class="flex select-none flex-col items-center justify-center gap-2 px-7 py-3 rounded-xl">
    <p class="text-xl font-semibold">Update available :3</p>
    <p class="text-sm text-center text-neutral-700">Upgrade to {{ update.version }}</p>

    <div v-if="update.body">
      {{ update.body }}
    </div>

    <button :disabled="isInstalling" @click="installUpdate">
      {{ isInstalling ? `Installing ${progress}%` : `Install ${update.version}` }}
    </button>

    <div v-if="isInstalling">
      <progress :value="progress" max="100"></progress>
    </div>
  </div>
</template>

<script setup lang="ts">
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

const update = ref<Awaited<ReturnType<typeof check>>>(null);
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
    if (error instanceof Error) errorMessage.value = error.message ?? "couldnt install update. womp womp";
    else errorMessage.value = "couldnt install update. womp womp";
    isInstalling.value = false;
  }
}
</script>

<style scoped></style>
