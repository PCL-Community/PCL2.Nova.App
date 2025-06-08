<script setup lang="ts">
    import { useRoute } from "vue-router";
    import SideMenu from "@/components/SideMenu.vue";
    import SideMenuButton from "@/components/SideMenuButton.vue";
    import SideMenuText from "@/components/SideMenuText.vue";
</script>

<template>
    <section class="w-full h-[calc(100vh-calc(var(--spacing)*12))] flex">
        <SideMenu class="w-36!">
            <SideMenuText>Minecraft</SideMenuText>
            <SideMenuButton
                text="游戏下载"
                :selected="$route.path === '/download/client/auto'"
                @click="$router.push('/download/client/auto')">
                <i class="icon-[icon-park-outline--cube-three] size-5"></i>
            </SideMenuButton>
            <SideMenuButton
                text="手动安装包"
                :selected="$route.path === '/download/client/manual'"
                @click="$router.push('/download/client/manual')">
                <i class="icon-[icon-park-outline--components] size-5 scale-90"></i>
            </SideMenuButton>
            <br />
            <SideMenuText>社区资源</SideMenuText>
            <SideMenuButton
                text="Mod"
                :selected="$route.path === '/download/comp/mods'"
                @click="$router.push('/download/comp/mods')">
                <svg viewBox="0 0 1024 1024" class="size-5">
                    <path
                        fill="currentColor "
                        d="M789.504 912.896h-195.072a35.328 35.328 0 0 1-34.816-35.328v-77.312a39.936 39.936 0 0 0-40.448-39.936H481.28a40.448 40.448 0 0 0-40.448 39.936v77.312a34.816 34.816 0 0 1-34.816 35.328H204.8a41.984 41.984 0 0 1-40.448-42.496v-200.192a35.328 35.328 0 0 1 35.328-35.328h72.704a39.936 39.936 0 0 0 39.936-39.936v-37.888a39.936 39.936 0 0 0-39.936-40.448H199.68a35.328 35.328 0 0 1-35.328-35.328V287.744A41.984 41.984 0 0 1 204.8 245.76h176.64v-32.768a102.4 102.4 0 0 1 102.4-102.4h33.792a102.4 102.4 0 0 1 102.4 102.4v32.768h170.496a41.984 41.984 0 0 1 41.984 41.984V460.8h28.672a102.4 102.4 0 0 1 102.4 102.4v33.792a102.4 102.4 0 0 1-102.4 102.4h-28.672V870.4a41.984 41.984 0 0 1-43.008 42.496z m-159.744-70.144h131.584V665.6a34.304 34.304 0 0 1 34.816-34.816h64a31.744 31.744 0 0 0 31.744-31.744V563.2a31.744 31.744 0 0 0-31.744-31.744h-64a34.816 34.816 0 0 1-34.816-35.328V316.416h-177.152a35.328 35.328 0 0 1-35.328-35.328V212.992a31.744 31.744 0 0 0-31.744-31.744h-33.792a31.744 31.744 0 0 0-31.744 31.744v68.096a35.328 35.328 0 0 1-34.816 35.328H234.496v130.048h37.888a110.592 110.592 0 0 1 110.08 110.592v37.888a110.592 110.592 0 0 1-110.08 110.592h-37.888v137.216h136.192v-42.496a110.592 110.592 0 0 1 110.592-110.08h37.888a110.592 110.592 0 0 1 110.592 110.08z"></path>
                </svg>
            </SideMenuButton>
        </SideMenu>
    </section>
</template>
