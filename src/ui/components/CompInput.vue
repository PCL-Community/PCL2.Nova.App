<script setup lang="ts">
    const props = defineProps<{
        id?: string;
        label?: string;
        legend?: string;
        placeholder?: string;
    }>();

    const model = defineModel();
    const $emit = defineEmits();
</script>

<template>
    <fieldset class="fieldset w-full">
        <legend class="fieldset-legend ml-1" v-if="props.legend">{{ props.legend }}</legend>
        <input
            type="text"
            class="input input-sm w-full"
            v-model="model"
            :id="props.id"
            :placeholder="props.placeholder"
            @focusin="$emit('focusin')"
            @focusout="$emit('focusout')" />
        <p class="fieldset-label ml-1 text-error" v-if="props.label">{{ props.label }}</p>
    </fieldset>
</template>

<style lang="scss" scoped>
    input {
        outline: none;
    }
</style>
