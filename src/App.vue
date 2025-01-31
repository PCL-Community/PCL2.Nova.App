<script lang="ts" setup>
    import { onMounted } from "vue";
    import LayoutHeader from "./ui/layouts/LayoutHeader.vue";

    onMounted(() => {
        const welcome = new Audio("/Popup.SAO.Welcome.wav");
        welcome.volume = 0.35;
        welcome.play();
    });
</script>

<template>
    <LayoutHeader />
    <main>
        <RouterView />
    </main>
</template>
