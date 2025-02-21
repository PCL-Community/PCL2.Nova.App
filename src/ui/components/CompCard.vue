<script setup lang="ts">
    const props = defineProps<{
        title: string;
        canSwap?: boolean;
    }>();
</script>

<template>
    <div class="collapse bg-base-300 rounded-lg" :class="{ 'collapse-arrow': props.canSwap, 'collapse-open': !props.canSwap }">
        <input type="checkbox" class="cb py-0" />
        <div class="collapse-title font-semibold">{{ props.title }}</div>
        <div class="collapse-content">
            <slot></slot>
        </div>
    </div>
</template>

<style lang="scss" scoped>
    .cb {
        padding: 0;
        min-height: 2.7rem;
    }
    .collapse-title {
        padding: 0;
        padding-top: calc(0.5rem + 3px);
        padding-left: 1rem;
        min-height: 2.7rem;

        &::after {
            top: calc(1.9rem - 8px);
        }
    }
</style>
