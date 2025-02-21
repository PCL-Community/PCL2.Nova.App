import { createRouter, createWebHistory } from "vue-router";
import { ModEventBus } from "./ModEventBus.ts";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        // Home
        {
            path: "/",
            name: "FrmHome",
            component: () => import("../app/FrmHome.vue"),
            children: [
                {
                    path: "",
                    name: "View",
                    component: () => import("../ui/views/home/View.vue"),
                },
                {
                    path: "firework",
                    name: "ViewFirework",
                    component: () => import("../ui/views/home/ViewFirework.vue"),
                },
            ],
        },
        // Downloads
        {
            path: "/downloads",
            name: "FrmDownloads",
            component: () => import("../app/FrmDownloads.vue"),
            children: [
                {
                    path: "auto",
                    name: "ViewDownloadsAuto",
                    component: () => import("../ui/views/downloads/ViewAuto.vue"),
                },
                {
                    path: "manual",
                    name: "ViewDownloadsManual",
                    component: () => import("../ui/views/downloads/ViewManual.vue"),
                },
                {
                    path: "mods",
                    name: "ViewDownloadsMods",
                    component: () => import("../ui/views/downloads/ViewMods.vue"),
                },
                {
                    path: "modpacks",
                    name: "ViewDownloadsModpacks",
                    component: () => import("../ui/views/downloads/ViewModpacks.vue"),
                },
                {
                    path: "resources",
                    name: "ViewDownloadsResources",
                    component: () => import("../ui/views/downloads/ViewResources.vue"),
                },
                {
                    path: "shaders",
                    name: "ViewDownloadsShaders",
                    component: () => import("../ui/views/downloads/ViewShaders.vue"),
                },
                {
                    path: "favorite/client",
                    name: "ViewDownloadsFavoriteClient",
                    component: () => import("../ui/views/downloads/ViewFavoriteClient.vue"),
                },
                {
                    path: "favorite/extra",
                    name: "ViewDownloadsFavoriteExtra",
                    component: () => import("../ui/views/downloads/ViewFavoriteExtra.vue"),
                },
            ],
        },
        // Links
        {
            path: "/links",
            name: "FrmLinks",
            component: () => import("../app/FrmLinks.vue"),
            children: [
                {
                    path: "frp",
                    name: "ViewLinksFrp",
                    component: () => import("../ui/views/links/ViewFrp.vue"),
                },
                {
                    path: "ioi",
                    name: "ViewLinksIOI",
                    component: () => import("../ui/views/links/ViewIOI.vue"),
                },
                {
                    path: "octo",
                    name: "ViewLinksOcto",
                    component: () => import("../ui/views/links/ViewOcto.vue"),
                },
            ],
        },
        // Settings
        {
            path: "/settings",
            name: "FrmSettings",
            component: () => import("../app/FrmSettings.vue"),
            children: [
                {
                    path: "custom",
                    name: "ViewSettingsCustom",
                    component: () => import("../ui/views/settings/ViewCustom.vue"),
                },
                {
                    path: "launch",
                    name: "ViewSettingsLaunch",
                    component: () => import("../ui/views/settings/ViewLaunch.vue"),
                },
                {
                    path: "system",
                    name: "ViewSettingsSystem",
                    component: () => import("../ui/views/settings/ViewSystem.vue"),
                },
            ],
        },
        // Others
        {
            path: "/others",
            name: "FrmOthers",
            component: () => import("../app/FrmOthers.vue"),
            children: [
                {
                    path: "tools",
                    name: "ViewOthersTools",
                    component: () => import("../ui/views/others/ViewTools.vue"),
                },
                {
                    path: "about",
                    name: "ViewOthersAbout",
                    component: () => import("../ui/views/others/ViewAbout.vue"),
                },
            ],
        },
    ],
});

router.beforeEach((to, from, next) => {
    const isSameParent = to.fullPath.split("/")[1] === from.fullPath.split("/")[1];
    if (!isSameParent) {
        const main = (<HTMLElement>document.body.querySelector("div#app")).querySelector("main");
        main?.classList.add("fadeout");
        setTimeout(next, 150);
    } else {
        const subrouter = (<HTMLElement>document.body.querySelector("div#app")).querySelector("section.subrouter");
        subrouter?.classList.add("fadeout");
        setTimeout(next, 150);
    }
});

router.afterEach((to, _from, _failure) => {
    const main = (<HTMLElement>document.body.querySelector("div#app")).querySelector("main");
    main?.classList.remove("fadeout");
    const subrouter = (<HTMLElement>document.body.querySelector("div#app")).querySelector("section.subrouter");
    subrouter?.classList.remove("fadeout");
    console.log("to.fullPath", to.fullPath);
    ModEventBus.emit("router:done", to.fullPath);
});

ModEventBus.on("router:push", async (path: string) => {
    await router.push(path);
});

export default router;
