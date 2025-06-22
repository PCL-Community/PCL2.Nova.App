import { ref } from "vue";
import type { ISelectSearchOption, IResolutionsKey, IPerfKey } from "@/types/SearchOptions";

export const Resolution = ref<ISelectSearchOption<IResolutionsKey>>({
    "8x-": null,
    "16x": null,
    "32x": null,
    "48x": null,
    "64x": null,
    "128x": null,
    "256x": null,
    "512x+": null,
});

export const Perf = ref<ISelectSearchOption<IPerfKey>>({
    high: null,
    medium: null,
    low: null,
    potato: null,
    screenshot: null,
});
