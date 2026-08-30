import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  vite: {
    plugins: [tailwindcss()],
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true
    }
  },
  ignore: ["**/src-tauri/**"],
  telemetry: false,

  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  css: ["~/assets/main.css"],
  modules: ["@pinia/nuxt"],

  app: {
    head: {
      title: "Better Slides",
      meta: [
        { charset: "UTF-8" },
        { name: "viewport", content: "width=device-width, initial-scale=1.0" },
        { name: "mobile-web-app-capable", content: "yes" },
        { name: "author", content: "Kenneth Ng" },
        { property: "og:title", content: "Better Slides" },
        { property: "og:site_name", content: "Better Slides" }
      ]
    }
  }
});
