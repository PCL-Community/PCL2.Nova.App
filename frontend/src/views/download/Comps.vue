<script setup lang="ts">
    import { computed, nextTick, onMounted, ref } from "vue";
    import { useRoute } from "vue-router";
    import { HintBar, CompSearchInput, FlowContainer, Card, CompSearchFilterItem, CompSearchFilterToggle } from "@/components";
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
    } from "@/types/SearchOptions";
    import { BidingLoaderInst, BindingCategoryInst, BindingVersionInst, OptionCategoryInst, OptionLoaderInst, OptionVersionInst, handleUpdateInst } from "@/modules/comps";

    // Global
    const route = useRoute();

    // Binding
    const search_input = ref("");
    const search_version = BindingVersionInst.Default;
    // Loader 绑定
    const search_mod_loader = BidingLoaderInst.Mod;
    const search_modpack_loader = BidingLoaderInst.Modpack;
    const search_shader_loader = BidingLoaderInst.Shader;
    // Category 绑定
    const search_mod_category = BindingCategoryInst.Mod;
    const search_resourcespack_category = BindingCategoryInst.Resourcespack;
    const search_datapack_category = BindingCategoryInst.Datapack;
    const search_shader_category = BindingCategoryInst.Shader;
    const search_modpack_category = BindingCategoryInst.Modpack;

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

    // Filter Toggling More
    // Input 没有啥可展开的，所以这里也不需要
    const toggle_version = ref<boolean>(false);
    const toggle_mod_loader = ref<boolean>(false);

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

    // Computed Properties
    const search_loader = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mods":
            default:
                return loader_mod_filters;
            case "modpacks":
                return loader_modpack_filters;
            case "shaders":
                return loader_shader_filters;
        }
    });
    const search_category = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mods":
            default:
                return category_mod_filters;
            case "modpacks":
                return category_modpack_filters;
            case "shaders":
                return category_shader_filters;
            case "resourcepacks":
                return category_resourcespack_filters;
            case "datapacks":
                return category_datapack_filters;
        }
    });
    // Computed Handlers
    const handleLoaderUpdate = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mods":
            default:
                return handleModLoaderUpdate;
            case "modpacks":
                return handleModpackLoaderUpdate;
            case "shaders":
                return handleShaderLoaderUpdate;
        }
    });
    const handleCategoryUpdate = computed(() => {
        switch (route.meta["comp_type"]) {
            case "mods":
            default:
                return handleModCategoryUpdate;
            case "modpacks":
                return handleModpackCategoryUpdate;
            case "shaders":
                return handleShaderCategoryUpdate;
            case "resourcepacks":
                return handleResourcespackCategoryUpdate;
            case "datapacks":
                return handleDatapackCategoryUpdate;
        }
    });

    // (perf) defer loading
    const show_filter = ref<boolean>(false);
    onMounted(() => {
        nextTick(() => {
            show_filter.value = true;
        });
    });
</script>

<template>
    <HintBar type="info">目前仅支持搜索和下载来自 Modrinth 的资源</HintBar>
    <CompSearchInput v-model="search_input" class="w-full mt-2" placeholder="搜索资源  ·  在输入框中按下 Enter 以进行搜索" />
    <section class="w-full h-full grid grid-cols-[2fr_5fr] -m-1.25 mt-2">
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
            <Card title="加载器" class="mb-2" v-if="['mods', 'modpacks', 'shaders'].includes($route.meta['comp_type'] as string)">
                <section class="mb-0.25" v-for="loader in search_loader">
                    <!-- @vue-ignore -->
                    <CompSearchFilterItem
                        v-if="loader.default || (!loader.default && toggle_mod_loader)"
                        :key="loader.key"
                        excludeable
                        :selected="search_loader[loader.key] === true"
                        :selected-exclude="search_loader[loader.key] === false"
                        @update:select="(e) => handleLoaderUpdate('select', loader.key, e)"
                        @update:exclude="(e) => handleLoaderUpdate('exclude', loader.key, e)">
                        <div class="flex items-center gap-2">
                            <component :is="loader.icon" class="size-4" />
                            <span class="translate-y-0.25">{{ loader.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
                <CompSearchFilterToggle v-if="$route.name === 'download-mods'" :more="toggle_mod_loader" @click="toggle_mod_loader = !toggle_mod_loader" />
            </Card>
            <Card title="分类">
                <section class="mb-0.25" v-for="loader in search_category">
                    <!-- @vue-ignore -->
                    <CompSearchFilterItem
                        v-if="loader.default || (!loader.default && toggle_mod_loader)"
                        :key="loader.key"
                        excludeable
                        :selected="search_loader[loader.key] === true"
                        :selected-exclude="search_loader[loader.key] === false"
                        @update:select="(e) => handleCategoryUpdate('select', loader.key, e)"
                        @update:exclude="(e) => handleCategoryUpdate('exclude', loader.key, e)">
                        <div class="flex items-center gap-2">
                            <component :is="loader.icon" class="size-4" />
                            <span class="translate-y-0.25">{{ loader.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
            </Card>
        </FlowContainer>
        <FlowContainer></FlowContainer>
    </section>
</template>

<style lang="css" scoped>
    input[type="checkbox"]::before {
        transform: translateX(0.5px);
    }
</style>
