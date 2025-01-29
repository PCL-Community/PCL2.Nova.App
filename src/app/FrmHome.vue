<script lang="ts" setup>
    import LayoutLeft from "../ui/layouts/home/LayoutLeft.vue";
</script>

<template>
    <section class="h-[calc(100vh-calc(var(--spacing)*14))] flex">
        <LayoutLeft class="fadein basis-1/2 relative" />
        <section class="basis-full relative">
            <RouterView />
        </section>
    </section>
</template>
