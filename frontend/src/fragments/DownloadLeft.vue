<script setup lang="ts">
    import SideMenuButton from "@/components/SideMenuButton.vue";
    import SideMenuText from "@/components/SideMenuText.vue";
</script>

<template>
    <SideMenuText>Minecraft</SideMenuText>
    <SideMenuButton
        text="游戏下载"
        :selected="$route.path === '/download/client/auto'"
        @click="$router.push('/download/client/auto')">
        <i class="icon-[icon-park-outline--cube-three] size-5 rotate-180"></i>
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
        <i class="icon-[mingcute--plugin-2-line] size-5 ml-0.25" />
    </SideMenuButton>
    <SideMenuButton
        text="整合包"
        :selected="$route.path === '/download/comp/modpacks'"
        @click="$router.push('/download/comp/modpacks')">
        <i class="icon-[material-symbols--package-2-outline] size-5 scale-105" />
    </SideMenuButton>
    <SideMenuButton
        text="数据包"
        :selected="$route.path === '/download/comp/datapacks'"
        @click="$router.push('/download/comp/datapacks')">
        <i class="icon-[ic--baseline-data-object] size-5"></i>
    </SideMenuButton>
    <SideMenuButton
        text="资源包"
        :selected="$route.path === '/download/comp/resourcepacks'"
        @click="$router.push('/download/comp/resourcepacks')">
        <i class="icon-[icon-park-outline--texture-two] size-5 scale-90 -ml-[0.5px]" />
    </SideMenuButton>
    <SideMenuButton
        text="光影包"
        :selected="$route.path === '/download/comp/shaders'"
        @click="$router.push('/download/comp/shaders')">
        <i class="icon-[material-symbols--ev-shadow-outline] size-5" />
    </SideMenuButton>
    <br />
    <SideMenuText>其他</SideMenuText>
    <SideMenuButton
        text="收藏夹"
        :selected="$route.path === '/download/favorites'"
        @click="$router.push('/download/favorites')">
        <i class="icon-[material-symbols--bookmark-heart-outline-rounded] size-5" />
    </SideMenuButton>
</template>
