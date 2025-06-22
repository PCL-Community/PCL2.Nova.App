import { ref } from "vue";
import type { ISelectSearchOption, IShaderFeatureKey, IResourcespackFeatureKey } from "@/types/SearchOptions";

export const Resourcespack = ref<ISelectSearchOption<IResourcespackFeatureKey>>({
    audio: null,
    blocks: null,
    "core-shaders": null,
    entities: null,
    environment: null,
    equipment: null,
    fonts: null,
    gui: null,
    items: null,
    locale: null,
    models: null,
});

export const Shader = ref<ISelectSearchOption<IShaderFeatureKey>>({
    atmosphere: null,
    bloom: null,
    "colored-lighting": null,
    foliage: null,
    "path-tracing": null,
    pbr: null,
    reflections: null,
    shadows: null,
});
