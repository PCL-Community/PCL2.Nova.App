<script setup lang="ts">
    import MySidebarButton from '../../components/button/MySidebarButton.vue'
    import { current_about } from '../../logic/changeBody'
    function clickSideButton(target: string) {
        current_about.value = target
    }
</script>
<template>
    <div style="display: flex; flex-direction: column;">
        <MySidebarButton :isChecked="current_about == 'Treasure-Box'" style="margin-top: 8px;" @click="clickSideButton('Treasure-Box')">
            <svg
                role="img"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
                fill="none" class="side-icon">
                <title id="coloursIconTitle">Colours</title>
                <circle cx="12" cy="9" r="5"/>
                <circle cx="9" cy="14" r="5"/>
                <circle cx="15" cy="14" r="5"/>
            </svg>
            <span style="margin-left: -10px;">百宝箱</span>
        </MySidebarButton>
        <MySidebarButton :isChecked="current_about == 'About'" @click="clickSideButton('About')">
            <svg
                role="img"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
                fill="none" class="side-icon">
                <path d="M12,12 L12,15"/>
                <line x1="12" y1="9" x2="12" y2="9"/>
                <circle cx="12" cy="12" r="10"/>
            </svg>
            <span style="margin-left: 20px;">关于与鸣谢</span>
        </MySidebarButton>
        <MySidebarButton :isChecked="current_about == 'Feedback'" @click="clickSideButton('Feedback')">
            <svg
                role="img"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
                fill="none" class="side-icon">
                <rect width="20" height="14" x="2" y="5"/>
                <path stroke-linecap="round" d="M2 5l10 9 10-9"/>
                <path stroke-linecap="round" d="M2 19l6.825-7.8"/>
                <path stroke-linecap="round" d="M22 19l-6.844-7.822"/>
            </svg>
            <span style="margin-left: -30px;">反馈</span>
        </MySidebarButton>
        <details class="grid"><summary>小游戏</summary></details>
        <div>
            <MySidebarButton :isChecked="current_about == 'Mine-Sweeper'" @click="clickSideButton('Mine-Sweeper')">
                <svg
                    role="img"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    stroke-width="1"
                    stroke-linecap="square"
                    stroke-linejoin="miter"
                    fill="none" class="side-icon">
                    <path d="M5 20v-5c1.333-1.333 3-2 5-2 3 0 4 2 6 2 1 0 2-.333 3-1V5c-1 .667-2 1-3 1-2 0-3-2-6-2-2 0-3.667.667-5 2v9"/>
                </svg>
                <span style="margin-left: -30px;">扫雷</span>
            </MySidebarButton>
            <MySidebarButton :isChecked="current_about == 'P2048'" @click="clickSideButton('P2048')">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    fill="none" class="side-icon">
                    <path d="M8 8a4 4 0 1 1 8 0c0 1.098-.564 2.025-1.159 2.815L8 20h8"/>
                </svg>
                <span style="margin-left: -30px;">2048</span>
            </MySidebarButton>
        </div>
    </div>
</template>
<style scoped>
    .grid {
        margin-top: 25px;
    }
    .grid summary {
        font-size: 12px;
        color: gray;
        margin-bottom: 7px;
        margin-left: 20px;
    }
    .grid + div {
        max-height: 0;
        transition: max-height 0.5s;
        margin: 0 0 1rem;
        overflow: hidden;
    }
    .grid[open] + div {
        max-height: 80px;
    }
</style>
<script lang="ts">
    export default {
        name: "aboutLeft"
    }
</script>