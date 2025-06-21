import { ref } from "vue";
import type { IDatapackCategoryKey, IModCategoryKey, IModpackCategoryKey, IResourcespackCategoryKey, ISelectSearchOption, IShaderCategoryKey } from "@/types/SearchOptions";

export const Mod = ref<ISelectSearchOption<IModCategoryKey>>({
    adventure: null,
    cursed: null,
    decoration: null,
    economy: null,
    equipment: null,
    food: null,
    "game-mechanics": null,
    library: null,
    magic: null,
    management: null,
    minigame: null,
    mobs: null,
    optimization: null,
    social: null,
    storage: null,
    technology: null,
    transportation: null,
    utility: null,
    "world-generation": null,
});

export const Resourcespack = ref<ISelectSearchOption<IResourcespackCategoryKey>>({
    combat: null,
    cursed: null,
    decoration: null,
    modded: null,
    realistic: null,
    simplistic: null,
    themed: null,
    tweaks: null,
    utility: null,
    "vanilla-like": null,
});

export const Datapack = ref<ISelectSearchOption<IDatapackCategoryKey>>({
    adventure: null,
    cursed: null,
    decoration: null,
    economy: null,
    equipment: null,
    food: null,
    "game-mechanics": null,
    library: null,
    magic: null,
    management: null,
    minigame: null,
    mobs: null,
    optimization: null,
    social: null,
    storage: null,
    technology: null,
    transportation: null,
    utility: null,
    "world-generation": null,
});

export const Shader = ref<ISelectSearchOption<IShaderCategoryKey>>({
    cartoon: null,
    cursed: null,
    fantasy: null,
    realistic: null,
    "semi-realistic": null,
    "vanilla-like": null,
});

export const Modpack = ref<ISelectSearchOption<IModpackCategoryKey>>({
    adventure: null,
    challenging: null,
    combat: null,
    "kitchen-sink": null,
    lightweight: null,
    magic: null,
    multiplayer: null,
    optimization: null,
    quests: null,
    technology: null,
});
