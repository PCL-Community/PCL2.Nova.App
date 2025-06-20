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
                // Clients
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
                // Components
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
                // Favorites
                {
                    path: "favorites",
                    name: "download-favorites",
                    component: () => import("@/views/download/Favorites.vue"),
                },
            ],
        },
        {
            path: "/settings",
            name: "settings",
            component: () => import("@/views/SettingsView.vue"),
            redirect: {
                name: "settings-launch",
            },
            meta: {
                navbar_mode: "normal",
            },
            children: [
                {
                    path: "launch",
                    name: "settings-launch",
                    component: () => import("@/views/settings/Launch.vue"),
                },
                {
                    path: "personalization",
                    name: "settings-personalization",
                    component: () => import("@/views/settings/Personalization.vue"),
                },
                {
                    path: "others",
                    name: "settings-others",
                    component: () => import("@/views/settings/Others.vue"),
                },
            ],
        },
        {
            path: "/more",
            name: "more",
            component: () => import("@/views/MoreView.vue"),
            meta: {
                navbar_mode: "normal",
            },
        },
    ],
});

export default router;

