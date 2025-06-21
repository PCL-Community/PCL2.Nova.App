<script setup lang="ts">
    import { ref } from "vue";
    import {
        HintBar,
        CompSearchInput,
        FlowContainer,
        Card,
        CompSearchFilterItem,
        CompSearchFilterToggle,
    } from "@/components";
    import {
        ModLoaderForge,
        ModLoaderFabric,
        ModLoaderNeoForge,
        ModLoaderQuilt,
        ModLoaderBabric,
        ModLoaderBTA,
        ModLoaderJavaAgent,
        ModLoaderLegacyFabric,
        ModLoaderLiteLoader,
        ModLoaderNilLoader,
        ModLoaderOrnithe,
        ModLoaderRML,
        ModLoaderRift,
    } from "@/icons";
    import { search_version_inst, version_filters_inst } from "@/modules/options/CompFilters";

    // TS Types
    type ISelectSearchOption = boolean | null;
    type ILoaderKey =
        | "forge"
        | "neoforge"
        | "fabric"
        | "quilt"
        | "babric"
        | "bta"
        | "javaagent"
        | "legacyfabric"
        | "liteloader"
        | "rml"
        | "nilloader"
        | "ornithe"
        | "rift";

    // Query Binding
    const search_string = ref("");
    const search_version = search_version_inst;
    const search_loader = ref<Record<ILoaderKey, ISelectSearchOption>>({
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

    // Filter Toggling More
    const toggle_version = ref<boolean>(false);
    const toggle_loader = ref<boolean>(false);

    // Filter Options
    const version_filters = version_filters_inst;
    const loader_filters: { key: ILoaderKey; name: string; icon: any; default?: boolean }[] = [
        // Default Show
        { key: "forge", name: "Forge", icon: ModLoaderForge, default: true },
        { key: "neoforge", name: "NeoForge", icon: ModLoaderNeoForge, default: true },
        { key: "fabric", name: "Fabric", icon: ModLoaderFabric, default: true },
        { key: "quilt", name: "Quilt", icon: ModLoaderQuilt, default: true },
        // More
        { key: "babric", name: "Babric", icon: ModLoaderBabric },
        { key: "bta", name: "BTA (Babric)", icon: ModLoaderBTA },
        { key: "javaagent", name: "Java Agent", icon: ModLoaderJavaAgent },
        { key: "legacyfabric", name: "Legacy Fabric", icon: ModLoaderLegacyFabric },
        { key: "liteloader", name: "LiteLoader", icon: ModLoaderLiteLoader },
        { key: "rml", name: "Risugami's ML", icon: ModLoaderRML },
        { key: "nilloader", name: "NilLoader", icon: ModLoaderNilLoader },
        { key: "ornithe", name: "Ornithe", icon: ModLoaderOrnithe },
        { key: "rift", name: "Rift", icon: ModLoaderRift },
    ];

    // Handler
    const handleOptionUpdate = (type: "select" | "exclude", key: ILoaderKey, value: boolean) => {
        switch (type) {
            case "select":
                search_loader.value[key] = value ? true : null;
                break;
            case "exclude":
                search_loader.value[key] = value ? false : null;
                break;
        }
    };
</script>

<template>
    <HintBar type="info">目前仅支持搜索和下载来自 Modrinth 的资源</HintBar>
    <CompSearchInput
        v-model="search_string"
        class="w-full mt-2"
        placeholder="搜索资源  ·  在输入框中按下 Enter 以进行搜索" />
    <section class="w-full h-full grid grid-cols-[2fr_5fr] -m-1.25 mt-2">
        <FlowContainer class="ml-0 pb-21">
            <Card title="游戏版本" class="mb-2 max-h-66">
                <section class="w-full h-full flex flex-col gap-2">
                    <FlowContainer class="h-full">
                        <section class="mb-0.25" v-for="version in version_filters">
                            <CompSearchFilterItem
                                v-if="version.default || (!version.default && toggle_version)"
                                :key="version.key">
                                {{ version.key }}
                            </CompSearchFilterItem>
                        </section>
                    </FlowContainer>
                    <label class="label mt-1 -mb-1">
                        <input
                            type="checkbox"
                            v-model="toggle_version"
                            class="checkbox checkbox-sm scale-85 checkbox-success ml-1" />
                        <span class="translate-y-0.25 ml-1">显示全部版本</span>
                    </label>
                </section>
            </Card>
            <Card title="加载器" class="mb-2">
                <section class="mb-0.25" v-for="loader in loader_filters">
                    <CompSearchFilterItem
                        v-if="loader.default || (!loader.default && toggle_loader)"
                        :key="loader.key"
                        excludeable
                        :selected="search_loader[loader.key] === true"
                        :selected-exclude="search_loader[loader.key] === false"
                        @update:select="(e) => handleOptionUpdate('select', loader.key, e)"
                        @update:exclude="(e) => handleOptionUpdate('exclude', loader.key, e)">
                        <div class="flex items-center gap-2">
                            <component :is="loader.icon" class="size-4" />
                            <span class="translate-y-0.25">{{ loader.name }}</span>
                        </div>
                    </CompSearchFilterItem>
                </section>
                <CompSearchFilterToggle :more="toggle_loader" @click="toggle_loader = !toggle_loader" />
            </Card>
            <Card title="分类"></Card>
        </FlowContainer>
        <FlowContainer></FlowContainer>
    </section>
</template>

<style lang="css" scoped>
    input[type="checkbox"]::before {
        transform: translateX(0.5px);
    }
</style>
