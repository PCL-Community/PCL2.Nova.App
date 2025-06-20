import { createRouter, createWebHistory } from "vue-router";
import HomeLeft from "@/fragments/HomeLeft.vue";
import DownloadLeft from "@/fragments/DownloadLeft.vue";
import SettingsLeft from "@/fragments/SettingsLeft.vue";
import MoreLeft from "@/fragments/MoreLeft.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: () => import("@/views/HomeView.vue"),
            meta: {
                navbar_mode: "normal",
                left_comp: HomeLeft,
                left_width: "33%",
                left_id: "E72035ED-F959-43A4-B01A-50E7A5606C37",
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
                left_comp: DownloadLeft,
                left_width: "calc(var(--spacing)*36)",
                left_id: "935475B0-D428-4D7A-9361-36AB29CEDFA1",
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
                left_comp: SettingsLeft,
                left_width: "calc(var(--spacing)*28)",
                left_id: "748EC7CE-65BE-4C20-B5F4-9B38BACD2C3E",
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
            redirect: {
                name: "more-help",
            },
            meta: {
                navbar_mode: "normal",
                left_comp: MoreLeft,
                left_width: "calc(var(--spacing)*34)",
                left_id: "8F73FEA2-847D-404E-990B-C9EC69B2B779",
            },
            children: [
                {
                    path: "help",
                    name: "more-help",
                    component: () => import("@/views/more/Help.vue"),
                },
                {
                    path: "credits",
                    name: "more-credits",
                    component: () => import("@/views/more/Credits.vue"),
                },
                {
                    path: "tools",
                    name: "more-tools",
                    component: () => import("@/views/more/Tools.vue"),
                },
            ],
        },
    ],
});

export default router;

