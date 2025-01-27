<script setup lang="ts">
    const props = defineProps<{
        label?: string;
        legend?: string;
        placeholder?: string;
    }>();

    const model = defineModel();

    const update = (event: Event) => {
        console.log((<HTMLInputElement>event.target).value);
        model.value = (<HTMLInputElement>event.target).value;
    };
</script>

<template>
    <fieldset class="fieldset w-full">
        <legend class="fieldset-legend ml-1" v-if="props.legend">{{ props.legend }}</legend>
        <input type="text" class="input input-sm w-full" :placeholder="props.placeholder" @input="update" />
        <p class="fieldset-label ml-1 text-error" v-if="props.label">{{ props.label }}</p>
    </fieldset>
</template>

<style lang="scss" scoped>
    input {
        outline: none;
    }
</style>
