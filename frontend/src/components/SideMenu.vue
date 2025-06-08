<script setup lang="ts">
    import FlowContainer from "./FlowContainer.vue";
</script>

<template>
    <FlowContainer>
        <aside class="h-full flex flex-col bg-base-100 py-4" style="--noise: 1">
            <slot></slot>
        </aside>
    </FlowContainer>
</template>
