import { createApp } from "vue";
import App from "./App.vue";
import router from "./modules/ModRouter.ts";

createApp(App).use(router).mount("#app");

window.addEventListener("contextmenu", (e: MouseEvent) => e.preventDefault());

import "./assets/tailwind.css";
import "./assets/style.css";
