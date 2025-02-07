<script setup lang="ts">
    import { ref } from "vue";
    import CompCard from "../../components/CompCard.vue";
    import CompInput from "../../components/CompInput.vue";
    import CompRadioButton from "../../components/CompRadioButton.vue";
    import CompButton from "../../components/CompButton.vue";
    import { versionFilter } from "../../../modules/ModUtils";
import CompComboBox from "../../components/CompComboBox.vue";

    type TLoader = "forge" | "fabric" | "quilt" | "neoforge";

    const refreshing = ref(false);
    const search = ref<string>("");
    const version = ref<string>("");
    const loader = ref<TLoader[]>(["forge", "fabric", "quilt", "neoforge"]);

    const setLoaderAll = () => {
        loader.value = ["forge", "fabric", "quilt", "neoforge"];
    };
    const toggleLoader = (value: TLoader) => {
        if (loader.value.length === 4) loader.value = [];
        if (loader.value.includes(value)) {
            loader.value = loader.value.filter((item) => item !== value);
        } else {
            loader.value.push(value);
        }
        if (loader.value.length === 0) setLoaderAll();
    };
</script>

<template>
    <section>
        <header>
            <CompCard title="搜索">
                <div class="px-6 flex flex-col gap-2" v-if="!refreshing">
                    <ul class="grid grid-cols-[4em_4fr] items-center gap-4">
                        <span>Mod 名</span>
                        <CompInput v-model="search" />
                    </ul>
                    <ul class="grid grid-cols-[4em_auto_64px_96px_128px_96px_96px] items-center gap-4">
                        <span>版本</span>
                        <CompComboBox v-model="version" :options="versionFilter" />
                        <CompRadioButton text="全部" :checked="loader.length === 4" highlight noicon @click="setLoaderAll" />
                        <CompRadioButton
                            text="Forge"
                            :checked="loader.length < 4 && loader.includes('forge')"
                            highlight
                            @click="toggleLoader('forge')">
                            <img src="/Images/Icons/Forge.png" class="w-7" />
                        </CompRadioButton>
                        <CompRadioButton
                            text="NeoForge"
                            :checked="loader.length < 4 && loader.includes('neoforge')"
                            highlight
                            @click="toggleLoader('neoforge')">
                            <img src="/Images/Icons/NeoForge.png" class="w-6" />
                        </CompRadioButton>
                        <CompRadioButton
                            text="Fabric"
                            :checked="loader.length < 4 && loader.includes('fabric')"
                            highlight
                            @click="toggleLoader('fabric')">
                            <img src="/Images/Icons/Fabric.png" class="w-9" />
                        </CompRadioButton>
                        <CompRadioButton
                            text="Quilt"
                            :checked="loader.length < 4 && loader.includes('quilt')"
                            highlight
                            @click="toggleLoader('quilt')">
                            <img src="/Images/Icons/Quilt.png" class="w-4 translate-y-[1px]" />
                        </CompRadioButton>
                    </ul>
                    <ul class="w-3/7 grid grid-cols-[1fr_1fr] gap-4">
                        <CompButton highlight>搜索</CompButton>
                        <CompButton>重置</CompButton>
                    </ul>
                </div>
            </CompCard>
        </header>
    </section>
</template>
