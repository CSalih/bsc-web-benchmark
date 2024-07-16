import "./assets/main.css";

import { createApp } from "vue";
// @ts-expect-error: App is written in JS, no types available
import App from "./App.vue";

createApp(App).mount("#app");
