<script setup lang="ts">
    import Card from "@/components/Card.vue";
    import FlowContainer from "@/components/FlowContainer.vue";
    import MenuButton from "@/components/MenuButton.vue";
    import SkinAvatar from "@/components/SkinAvatar.vue";
    import TextButton from "@/components/TextButton.vue";
</script>

<template>
    <section class="w-full h-[calc(100vh-calc(var(--spacing)*12))] flex">
        <aside class="w-1/3 h-full bg-base-200">
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
                    <SkinAvatar class="size-16" skin="/12957577107354380989.png" :size="64" />
                    <span class="text-base-content text-xl">{{ "LingyunAwA" }}</span>
                </section>
                <section class="w-full flex flex-col items-center gap-2">
                    <button
                        class="btn border-primary bg-transparent w-4/5 h-16 flex flex-col items-center hover:bg-primary/10">
                        <span class="font-medium text-primary text-lg leading-none mt-1">启动游戏</span>
                        <span class="font-medium text-base-content/50 text-sm leading-none -mt-0.5">1.20.1</span>
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
        <main class="w-2/3 h-full py-2">
            <FlowContainer class="px-4 py-2">
                <Card title="测试卡片">
                    <label class="swap swap-rotate">
                        <!-- this hidden checkbox controls the state -->
                        <input type="checkbox" class="theme-controller" value="novalight" />

                        <!-- sun icon -->
                        <svg
                            class="swap-on h-10 w-10 fill-current"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24">
                            <path
                                d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" />
                        </svg>

                        <!-- moon icon -->
                        <svg
                            class="swap-off h-10 w-10 fill-current"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24">
                            <path
                                d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" />
                        </svg>
                    </label>
                </Card>
            </FlowContainer>
        </main>
    </section>
</template>

<style lang="css" scoped></style>
