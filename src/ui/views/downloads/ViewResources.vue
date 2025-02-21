<script setup lang="ts">
    import { ref } from "vue";
    import CompCard from "../../components/CompCard.vue";
    import CompInput from "../../components/CompInput.vue";
    import CompButton from "../../components/CompButton.vue";
    import CompComboBox from "../../components/CompComboBox.vue";
    import { versionFilter } from "../../../modules/ModUtils";

    const refreshing = ref(false);
    const search = ref<string>("");
    const version = ref<string>("");
</script>

<template>
    <section>
        <header>
            <CompCard title="搜索">
                <div class="px-6 flex flex-col gap-2" v-if="!refreshing">
                    <ul class="grid grid-cols-[4em_3fr_2em_1fr] items-center gap-4">
                        <span>资源包名</span>
                        <CompInput v-model="search" />
                        <span>版本</span>
                        <CompComboBox v-model="version" :options="versionFilter" />
                    </ul>
                    <ul class="mt-1 w-3/7 grid grid-cols-[1fr_1fr] gap-4">
                        <CompButton highlight>搜索</CompButton>
                        <CompButton>重置</CompButton>
                    </ul>
                </div>
            </CompCard>
        </header>
    </section>
</template>
