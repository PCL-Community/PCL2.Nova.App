<script lang="ts" setup>
    import LayoutLeft from "../ui/layouts/home/LayoutLeft.vue";
</script>

<template>
    <section class="h-[calc(100vh-calc(var(--spacing)*14))] flex">
        <LayoutLeft class="basis-1/3" />
        <section class="basis-2/3 relative">
            <RouterView />
        </section>
    </section>
</template>

<style lang="scss" scoped></style>
