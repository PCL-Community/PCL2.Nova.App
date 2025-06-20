<script setup lang="ts">
    import SideMenuButton from "@/components/SideMenuButton.vue";
    import { BrowserOpenURL } from "@/modules/wailsjs/runtime/runtime";
</script>

<template>
    <SideMenuButton text="帮助" :selected="$route.path === '/more/help'" @click="$router.push('/more/help')">
        <i class="icon-[material-symbols--help-outline-rounded] size-5" />
    </SideMenuButton>
    <SideMenuButton
        text="关注与鸣谢"
        :selected="$route.path === '/more/credits'"
        @click="$router.push('/more/credits')">
        <i class="icon-[material-symbols--info-outline-rounded] size-5" />
    </SideMenuButton>
    <SideMenuButton text="百宝箱" :selected="$route.path === '/more/tools'" @click="$router.push('/more/tools')">
        <i class="icon-[material-symbols--package-2-outline] size-5 scale-110" />
    </SideMenuButton>
    <SideMenuButton text="反馈" @click="BrowserOpenURL('https://github.com/PCL-Community/PCL2.Nova.App/issues')">
        <i class="icon-[octicon--issue-tracks-16] size-5 scale-85" />
    </SideMenuButton>
    <SideMenuButton
        text="新功能投票"
        @click="
            BrowserOpenURL(
                'https://github.com/PCL-Community/PCL2.Nova.App/discussions/categories/%E5%8A%9F%E8%83%BD%E6%8A%95%E7%A5%A8'
            )
        ">
        <i class="icon-[material-symbols--how-to-vote-outline-rounded] size-5" />
    </SideMenuButton>
</template>
