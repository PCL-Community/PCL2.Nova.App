<script setup lang="ts">
    const props = defineProps<{
        label?: string;
        legend?: string;
        options: string[];
        placeholder?: string;
    }>();

    const model = defineModel();

    const update = (event: Event) => {
        console.log((<HTMLSelectElement>event.target).value);
        model.value = (<HTMLSelectElement>event.target).value;
    };
</script>

<template>
    <fieldset class="fieldset w-full">
        <legend class="fieldset-legend ml-1" v-if="props.legend">{{ props.legend }}</legend>
        <select class="input input-sm w-full" :placeholder="props.placeholder" @change="update">
            <option v-for="option in props.options" :key="option" :value="option">{{ option }}</option>
        </select>
        <p class="fieldset-label ml-1 text-error" v-if="props.label">{{ props.label }}</p>
    </fieldset>
</template>

<style lang="scss" scoped>
    select {
        outline: none;
    }
</style>