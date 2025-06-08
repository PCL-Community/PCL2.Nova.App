<script setup lang="ts">
    import Eventbus from "@/modules/eventbus";
    import { useConfigStore, ThemeMode } from "@/modules/stores/configStore";
    import { CError } from "@/modules/utils/logger";
    import { ref } from "vue";

    const configStore = useConfigStore();
    configStore.loadConfig();

    const $ThemeConfig = configStore.config["Customize"]["Theme"];
    const currentTheme = ref(
        $ThemeConfig["Mode"] === ThemeMode.Auto
            ? window.matchMedia("(prefers-color-scheme: dark)").matches
                ? `${$ThemeConfig["Name"]}Dark`
                : `${$ThemeConfig["Name"]}Light`
            : $ThemeConfig["Mode"] === ThemeMode.Dark
            ? `${$ThemeConfig["Name"]}Dark`
            : $ThemeConfig["Mode"] === ThemeMode.Light
            ? `${$ThemeConfig["Name"]}Light`
            : CError("[ThemeControler.vue] Invalid Theme Mode")
    );
    const themes = ["NovaLight", "NovaDark"];
    Eventbus.on("theme:change", async (theme: string, mode: typeof ThemeMode) => {
        if (!themes.includes(theme)) {
            CError("[ThemeControler.vue] Invalid Theme");
            return;
        }
        currentTheme.value = theme;
        configStore.config["Customize"]["Theme"]["Name"] = theme;
        // configStore.config["Customize"]["Theme"]["Mode"] = mode;
        await configStore.saveConfig(); // save config
    });
</script>

<template>
    <input
        v-for="theme in themes"
        :key="theme"
        :value="theme"
        :checked="currentTheme === theme"
        type="checkbox"
        class="toggle theme-controller hidden" />
</template>
