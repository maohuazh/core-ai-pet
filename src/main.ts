import { createApp } from "vue";
import App from "./App.vue";
import { triggerHandler } from "./core/events/triggerHandler";

const app = createApp(App);

app.mount("#app");

// Initialize trigger handler after app is mounted
// This will start listening for external events and the daily tick timer
triggerHandler.init().catch((e) => {
  console.error("Failed to initialize trigger handler:", e);
});
