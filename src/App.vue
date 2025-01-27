<script lang="ts" setup>
    import LayoutHeader from "./ui/layouts/LayoutHeader.vue";
</script>

<template>
    <LayoutHeader />
    <RouterView />
</template>

<style lang="scss">
@font-face {
    font-family: PCL;
    src: url('/font.ttf');
}
body {
    font-family: PCL
}
</style>