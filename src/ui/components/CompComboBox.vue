<script setup lang="ts">
    import { ref } from "vue";

    const props = defineProps<{
        label?: string;
        legend?: string;
        options: string[];
        placeholder?: string;
    }>();

    const model = defineModel();
    const selectRef = ref<HTMLSelectElement>();

    const update = (event: Event) => {
        console.log((<HTMLSelectElement>event.target).value);
        model.value = (<HTMLSelectElement>event.target).value;
    };
</script>

<template>
    <!-- TODO: 支持输入 -->
    <fieldset class="fieldset relative">
        <select class="input input-sm w-full cursor-pointer" ref="selectRef" :placeholder="props.placeholder" @change="update">
            <option v-for="option in props.options" :key="option" :value="option">{{ option }}</option>
        </select>
    </fieldset>
</template>

<style lang="scss" scoped>
    select {
        outline: none;
    }
</style>
