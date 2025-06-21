import { ref } from "vue";
import type { ISelectSearchOptionValue, IModLoaderKey, IModpackLoaderKey, IShaderLoaderKey } from "@/types/SearchOptions";

type ILoaderSelectSearchOption<T extends string> = Record<T, ISelectSearchOptionValue>;

export const Mod = ref<ILoaderSelectSearchOption<IModLoaderKey>>({
    forge: null,
    neoforge: null,
    fabric: null,
    quilt: null,
    babric: null,
    bta: null,
    javaagent: null,
    legacyfabric: null,
    liteloader: null,
    rml: null,
    nilloader: null,
    ornithe: null,
    rift: null,
});

export const Modpack = ref<ILoaderSelectSearchOption<IModpackLoaderKey>>({
    forge: null,
    neoforge: null,
    fabric: null,
    quilt: null,
});

export const Shader = ref<ILoaderSelectSearchOption<IShaderLoaderKey>>({
    canvas: null,
    iris: null,
    optifine: null,
    vanilla: null,
});
