<script lang="ts" setup>
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import CompRadioButton from "../components/CompRadioButton.vue";
    import ModEventBus from "../../modules/ModEventBus.ts";
    import { useRoute } from "vue-router";
    
    const AppWindow = getCurrentWindow();
    
    const $route = useRoute();
    
    const handleNavigate = (path: string) => {
        ModEventBus.emit("router:push", path);
    };
</script>

<template>
    <header
        class="w-full h-14 bg-linear-to-r from-primary from-5% to-secondary to-95% flex justify-between items-center px-4"
        data-tauri-drag-region>
        <!-- Title Align=Left -->
        <!-- TODO: Dynamic Title Content -->
        <section class="text-lg font-semibold text-white flex gap-2 items-center">
            Plain Craft Launcher
            <span class="badge px-1.5 rounded-sm translate-y-[1px]">Nova</span>
        </section>
        <!-- NavigateButtons Align=Center -->
        <section class="flex gap-4 mr-4">
            <CompRadioButton :checked="$route.path === '/'" text="主页" @click="handleNavigate(`/`)" />
            <CompRadioButton :checked="$route.path.includes('/downloads')" text="下载" @click="handleNavigate(`/downloads/auto`)" />
            <CompRadioButton :checked="$route.path.includes('/links')" text="联机" @click="handleNavigate(`/links/frp`)" />
            <CompRadioButton :checked="$route.path.includes('/settings')" text="设置" @click="handleNavigate(`/settings/custom`)" />
            <CompRadioButton :checked="$route.path.includes('/others')" text="更多" @click="handleNavigate(`/others/tools`)" />
        </section>
        <!-- WindowControlButtons Align=Right -->
        <section class="flex gap-2 ">
            <button class="btn btn-ghost h-8 p-1 rounded-full border-none hover:bg-black/20"
                    @click="AppWindow.minimize()">
                <svg class="w-6" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d="M20 14H4v-4h16" fill="white" />
                </svg>
            </button>
            <button class="btn btn-ghost h-8 p-1 rounded-full border-none hover:bg-black/20"
                    @click="AppWindow.close()">
                <svg class="w-6" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M20 6.91L17.09 4L12 9.09L6.91 4L4 6.91L9.09 12L4 17.09L6.91 20L12 14.91L17.09 20L20 17.09L14.91 12z"
                        fill="white" />
                </svg>
            </button>
        </section>
    </header>
</template>

