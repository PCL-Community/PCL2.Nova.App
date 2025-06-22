import type { IFilterItem, IModLoaderKey, IModpackLoaderKey, IShaderLoaderKey } from "@/types/SearchOptions";
import * as LoaderIcon from "@/icons/loader";

export const Modpack: IFilterItem<IModpackLoaderKey>[] = [
    { key: "forge", name: "Forge", icon: LoaderIcon.Forge, default: true },
    { key: "neoforge", name: "NeoForge", icon: LoaderIcon.NeoForge, default: true },
    { key: "fabric", name: "Fabric", icon: LoaderIcon.Fabric, default: true },
    { key: "quilt", name: "Quilt", icon: LoaderIcon.Quilt, default: true },
];

export const Mod: IFilterItem<IModLoaderKey>[] = [
    // Default Show
    ...Modpack,
    // More
    { key: "babric", name: "Babric", icon: LoaderIcon.Babric },
    { key: "bta", name: "BTA (Babric)", icon: LoaderIcon.BTA },
    { key: "javaagent", name: "Java Agent", icon: LoaderIcon.JavaAgent },
    { key: "legacyfabric", name: "Legacy Fabric", icon: LoaderIcon.LegacyFabric },
    { key: "liteloader", name: "LiteLoader", icon: LoaderIcon.LiteLoader },
    { key: "rml", name: "Risugami's ML", icon: LoaderIcon.RML },
    { key: "nilloader", name: "Nilloader", icon: LoaderIcon.NilLoader },
    { key: "ornithe", name: "Ornithe", icon: LoaderIcon.Ornithe },
    { key: "rift", name: "Rift", icon: LoaderIcon.Rift },
];

export const Shader: IFilterItem<IShaderLoaderKey>[] = [
    { key: "canvas", name: "Canvas", icon: LoaderIcon.Canvas, default: true },
    { key: "iris", name: "Iris", icon: LoaderIcon.Iris, default: true },
    { key: "optifine", name: "OptiFine", icon: LoaderIcon.Optifine, default: true },
    { key: "vanilla", name: "Vanilla", icon: LoaderIcon.Vanilla, default: true },
];
