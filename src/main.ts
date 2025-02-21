import { createApp } from "vue";
import App from "./App.vue";
import router from "./modules/ModRouter.ts";
import { $env, booleanWrapper } from "./modules/ModUtils.ts";

createApp(App).use(router).mount("#app");

import "./assets/tailwind.css";
import "./assets/style.scss";

(function () {
    window.oncontextmenu = (e: MouseEvent) => e.preventDefault();
    window.onkeydown = (e: KeyboardEvent) => {
        // 禁用刷新：[F5, Ctrl + R, Ctrl + Shift + R]
        if (
            booleanWrapper[$env.VITE_FLAG_DISABLE_REFRESH] &&
            (e.key === "F5" || (e.ctrlKey && e.key.toLowerCase() === "r") || (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "r"))
        ) {
            e.preventDefault();
        }
        // 禁用历史切换：Alt + [↑, ↓, ←, →]
        if (
            booleanWrapper[$env.VITE_FLAG_DISABLE_HISTORY] &&
            e.altKey &&
            ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.key)
        ) {
            e.preventDefault();
        }
        // 禁用在构建中的 DevTools
        if (
            booleanWrapper[$env.VITE_FLAG_DISABLE_DEVTOOLS] &&
            (e.key === "F12" || (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "i"))
        ) {
            e.preventDefault();
        }
    };
})();
