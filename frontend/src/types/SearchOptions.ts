export type ISelectSearchOptionValue = boolean | null;
export type ISelectSearchOption<T extends string> = Record<T, ISelectSearchOptionValue>;
export interface IFilterItem<T> {
    key: T;
    name: string;
    icon: any;
    default?: boolean;
}

export type { IModLoaderKey, IModpackLoaderKey, IShaderLoaderKey } from "./key/LoaderKey";
export type { IVersionKey } from "./key/VersionKey";
export type { IModCategoryKey, IResourcespackCategoryKey, IDatapackCategoryKey, IShaderCategoryKey, IModpackCategoryKey } from "./key/CategoryKey";
