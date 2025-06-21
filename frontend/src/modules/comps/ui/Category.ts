import type { IFilterItem, IModCategoryKey, IResourcespackCategoryKey, IDatapackCategoryKey, IShaderCategoryKey, IModpackCategoryKey } from "@/types/SearchOptions";
import * as CatgIcon from "@/icons/category";

export const Mod: IFilterItem<IModCategoryKey>[] = [
    { key: "adventure", name: "Adventure", icon: CatgIcon.Adventure, default: true },
    { key: "cursed", name: "Cursed", icon: CatgIcon.Cursed, default: true },
    { key: "decoration", name: "Decoration", icon: CatgIcon.Decoration, default: true },
    { key: "economy", name: "Economy", icon: CatgIcon.Economy, default: true },
    { key: "equipment", name: "Equipment", icon: CatgIcon.Equipment, default: true },
    { key: "food", name: "Food", icon: CatgIcon.Food, default: true },
    { key: "game-mechanics", name: "Game-mechanics", icon: CatgIcon.GameMechanics, default: true },
    { key: "library", name: "Library", icon: CatgIcon.Library, default: true },
    { key: "magic", name: "Magic", icon: CatgIcon.Magic, default: true },
    { key: "management", name: "Management", icon: CatgIcon.Management, default: true },
    { key: "minigame", name: "Minigame", icon: CatgIcon.Minigame, default: true },
    { key: "mobs", name: "Mobs", icon: CatgIcon.Mobs, default: true },
    { key: "optimization", name: "Optimization", icon: CatgIcon.Optimization, default: true },
    { key: "social", name: "Social", icon: CatgIcon.Social, default: true },
    { key: "storage", name: "Storage", icon: CatgIcon.Storage, default: true },
    { key: "technology", name: "Technology", icon: CatgIcon.Technology, default: true },
    { key: "transportation", name: "Transportation", icon: CatgIcon.Transportation, default: true },
    { key: "utility", name: "Utility", icon: CatgIcon.Utility, default: true },
    { key: "world-generation", name: "World Generation", icon: CatgIcon.WorldGeneration, default: true },
];

export const Resourcespack: IFilterItem<IResourcespackCategoryKey>[] = [
    { key: "combat", name: "Combat", icon: CatgIcon.Combat, default: true },
    { key: "cursed", name: "Cursed", icon: CatgIcon.Cursed, default: true },
    { key: "decoration", name: "Decoration", icon: CatgIcon.Decoration, default: true },
    { key: "modded", name: "Modded", icon: CatgIcon.Modded, default: true },
    { key: "realistic", name: "Realistic", icon: CatgIcon.Realistic, default: true },
    { key: "simplistic", name: "Simplistic", icon: CatgIcon.Simplistic, default: true },
    { key: "themed", name: "Themed", icon: CatgIcon.Themed, default: true },
    { key: "tweaks", name: "Tweaks", icon: CatgIcon.Tweaks, default: true },
    { key: "utility", name: "Utility", icon: CatgIcon.Utility, default: true },
    { key: "vanilla-like", name: "Vanilla Like", icon: CatgIcon.VanillaLike, default: true },
];

export const Datapack: IFilterItem<IDatapackCategoryKey>[] = Mod;

export const Shader: IFilterItem<IShaderCategoryKey>[] = [
    { key: "cartoon", name: "Cartoon", icon: CatgIcon.Cartoon, default: true },
    { key: "cursed", name: "Cursed", icon: CatgIcon.Cursed, default: true },
    { key: "fantasy", name: "Fantasy", icon: CatgIcon.Fantasy, default: true },
    { key: "realistic", name: "Realistic", icon: CatgIcon.Realistic, default: true },
    { key: "semi-realistic", name: "Semi Realistic", icon: CatgIcon.SemiRealistic, default: true },
    { key: "vanilla-like", name: "Vanilla-like", icon: CatgIcon.VanillaLike, default: true },
];

export const Modpack: IFilterItem<IModpackCategoryKey>[] = [
    { key: "adventure", name: "Adventure", icon: CatgIcon.Adventure, default: true },
    { key: "challenging", name: "Challenging", icon: CatgIcon.Challenging, default: true },
    { key: "combat", name: "Combat", icon: CatgIcon.Combat, default: true },
    { key: "kitchen-sink", name: "Kitchen Sink", icon: CatgIcon.KitchenSink, default: true },
    { key: "lightweight", name: "Lightweight", icon: CatgIcon.Lightweight, default: true },
    { key: "magic", name: "Magic", icon: CatgIcon.Magic, default: true },
    { key: "multiplayer", name: "Multiplayer", icon: CatgIcon.Multiplayer, default: true },
    { key: "optimization", name: "Optimization", icon: CatgIcon.Optimization, default: true },
    { key: "quests", name: "Quests", icon: CatgIcon.Quests, default: true },
    { key: "technology", name: "Technology", icon: CatgIcon.Technology, default: true },
];
