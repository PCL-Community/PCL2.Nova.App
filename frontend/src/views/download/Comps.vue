<script setup lang="ts">
    import { computed, nextTick, onMounted, ref } from "vue";
    import { useRoute } from "vue-router";
    import { HintBar, CompSearchInput, FlowContainer, Card, CompSearchFilterItem, CompSearchFilterToggle, CompItem } from "@/components";
    import type {
        // Version Key
        IVersionKey,
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
    import {
        // Bindings
        BindingLoaderInst,
        BindingCategoryInst,
        BindingFeatureInst,
        BindingMoreInst,
        BindingVersionInst,
        // Filter Options
        OptionCategoryInst,
        OptionFeatureInst,
        OptionLoaderInst,
        OptionMoreInst,
        OptionVersionInst,
        // Handlers
        handleUpdateInst,
    } from "@/modules/comps";
    import { BuildQueryParams, type IQueryType } from "@/modules/comps/handler/Querys";
    import { CompTypeMapper, ModrinthApiUrl } from "@/modules/Const";
    import { HttpGet } from "@/modules/Network";
    import type { IModrinthProject } from "@/types/ModrinthProject";
    import LoadingAnimation from "@/components/LoadingAnimation.vue";

    /**
     * 查询参数相关
     */

    // Global
    const route = useRoute();

    // Binding
    const search_input = ref("");
    const search_version = BindingVersionInst.Default;
    // Loader 绑定
    const search_mod_loader = BindingLoaderInst.Mod;
    const search_modpack_loader = BindingLoaderInst.Modpack;
    const search_shader_loader = BindingLoaderInst.Shader;
    // Category 绑定
    const search_mod_category = BindingCategoryInst.Mod;
    const search_resourcespack_category = BindingCategoryInst.Resourcespack;
    const search_datapack_category = BindingCategoryInst.Datapack;
    const search_shader_category = BindingCategoryInst.Shader;
    const search_modpack_category = BindingCategoryInst.Modpack;
    // Features 绑定
    const search_resourcespack_feature = BindingFeatureInst.Resourcespack;
    const search_shader_feature = BindingFeatureInst.Shader;
    // More 绑定
    const search_resolution = BindingMoreInst.Resolution;
    const search_perf = BindingMoreInst.Perf;

    // Filter Options
    // Input 没有筛选器，写个注释让代码块长度一致 :D
    const version_filters = OptionVersionInst.Default;
    // Loader 选项
    const loader_mod_filters = OptionLoaderInst.Mod;
    const loader_modpack_filters = OptionLoaderInst.Modpack;
    const loader_shader_filters = OptionLoaderInst.Shader;
    // Category 选项
    const category_mod_filters = OptionCategoryInst.Mod;
    const category_resourcespack_filters = OptionCategoryInst.Resourcespack;
    const category_datapack_filters = OptionCategoryInst.Datapack;
    const category_shader_filters = OptionCategoryInst.Shader;
    const category_modpack_filters = OptionCategoryInst.Modpack;
    // Features 选项
    const feature_resourcespack_filters = OptionFeatureInst.Resourcespack;
    const feature_shader_filters = OptionFeatureInst.Shader;
    // More 选项
    const more_resolution_filters = OptionMoreInst.Resolution;
    const more_perf_filters = OptionMoreInst.Perf;

    // Filter Toggling More
    // Input 没有啥可展开的，所以这里也不需要
    const toggle_version = ref<boolean>(false);
    const toggle_mod_loader = ref<boolean>(false);
    // 剩下的也没有需要展开的了

    // Handlers
    // Version 的处理函数
    const handleVersionUpdate = (key: IVersionKey) => (search_version.value[key] = !search_version.value[key]);
    // Loader 的处理函数们
    const handleModLoaderUpdate = (type: "select" | "exclude", key: IModLoaderKey, value: boolean) => handleUpdateInst(search_mod_loader, type, key, value);
    const handleModpackLoaderUpdate = (type: "select" | "exclude", key: IModpackLoaderKey, value: boolean) => handleUpdateInst(search_modpack_loader, type, key, value);
    const handleShaderLoaderUpdate = (type: "select" | "exclude", key: IShaderLoaderKey, value: boolean) => handleUpdateInst(search_shader_loader, type, key, value);
    // Category 的处理函数们
    const handleModCategoryUpdate = (type: "select" | "exclude", key: IModCategoryKey, value: boolean) => handleUpdateInst(search_mod_category, type, key, value);
    const handleResourcespackCategoryUpdate = (type: "select" | "exclude", key: IResourcespackCategoryKey, value: boolean) =>
        handleUpdateInst(search_resourcespack_category, type, key, value);
    const handleDatapackCategoryUpdate = (type: "select" | "exclude", key: IDatapackCategoryKey, value: boolean) => handleUpdateInst(search_datapack_category, type, key, value);
    const handleShaderCategoryUpdate = (type: "select" | "exclude", key: IShaderCategoryKey, value: boolean) => handleUpdateInst(search_shader_category, type, key, value);
    const handleModpackCategoryUpdate = (type: "select" | "exclude", key: IModpackCategoryKey, value: boolean) => handleUpdateInst(search_modpack_category, type, key, value);
    // Features 的处理函数们
    const handleResourcespackFeatureUpdate = (type: "select" | "exclude", key: IResourcespackFeatureKey, value: boolean) =>
        handleUpdateInst(search_resourcespack_feature, type, key, value);
    const handleShaderFeatureUpdate = (type: "select" | "exclude", key: IShaderFeatureKey, value: boolean) => handleUpdateInst(search_shader_feature, type, key, value);
    // More 的处理函数们
    const handleMoreResolutionUpdate = (type: "select" | "exclude", key: IResolutionsKey, value: boolean) => handleUpdateInst(search_resolution, type, key, value);
    const handleMorePerfUpdate = (type: "select" | "exclude", key: IPerfKey, value: boolean) => handleUpdateInst(search_perf, type, key, value);

    // Computed Bindings
    const binding_loader = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mod":
            default:
                return search_mod_loader.value;
            case "modpack":
                return search_modpack_loader.value;
            case "shader":
                return search_shader_loader.value;
        }
    });
    const binding_category = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mod":
            default:
                return search_mod_category.value;
            case "modpack":
                return search_modpack_category.value;
            case "shader":
                return search_shader_category.value;
            case "resourcepack":
                return search_resourcespack_category.value;
            case "datapack":
                return search_datapack_category.value;
        }
    });
    const binding_feature = computed(() => {
        switch (route.meta["comp_type"]) {
            case "resourcepack":
            default:
                return search_resourcespack_feature.value;
            case "shader":
                return search_shader_feature.value;
        }
    });

    // Computed Filters
    const loader_filters = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mod":
            default:
                return loader_mod_filters;
            case "modpack":
                return loader_modpack_filters;
            case "shader":
                return loader_shader_filters;
        }
    });
    const category_filters = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mod":
            default:
                return category_mod_filters;
            case "modpack":
                return category_modpack_filters;
            case "shader":
                return category_shader_filters;
            case "resourcepack":
                return category_resourcespack_filters;
            case "datapack":
                return category_datapack_filters;
        }
    });
    const feature_filters = computed(() => {
        switch (route.meta["comp_type"]) {
            case "resourcepack":
            default:
                return feature_resourcespack_filters;
            case "shader":
                return feature_shader_filters;
        }
    });

    // Computed Handlers
    const handleLoaderUpdate = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mod":
            default:
                return handleModLoaderUpdate;
            case "modpack":
                return handleModpackLoaderUpdate;
            case "shader":
                return handleShaderLoaderUpdate;
        }
    });
    const handleCategoryUpdate = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mod":
            default:
                return handleModCategoryUpdate;
            case "modpack":
                return handleModpackCategoryUpdate;
            case "shader":
                return handleShaderCategoryUpdate;
            case "resourcepack":
                return handleResourcespackCategoryUpdate;
            case "datapack":
                return handleDatapackCategoryUpdate;
        }
    });
    const handleFeatureUpdate = computed(() => {
        switch (route.meta["comp_type"]) {
            case "resourcepack":
            default:
                return handleResourcespackFeatureUpdate;
            case "shader":
                return handleShaderFeatureUpdate;
        }
    });
    const handleMoreUpdate = computed(() => {
        switch (route.meta["comp_type"]) {
            case "resourcepack":
            default:
                return handleMoreResolutionUpdate;
            case "shader":
                return handleMorePerfUpdate;
        }
    });

    // (perf) defer loading
    const show_filter = ref<boolean>(false);
    onMounted(() => {
        nextTick(() => {
            show_filter.value = true;
        });
    });

    /**
     * 查询功能相关
     */
    let cache_latest_p: string[] = [];
    const handleQuery = async (useOffset: boolean) => {
        // 构建 Query Params
        const params = [];
        if (search_input.value) {
            params.push("query=" + search_input.value);
        }
        params.push(
            "facets=" +
                BuildQueryParams(
                    route.meta["comp_type"] as IQueryType,
                    search_version.value,
                    binding_loader.value,
                    binding_category.value,
                    binding_feature.value,
                    search_resolution.value,
                    search_perf.value
                )
        );
        if (useOffset) {
            params.push("offset=" + current_offset.value);
        }
        params.push("limit=20");

        // 校验是否发起过请求
        if (cache_latest_p.join("&") === params.join("&")) {
            in_loading.value = false;
            return;
        }

        // 初始化加载项
        cache_latest_p = params;
        in_loading.value = true;
        current_hits.value.length = 0;

        // 正式加载
        const resp = await HttpGet(ModrinthApiUrl("/search?" + params.join("&")));
        console.log(resp.data);
        current_total.value = resp.data.hits.length;
        current_hits.value = resp.data.hits;

        setTimeout(() => {
            in_loading.value = false;
        }, 3000);
    };

    const current_total = ref(0);
    const current_offset = ref(0);
    const current_hits = ref<Array<IModrinthProject>>([]);
    const in_loading = ref<boolean>(false);

    onMounted(() => {
        handleQuery(false);
    });
