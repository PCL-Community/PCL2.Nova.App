import type { Component } from "vue";

// Types & Interfaces
export type ISelectSearchOptionValue = boolean | null;
export type ISelectSearchOption<T extends string> = Record<T, ISelectSearchOptionValue>;
export interface IBaseFilterItem<T extends string> {
    key: T;
    default?: boolean;
}
export interface IFilterItemNoIcon<T extends string> extends IBaseFilterItem<T> {
    name: string;
}
export interface IFilterItem<T extends string> extends IFilterItemNoIcon<T> {
    icon: Component;
}

// Keys
export type { IModLoaderKey, IModpackLoaderKey, IShaderLoaderKey } from "./key/LoaderKey";
export type { IVersionKey } from "./key/VersionKey";
export type { IModCategoryKey, IResourcespackCategoryKey, IDatapackCategoryKey, IShaderCategoryKey, IModpackCategoryKey } from "./key/CategoryKey";
export type { IResourcespackFeatureKey, IShaderFeatureKey } from "./key/FeatureKey";
export type { IResolutionsKey, IPerfKey } from "./key/MoreKey";
