import { createRouter, createWebHistory } from "vue-router";
import ModEventBus from "./ModEventBus.ts";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            name: "FrmHome",
            component: () => import("../app/FrmHome.vue")
        },
        {
            path: "/settings",
            name: "FrmSettings",
            component: () => import("../app/FrmSettings.vue"),
            children: []
        }
    ]
});

ModEventBus.on("router:push", async (path: string) => {
    await router.push(path);
});

export default router;