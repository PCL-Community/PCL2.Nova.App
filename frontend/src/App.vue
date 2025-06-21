<script setup lang="ts">
    import NavigatorBar from "./components/NavigatorBar.vue";
    import ThemeControler from "./controls/ThemeControler.vue";
</script>

<template>
    <ThemeControler />
    <div class="w-screen h-screen max-h-screen overflow-y-hidden bg-base-300">
        <NavigatorBar />
        <section class="flex w-full h-[calc(100vh-calc(var(--spacing)*12))] max-h-[calc(100vh-calc(var(--spacing)*12))]">
            <aside
                class="h-full bg-base-200 transition-[width] duration-250 ease-in-out overflow-hidden py-4"
                :style="{width: ($route.meta['left_width'] as string)}">
                <Transition name="slide" mode="out-in">
                    <section class="w-full h-full" :key="($route.meta['left_id'] as string)">
                        <component :is="$route.meta['left_comp']" />
                    </section>
                </Transition>
            </aside>
            <main class="w-full max-h-[calc(100vh-calc(var(--spacing)*12))]">
                <RouterView v-slot="{ Component }">
                    <Transition name="fade" mode="out-in">
                        <component :is="Component" />
                    </Transition>
                </RouterView>
            </main>
        </section>
    </div>
</template>

<style lang="css" scoped>
    .fade-enter-active,
    .fade-leave-active {
        transition: opacity 0.1s ease;
    }

    .fade-enter-from,
    .fade-leave-to {
        opacity: 0;
    }

    .slide-enter-active,
    .slide-leave-active {
        transition: transform 0.1s ease;
        transition-delay: 25ms;
    }

    .slide-enter-from,
    .slide-leave-to {
        transform: translateX(-100%);
    }
</style>

