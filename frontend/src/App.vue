<script setup lang="ts">
    import NavigatorBar from "./components/NavigatorBar.vue";
</script>

<template>
    <div class="w-screen h-screen">
        <NavigatorBar />
        <RouterView />
    </div>
</template>

