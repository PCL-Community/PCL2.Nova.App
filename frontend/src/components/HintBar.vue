<script setup lang="ts">
    defineProps<{
        type: "info" | "warning" | "error";
    }>();
</script>

<template>
    <span
        class="w-full h-fit rounded-[2px] border-l-4 px-2 py-1 flex items-center gap-1"
        :class="{
            'bg-info-content border-l-info text-info': $props.type === 'info',
            'bg-warning-content/50 border-l-warning text-warning': $props.type === 'warning',
            'bg-error-content border-l-error text-error': $props.type === 'error',
        }">
        <slot></slot>
    </span>
</template>
