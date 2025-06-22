import type { IFilterItem, IResourcespackFeatureKey, IShaderFeatureKey } from "@/types/SearchOptions";
import * as FeatIcon from "@/icons/features";

export const Resourcespack: IFilterItem<IResourcespackFeatureKey>[] = [
    { key: "audio", name: "音效", icon: FeatIcon.Audio, default: true },
    { key: "blocks", name: "方块", icon: FeatIcon.Blocks, default: true },
    { key: "core-shaders", name: "核心着色器", icon: FeatIcon.CoreShaders, default: true },
    { key: "entities", name: "实体", icon: FeatIcon.Entities, default: true },
    { key: "environment", name: "环境", icon: FeatIcon.Environment, default: true },
    { key: "equipment", name: "装备与工具", icon: FeatIcon.Equipment, default: true },
    { key: "fonts", name: "字体包", icon: FeatIcon.Fonts, default: true },
    { key: "gui", name: "用户界面", icon: FeatIcon.Gui, default: true },
    { key: "items", name: "物品", icon: FeatIcon.Items, default: true },
    { key: "locale", name: "语言包", icon: FeatIcon.Locale, default: true },
    { key: "models", name: "模型", icon: FeatIcon.Models, default: true },
];

export const Shader: IFilterItem<IShaderFeatureKey>[] = [
    { key: "atmosphere", name: "大气层", icon: FeatIcon.Atmosphere, default: true },
    { key: "bloom", name: "泛光", icon: FeatIcon.Bloom, default: true },
    { key: "colored-lighting", name: "彩色光照", icon: FeatIcon.ColoredLighting, default: true },
    { key: "foliage", name: "树叶", icon: FeatIcon.Foliage, default: true },
    { key: "path-tracing", name: "光线追踪", icon: FeatIcon.PathTracing, default: true },
    { key: "pbr", name: "PBR", icon: FeatIcon.PBR, default: true },
    { key: "reflections", name: "反射", icon: FeatIcon.Reflections, default: true },
    { key: "shadows", name: "阴影", icon: FeatIcon.Shadows, default: true },
];
