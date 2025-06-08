<script setup lang="ts">
    import NavigatorBar from "./components/NavigatorBar.vue";
    import ThemeControler from "./controls/ThemeControler.vue";
</script>

<template>
    <ThemeControler />
    <div class="w-screen h-screen bg-base-300">
        <NavigatorBar />
        <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
                <component :is="Component" />
            </Transition>
        </RouterView>
    </div>
</template>

<style lang="css" scoped>
    .fade-enter-active,
    .fade-leave-active {
        transition: opacity 0.3s ease;
    }

    .fade-enter-from,
    .fade-leave-to {
        opacity: 0;
    }
</style>

