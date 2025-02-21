<script lang="ts" setup>
    const props = defineProps<{
        text: string;
        checked: boolean;
        noicon?: boolean;
        highlight?: boolean;
    }>();
</script>

<template>
    <div
        :class="{
            'btn-ghost': !props.checked,
            'hover:bg-black/20': !props.checked && !props.highlight,
            'hover:bg-primary/40': !props.checked && props.highlight,
            'btn-primary': props.checked && props.highlight,
            'text-[var(--color-radiobtn-text)]': (!props.checked && !props.highlight) || (props.checked && props.highlight),
            'text-[var(--color-radiobtn-atext)]': (!props.checked && props.highlight) || (props.checked && !props.highlight),
        }"
        class="btn h-auto py-1 rounded-full"
        role="button">
        <section class="w-5 h-5 flex justify-center items-center" v-if="!props.noicon">
            <slot></slot>
        </section>
        <span class="translate-x-[.5px] leading-4.5">{{ props.text }}</span>
    </div>
</template>