</script>

<template>
    <HintBar type="info">目前仅支持搜索和下载来自 Modrinth 的资源</HintBar>
    <div class="join w-full mt-2">
        <CompSearchInput v-model="search_input" class="join-item w-full" placeholder="搜索资源  ·  在输入框中按下 Enter 以进行搜索" @keyup.enter="handleQuery(false)" />
        <button class="join-item w-16 btn btn-primary" @click="handleQuery(false)">搜索</button>
    </div>
    <section class="w-full h-full grid grid-cols-[2fr_5fr] gap-x-2 mt-3">
        <FlowContainer class="ml-0 pb-21" v-if="show_filter">
            <Card title="游戏版本" class="mb-2 max-h-66" isSwapped>
                <section class="w-full h-full flex flex-col gap-2">
                    <FlowContainer class="h-full pr-0 translate-x-[2px]">
                        <section class="mb-0.25" v-for="version in version_filters">
                            <CompSearchFilterItem v-if="version.default || (!version.default && toggle_version)" :key="version.key" @click="handleVersionUpdate(version.key)">
                                {{ version.key }}
                            </CompSearchFilterItem>
                        </section>
                    </FlowContainer>
                    <label class="label mt-1 -mb-1">
                        <input type="checkbox" v-model="toggle_version" class="checkbox checkbox-sm scale-85 checkbox-success ml-1" />
                        <span class="translate-y-0.25 ml-1">显示全部版本</span>
                    </label>
                </section>
            </Card>
            <Card title="加载器" class="mb-2" v-if="['mod', 'modpack', 'shader'].includes($route.meta['comp_type'] as string)">
                <section class="mb-0.25" v-for="item in loader_filters">
                    <!-- @vue-ignore -->
                    <CompSearchFilterItem
                        v-if="item.default || (!item.default && toggle_mod_loader)"
                        :key="item.key"
                        excludeable
                        :selected="loader_filters[item.key] === true"
                        :selected-exclude="loader_filters[item.key] === false"
                        @update:select="(e) => handleLoaderUpdate('select', item.key, e)"
                        @update:exclude="(e) => handleLoaderUpdate('exclude', item.key, e)">
                        <div class="flex items-center gap-2">
                            <component :is="item.icon" class="size-4" />
                            <span class="translate-y-0.25">{{ item.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
                <CompSearchFilterToggle v-if="$route.meta['comp_type'] === 'mod'" :more="toggle_mod_loader" @click="toggle_mod_loader = !toggle_mod_loader" />
            </Card>
            <Card title="分类">
                <section class="mb-0.25" v-for="item in category_filters">
                    <!-- @vue-ignore -->
                    <CompSearchFilterItem
                        v-if="item.default || (!item.default && toggle_mod_loader)"
                        :key="item.key"
                        excludeable
                        :selected="loader_filters[item.key] === true"
                        :selected-exclude="loader_filters[item.key] === false"
                        @update:select="(e) => handleCategoryUpdate('select', item.key, e)"
                        @update:exclude="(e) => handleCategoryUpdate('exclude', item.key, e)">
                        <div class="flex items-center gap-2">
                            <component :is="item.icon" class="size-4" />
                            <span class="translate-y-0.25">{{ item.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
            </Card>
            <Card title="特性" v-if="['resourcepack', 'shader'].includes($route.meta['comp_type'] as string)" class="mt-2">
                <section class="mb-0.25" v-for="item in feature_filters">
                    <!-- @vue-ignore -->
                    <CompSearchFilterItem
                        v-if="item.default || (!item.default && toggle_mod_loader)"
                        :key="item.key"
                        excludeable
                        :selected="loader_filters[item.key] === true"
                        :selected-exclude="loader_filters[item.key] === false"
                        @update:select="(e) => handleFeatureUpdate('select', item.key, e)"
                        @update:exclude="(e) => handleFeatureUpdate('exclude', item.key, e)">
                        <div class="flex items-center gap-2">
                            <component :is="item.icon" class="size-4" />
                            <span class="translate-y-0.25">{{ item.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
            </Card>
            <Card title="分辨率" v-if="$route.meta['comp_type'] === 'resourcepack'" class="mt-2">
                <section class="mb-0.25" v-for="item in more_resolution_filters">
                    <!-- @vue-ignore -->
                    <CompSearchFilterItem
                        v-if="item.default || (!item.default && toggle_mod_loader)"
                        :key="item.key"
                        excludeable
                        :selected="loader_filters[item.key] === true"
                        :selected-exclude="loader_filters[item.key] === false"
                        @update:select="(e) => handleMoreUpdate('select', item.key, e)"
                        @update:exclude="(e) => handleMoreUpdate('exclude', item.key, e)">
                        <div class="flex items-center gap-2">
                            <span class="translate-y-0.25">{{ item.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
            </Card>
            <Card title="性能影响" v-if="$route.meta['comp_type'] === 'shader'" class="mt-2">
                <section class="mb-0.25" v-for="item in more_perf_filters">
                    <!-- @vue-ignore -->
                    <CompSearchFilterItem
                        v-if="item.default || (!item.default && toggle_mod_loader)"
                        :key="item.key"
                        excludeable
                        :selected="loader_filters[item.key] === true"
                        :selected-exclude="loader_filters[item.key] === false"
                        @update:select="(e) => handleMoreUpdate('select', item.key, e)"
                        @update:exclude="(e) => handleMoreUpdate('exclude', item.key, e)">
                        <div class="flex items-center gap-2">
                            <component :is="item.icon" class="size-4" />
                            <span class="translate-y-0.25">{{ item.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
            </Card>
        </FlowContainer>
        <FlowContainer class="pb-21.5" v-if="current_hits.length > 0">
            <LoadingAnimation
                v-if="in_loading"
                class="mx-auto mt-32"
                :message="`正在获取${CompTypeMapper[$route.meta['comp_type'] as string]}列表`"
                state="loading"
                :error="`获取${CompTypeMapper[$route.meta['comp_type'] as string]}列表时发生错误`" />
            <section
                v-else-if="current_hits.length === 0"
                class="w-fit bg-[color-mix(in_oklch,_var(--color-base-100)_90%,_var(--color-base-content)_10%)] flex flex-col justify-center items-center gap-4 py-4 px-6 rounded-lg transition-all duration-150 ease-in-out">
                <i class="icon-[fluent-color--cloud-dismiss-16] size-12"></i>
                这里啥都没有……
            </section>
            <CompItem
                v-if="!in_loading"
                v-for="item in current_hits"
                :key="item.slug"
                :title="item.title"
                :description="item.description"
                :slug="item.slug"
                :project_id="item.project_id"
                :downloads="item.downloads"
                :icon_url="item.icon_url" />
        </FlowContainer>
    </section>
</template>

<style lang="css" scoped>
    input[type="checkbox"]::before {
        transform: translateX(0.5px);
    }
</style>
