import type { BindingVersionInst, BindingLoaderInst, BindingCategoryInst, BindingFeatureInst, BindingMoreInst } from "..";

export type IQueryType = "mods" | "modpacks" | "resourcepacks" | "shaders" | "datapacks";
type IQueryVersion = typeof BindingVersionInst.Default.value;
type IQueryLoader = typeof BindingLoaderInst.Mod.value | typeof BindingLoaderInst.Modpack.value | typeof BindingLoaderInst.Shader.value;
type IQueryCategory =
    | typeof BindingCategoryInst.Mod.value
    | typeof BindingCategoryInst.Modpack.value
    | typeof BindingCategoryInst.Resourcespack.value
    | typeof BindingCategoryInst.Datapack.value
    | typeof BindingCategoryInst.Shader.value;
type IQueryFeature = typeof BindingFeatureInst.Resourcespack.value | typeof BindingFeatureInst.Shader.value;
type IQueryResolution = typeof BindingMoreInst.Resolution.value;
type IQueryPerf = typeof BindingMoreInst.Perf.value;

export const BuildQueryParams = (
    type: IQueryType,
    version: IQueryVersion,
    loader: IQueryLoader,
    category: IQueryCategory,
    feature: IQueryFeature,
    resolution: IQueryResolution,
    perf: IQueryPerf
) => {
    const params: string[] = [];

    // 添加项目类型
    params.push(`["project_type:${type}"]`);

    // 通用处理函数
    const handleParamGroup = (
        obj: IQueryVersion | IQueryLoader | IQueryCategory | IQueryFeature | IQueryResolution | IQueryPerf,
        keyPrefix: string,
    ) => {
        const included: string[] = [];
        const results: string[] = [];

        for (const [key, value] of Object.entries(obj)) {
            if (value === true) {
                included.push(`["${keyPrefix}=${key}"]`);
            } else if (value === false && keyPrefix !== "versions") {
                results.push(`["${keyPrefix}!=${key}"]`);
            }
        }

        if (included.length === 1) {
            results.push(included[0]);
        } else if (included.length >= 2) {
            results.push(`[${included.join(",")}]`);
        }

        if (results.length === 1) {
            params.push(results[0]);
        } else if (results.length > 1) {
            params.push(`[${results.join(",")}]`);
        }
    };

    // 分组处理
    handleParamGroup(version, "versions");
    handleParamGroup(loader, "categories");
    handleParamGroup(category, "categories");
    handleParamGroup(feature, "features");
    handleParamGroup(resolution, "resolution");
    handleParamGroup(perf, "perf");

    return `[${params.join(",")}]`;
};
