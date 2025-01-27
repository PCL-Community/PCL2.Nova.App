<script lang="ts" setup>
    const props = defineProps<{
        highlight?: boolean;
        red?: boolean;
    }>();
</script>

<template>
    <button
        class="btn btn-outline hover:bg-[var(--color-btn-hover)]"
        :class="{
            'border-base-content': !(props.highlight && props.red),
            'border-info': props.highlight,
            'text-info': props.highlight,
            'border-error': props.red,
            'text-error': props.red,
        }">
        <slot></slot>
    </button>
</template>
