import { defineStore } from "pinia";
import { CLog, CWarn } from "../utils/logger";
import { LoadOrInitConfig, ReadConfig, WriteConfig } from "@/modules/wailsjs/go/config/ConfigBinding";
import { config } from "@/modules/wailsjs/go/models";

export enum ThemeMode {
    Light = "Light",
    Dark = "Dark",
    Auto = "Auto",
}

export const useConfigStore = defineStore("config", {
    state: () => ({
        config: {
            Customize: {
                Theme: {
                    Name: "Nova",
                    Mode: ThemeMode.Auto,
                },
            },
        } as config.Config,
    }),

    actions: {
        async loadConfig() {
            try {
                this.config = await LoadOrInitConfig();
                this.config = await ReadConfig();
            } catch (e) {
                CWarn("[Config] Failed to load config, using default config", e);
                await this.saveConfig();
            }
            CLog("[Config] Loaded config");
        },

        async saveConfig() {
            await WriteConfig(this.config);
        },

        updateConfig(newConfig: Record<string, any>) {
            this.config = {
                ...this.config,
                ...newConfig,
            };
        },
    },
});

