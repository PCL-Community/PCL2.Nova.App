<script setup lang="ts">
    defineProps<{
        title?: string;
        isSwapped?: boolean;
    }>();
</script>

<template>
    <div
        class="collapse collapse-arrow bg-[color-mix(in_oklch,_var(--color-base-100)_90%,_var(--color-base-content)_10%)] border border-base-200 group shadow-[0_0_4px_transparent] hover:shadow-primary transition-shadow ease-in-out duration-150">
        <input type="checkbox" class="h-12" :checked="!isSwapped" />
        <div
            class="collapse-title font-semibold h-12 -mt-1 group-hover:text-secondary transition-colors ease-in-out duration-150">
            {{ $props.title }}
        </div>
        <div class="collapse-content text-sm">
            <slot></slot>
        </div>
    </div>
</template>

<style lang="css" scoped>
    .collapse > .collapse-title::after {
        margin-top: calc(var(--spacing) * -0.5) !important;
    }
</style>
