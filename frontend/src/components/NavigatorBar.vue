<script setup lang="ts">
    import { Quit, WindowMinimise } from "@/modules/wailsjs/runtime/runtime";
    import MenuButton from "./MenuButton.vue";
    import IconButton from "./IconButton.vue";
</script>

<template>
    <nav
        class="pl-5 pr-2 w-full h-12 bg-gradient-to-r from-primary via-secondary to-primary relative"
        style="--wails-draggable: drag">
        <Transition name="fade" mode="out-in">
            <section class="flex justify-between" v-if="$route.meta['navbar_mode'] === 'normal'">
                <section class="absolute top-1/2 left-1/2 -translate-1/2 flex">
                    <MenuButton
                        text="启动"
                        style="--wails-draggable: no-drag"
                        :selected="$route.fullPath === '/'"
                        @click="$router.push('/')">
                        <i class="icon-[material-symbols--play-arrow-outline-rounded] size-8 -ml-2"></i>
                    </MenuButton>
                    <MenuButton
                        text="下载"
                        style="--wails-draggable: no-drag"
                        :selected="$route.fullPath.startsWith('/download')"
                        @click="$router.push('/download')">
                        <i class="icon-[material-symbols--download-rounded] size-7 -ml-1"></i>
                    </MenuButton>
                    <MenuButton
                        text="设置"
                        style="--wails-draggable: no-drag"
                        :selected="$route.fullPath.startsWith('/settings')"
                        @click="$router.push('/settings')">
                        <i class="icon-[material-symbols--settings] size-6 -ml-1 mr-1"></i>
                    </MenuButton>
                    <MenuButton
                        text="更多"
                        style="--wails-draggable: no-drag"
                        :selected="$route.fullPath.startsWith('/more')"
                        @click="$router.push('/more')">
                        <i class="icon-[material-symbols--grid-view-outline-rounded] size-6 -ml-1 mr-1"></i>
                    </MenuButton>
                </section>
                <img
                    src="/nova.svg"
                    class="size-12"
                    :class="{ 'transform scale-x-[-1]': Math.floor(Math.random() * 10000) + 1 === 2333 }" />
                <section class="flex items-center">
                    <IconButton rounded flex-center @click="WindowMinimise" style="--wails-draggable: no-drag">
                        <i class="icon-[ic--round-minus] size-8 bg-white"></i>
                    </IconButton>
                    <IconButton rounded flex-center class="-ml-1" @click="Quit" style="--wails-draggable: no-drag">
                        <i class="icon-[material-symbols--close] size-8 bg-white"></i>
                    </IconButton>
                </section>
            </section>
        </Transition>
    </nav>
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
