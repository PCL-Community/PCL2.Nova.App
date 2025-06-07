<script setup lang="ts">
    import MenuButton from "@/components/MenuButton.vue";
    import SkinAvatar from "@/components/SkinAvatar.vue";
    import TextButton from "@/components/TextButton.vue";
</script>

<template>
    <section class="w-full h-[calc(100vh-calc(var(--spacing)*12))] flex">
        <aside class="w-1/3 h-full bg-white">
            <section class="w-full h-full flex flex-col justify-between items-center py-6">
                <section class="w-full flex justify-center items-center">
                    <MenuButton text="正版" selected reverse>
                        <i class="icon-[material-symbols--verified-user-outline] size-6"></i>
                    </MenuButton>
                    <MenuButton text="离线" reverse>
                        <i class="icon-[meteor-icons--unlink] size-5 scale-90"></i>
                    </MenuButton>
                </section>
                <section class="flex flex-col items-center gap-4">
                    <SkinAvatar skin="/12957577107354380989.png" :size="64" />
                    <span class="text-[oklch(25.33%_0.016_252.42)] text-xl">{{ "LingyunAwA" }}</span>
                </section>
                <section class="w-full flex flex-col items-center gap-2">
                    <button class="btn border-primary bg-transparent w-4/5 h-16 flex flex-col items-center hover:bg-primary/10">
                        <span class="font-medium text-primary text-lg leading-none mt-1">启动游戏</span>
                        <span class="font-medium text-base-100/50 text-sm leading-none -mt-0.5">1.20.1</span>
                    </button>
                    <section class="w-4/5 grid grid-cols-2 gap-x-4">
                        <TextButton>
                            <span class="font-medium">版本选择</span>
                        </TextButton>
                        <TextButton>
                            <span class="font-medium">版本设置</span>
                        </TextButton>
                    </section>
                </section>
            </section>
        </aside>
        <main class="w-2/3 h-full"></main>
    </section>
</template>

<style lang="css" scoped></style>
