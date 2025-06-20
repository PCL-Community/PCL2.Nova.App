<script setup lang="ts">
    import SideMenu from "@/components/SideMenu.vue";
    import SideMenuButton from "@/components/SideMenuButton.vue";
</script>

<template>
    <section class="w-full h-[calc(100vh-calc(var(--spacing)*12))] flex">
        <SideMenu class="w-32!">
            <SideMenuButton
                text="帮助"
                :selected="$route.path === '/more/help'"
                @click="$router.push('/more/help')">
                <i class="icon-[material-symbols--help-outline-rounded] size-5" />
            </SideMenuButton>
            <SideMenuButton
                text="关注与鸣谢"
                :selected="$route.path === '/more/thanks'"
                @click="$router.push('/more/thanks')">
                <i class="icon-[material-symbols--info-outline-rounded] size-5" />
            </SideMenuButton>
            <SideMenuButton
                text="百宝箱"
                :selected="$route.path === '/more/tools'"
                @click="$router.push('/more/tools')">
                <i class="icon-[material-symbols--package-2-outline] size-5 scale-110" />
            </SideMenuButton>
        </SideMenu>
    </section>
</template>
