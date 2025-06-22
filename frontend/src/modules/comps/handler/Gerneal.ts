import type { Ref } from "vue";
import type {
    // Loader Keys
    IModLoaderKey,
    IModpackLoaderKey,
    IShaderLoaderKey,
    // Category Keys
    IModCategoryKey,
    IResourcespackCategoryKey,
    IDatapackCategoryKey,
    IShaderCategoryKey,
    IModpackCategoryKey,
    // Feature Keys
    IResourcespackFeatureKey,
    IShaderFeatureKey,
    // More Keys
    IResolutionsKey,
    IPerfKey,
} from "@/types/SearchOptions";

export function handleUpdateInst(search_mod_loader: Ref<Record<IModLoaderKey, boolean | null>>, type: "select" | "exclude", key: IModLoaderKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IModpackLoaderKey, boolean | null>>, type: "select" | "exclude", key: IModpackLoaderKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IShaderLoaderKey, boolean | null>>, type: "select" | "exclude", key: IShaderLoaderKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IModCategoryKey, boolean | null>>, type: "select" | "exclude", key: IModCategoryKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IResourcespackCategoryKey, boolean | null>>, type: "select" | "exclude", key: IResourcespackCategoryKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IDatapackCategoryKey, boolean | null>>, type: "select" | "exclude", key: IDatapackCategoryKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IShaderCategoryKey, boolean | null>>, type: "select" | "exclude", key: IShaderCategoryKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IModpackCategoryKey, boolean | null>>, type: "select" | "exclude", key: IModpackCategoryKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IResourcespackFeatureKey, boolean | null>>, type: "select" | "exclude", key: IResourcespackFeatureKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IShaderFeatureKey, boolean | null>>, type: "select" | "exclude", key: IShaderFeatureKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IResolutionsKey, boolean | null>>, type: "select" | "exclude", key: IResolutionsKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: Ref<Record<IPerfKey, boolean | null>>, type: "select" | "exclude", key: IPerfKey, value: boolean): void;
export function handleUpdateInst(search_mod_loader: any, type: "select" | "exclude", key: any, value: boolean) {
    switch (type) {
        case "select":
            search_mod_loader.value[key] = value ? true : null;
            break;
        case "exclude":
            search_mod_loader.value[key] = value ? false : null;
            break;
    }
}
