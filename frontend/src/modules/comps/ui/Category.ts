import type { IFilterItem, IModCategoryKey, IResourcespackCategoryKey, IDatapackCategoryKey, IShaderCategoryKey, IModpackCategoryKey } from "@/types/SearchOptions";
import * as CatgIcon from "@/icons/category";

export const Mod: IFilterItem<IModCategoryKey>[] = [
    { key: "adventure", name: "探索", icon: CatgIcon.Adventure, default: true },
    { key: "cursed", name: "诅咒", icon: CatgIcon.Cursed, default: true },
    { key: "decoration", name: "装饰", icon: CatgIcon.Decoration, default: true },
    { key: "economy", name: "经济", icon: CatgIcon.Economy, default: true },
    { key: "equipment", name: "装备与工具", icon: CatgIcon.Equipment, default: true },
    { key: "food", name: "食物", icon: CatgIcon.Food, default: true },
    { key: "game-mechanics", name: "游戏机制", icon: CatgIcon.GameMechanics, default: true },
    { key: "library", name: "支持库", icon: CatgIcon.Library, default: true },
    { key: "magic", name: "魔法", icon: CatgIcon.Magic, default: true },
    { key: "management", name: "配置", icon: CatgIcon.Management, default: true },
    { key: "minigame", name: "小游戏", icon: CatgIcon.Minigame, default: true },
    { key: "mobs", name: "生物", icon: CatgIcon.Mobs, default: true },
    { key: "optimization", name: "优化", icon: CatgIcon.Optimization, default: true },
    { key: "social", name: "社交", icon: CatgIcon.Social, default: true },
    { key: "storage", name: "仓储", icon: CatgIcon.Storage, default: true },
    { key: "technology", name: "科技", icon: CatgIcon.Technology, default: true },
    { key: "transportation", name: "运输", icon: CatgIcon.Transportation, default: true },
    { key: "utility", name: "实用", icon: CatgIcon.Utility, default: true },
    { key: "world-generation", name: "世界生成", icon: CatgIcon.WorldGeneration, default: true },
];

export const Resourcespack: IFilterItem<IResourcespackCategoryKey>[] = [
    { key: "combat", name: "战斗", icon: CatgIcon.Combat, default: true },
    { key: "cursed", name: "诅咒", icon: CatgIcon.Cursed, default: true },
    { key: "decoration", name: "装饰", icon: CatgIcon.Decoration, default: true },
    { key: "modded", name: "模组相关", icon: CatgIcon.Modded, default: true },
    { key: "realistic", name: "写实", icon: CatgIcon.Realistic, default: true },
    { key: "simplistic", name: "简单", icon: CatgIcon.Simplistic, default: true },
    { key: "themed", name: "主题", icon: CatgIcon.Themed, default: true },
    { key: "tweaks", name: "改良", icon: CatgIcon.Tweaks, default: true },
    { key: "utility", name: "实用", icon: CatgIcon.Utility, default: true },
    { key: "vanilla-like", name: "原版风", icon: CatgIcon.VanillaLike, default: true },
];

export const Datapack: IFilterItem<IDatapackCategoryKey>[] = Mod;

export const Shader: IFilterItem<IShaderCategoryKey>[] = [
    { key: "cartoon", name: "卡通", icon: CatgIcon.Cartoon, default: true },
    { key: "cursed", name: "诅咒", icon: CatgIcon.Cursed, default: true },
    { key: "fantasy", name: "幻想", icon: CatgIcon.Fantasy, default: true },
    { key: "realistic", name: "写实", icon: CatgIcon.Realistic, default: true },
    { key: "semi-realistic", name: "半写实", icon: CatgIcon.SemiRealistic, default: true },
    { key: "vanilla-like", name: "原版风", icon: CatgIcon.VanillaLike, default: true },
];

export const Modpack: IFilterItem<IModpackCategoryKey>[] = [
    { key: "adventure", name: "探索", icon: CatgIcon.Adventure, default: true },
    { key: "challenging", name: "挑战", icon: CatgIcon.Challenging, default: true },
    { key: "combat", name: "战斗", icon: CatgIcon.Combat, default: true },
    { key: "kitchen-sink", name: "水槽包", icon: CatgIcon.KitchenSink, default: true },
    { key: "lightweight", name: "轻量整合", icon: CatgIcon.Lightweight, default: true },
    { key: "magic", name: "魔法", icon: CatgIcon.Magic, default: true },
    { key: "multiplayer", name: "多人联机", icon: CatgIcon.Multiplayer, default: true },
    { key: "optimization", name: "优化", icon: CatgIcon.Optimization, default: true },
    { key: "quests", name: "任务", icon: CatgIcon.Quests, default: true },
    { key: "technology", name: "科技", icon: CatgIcon.Technology, default: true },
];
