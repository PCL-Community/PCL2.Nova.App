<script lang="ts" setup>
    import { onMounted } from "vue";
    import { ModEventBus } from "./modules/ModEventBus";
    import { LogLevel } from "./modules/rust/ModLogger";
    import metadata from "./metadata.json"
    import LayoutHeader from "./ui/layouts/LayoutHeader.vue";

    onMounted(() => {
        if (JSON.parse(import.meta.env.VITE_FLAG_LAUNCH_SOUND)) {
            const welcome = new Audio("/Popup.SAO.Welcome.wav");
            welcome.volume = 0.35;
            welcome.play();
        }
        ModEventBus.emit("logger:modify", "log.txt");
        ModEventBus.emit("logger:log", {
            message: `PCL II: Nova [渠道 ${metadata.channel} | 版本 ${metadata.version}] 已启动！`,
            level: LogLevel.INFO,
        });
    });
</script>

<template>
    <LayoutHeader />
    <main>
        <RouterView />
    </main>
</template>
