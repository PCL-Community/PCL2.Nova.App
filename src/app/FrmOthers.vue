<script setup lang="ts">
    import LayoutLeft from "../ui/layouts/others/LayoutLeft.vue";
</script>

<template>
    <section class="h-[calc(100vh-calc(var(--spacing)*14))] flex">
        <LayoutLeft class="basis-44" />
        <section class="subrouter basis-full w-full h-full p-4 overflow-y-auto">
            <RouterView />
        </section>
    </section>
</template>
