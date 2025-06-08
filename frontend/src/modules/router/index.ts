import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: () => import("@/views/HomeView.vue"),
            meta: {
                navbar_mode: "normal",
            },
        },
        {
            path: "/download",
            name: "download",
            component: () => import("@/views/DownloadView.vue"),
            redirect: {
                name: "download-client-auto",
            },
            meta: {
                navbar_mode: "normal",
            },
            children: [
                {
                    path: "client/auto",
                    name: "download-client-auto",
                    component: () => import("@/views/download/Auto.vue"),
                },
                {
                    path: "client/manual",
                    name: "download-client-manual",
                    component: () => import("@/views/download/Manual.vue"),
                },
                {
                    path: "comp/mods",
                    name: "download-server-mods",
                    component: () => import("@/views/download/Comps.vue"),
                    meta: {
                        comp_type: "mods",
                    },
                },
                {
                    path: "comp/mods",
                    name: "download-server-mods",
                    component: () => import("@/views/download/Comps.vue"),
                    meta: {
                        comp_type: "mods",
                    },
                },
                {
                    path: "comp/modpacks",
                    name: "download-server-modpacks",
                    component: () => import("@/views/download/Comps.vue"),
                    meta: {
                        comp_type: "modpacks",
                    },
                },
                {
                    path: "comp/resourcepacks",
                    name: "download-server-resourcepacks",
                    component: () => import("@/views/download/Comps.vue"),
                    meta: {
                        comp_type: "resourcepacks",
                    },
                },
                {
                    path: "comp/shaders",
                    name: "download-server-shaders",
                    component: () => import("@/views/download/Comps.vue"),
                    meta: {
                        comp_type: "shaders",
                    },
                },
                {
                    path: "comp/datapacks",
                    name: "download-server-datapacks",
                    component: () => import("@/views/download/Comps.vue"),
                    meta: {
                        comp_type: "datapacks",
                    },
                },
            ],
        },
    ],
});

export default router;

