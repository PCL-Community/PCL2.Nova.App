<script setup lang="ts">
    import { ref } from "vue";
    import CompCard from "../../components/CompCard.vue";
    import CompInput from "../../components/CompInput.vue";
    import CompRadioButton from "../../components/CompRadioButton.vue";
    import CompButton from "../../components/CompButton.vue";
    import CompComboBox from "../../components/CompComboBox.vue";
    import { versionFilter } from "../../../modules/ModUtils";

    type TLoader = "optifine" | "oculus" | "iris";

    const refreshing = ref(false);
    const search = ref<string>("");
    const version = ref<string>("");
    const loader = ref<TLoader[]>(["optifine", "oculus", "iris"]);

    const setLoaderAll = () => {
        loader.value = ["optifine", "oculus", "iris"];
    };

    const toggleLoader = (value: TLoader) => {
        if (loader.value.length === 3) loader.value = [];
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
                        <span>光影包名</span>
                        <CompInput v-model="search" />
                    </ul>
                    <ul class="grid grid-cols-[4em_auto_64px_106px_106px_84px] items-center gap-4">
                        <span>版本</span>
                        <CompComboBox v-model="version" :options="versionFilter" />
                        <CompRadioButton text="全部" :checked="loader.length === 3" highlight noicon @click="setLoaderAll" />
                        <CompRadioButton
                            text="OptiFine"
                            :checked="loader.length < 3 && loader.includes('optifine')"
                            highlight
                            @click="toggleLoader('optifine')">
                            <img src="/Images/Icons/OptiFine.png" class="w-4 translate-y-[.5px]" />
                        </CompRadioButton>
                        <CompRadioButton
                            text="Oculus"
                            :checked="loader.length < 3 && loader.includes('oculus')"
                            highlight
                            @click="toggleLoader('oculus')">
                            <img src="/Images/Icons/Oculus.png" class="w-6" />
                        </CompRadioButton>
                        <CompRadioButton
                            text="Iris"
                            :checked="loader.length < 3 && loader.includes('iris')"
                            highlight
                            @click="toggleLoader('iris')">
                            <img src="/Images/Icons/Iris.png" class="w-4" />
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
