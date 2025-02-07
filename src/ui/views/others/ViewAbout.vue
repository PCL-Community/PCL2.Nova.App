<script setup lang="ts">
    import CompButton from "../../components/CompButton.vue";
    import CompCard from "../../components/CompCard.vue";
    import CompListItem from "../../components/CompListItem.vue";
    import metadata from "../../../metadata.json";

    const $jsWindow = window;

    const { channel, version } = metadata;
</script>

<template>
    <main class="flex flex-col gap-4">
        <CompCard title="关于">
            <section class="w-full px-4 flex flex-col gap-2">
                <div class="flex">
                    <CompListItem text="凌云" desp="Plain Craft Launcher II: Nova 的作者！" imgSrc="/Images/Heads/lingyun.png" />
                </div>
                <div class="flex">
                    <CompListItem
                        text="Plain Craft Launcher II: Nova"
                        :desp="`当前版本：${version}（${channel} 渠道）`"
                        :imgSrc="`/Images/PCLNova.${channel}.png`" />
                </div>
            </section>
        </CompCard>
        <CompCard title="鸣谢" can-swap>
            <section class="w-full px-4 flex flex-col gap-2">
                <CompListItem text="龙腾猫跃" desp="Plain Craft Launcher II 的原作者！" imgSrc="/Images/Heads/ltcatt.png" />
            </section>
        </CompCard>
        <CompCard title="版权" can-swap is-swapped>
            <article class="w-full px-4 flex flex-col gap-2">
                <p>PCL2.Nova.App 开源项目及其官方衍生程序均遵循 GNU General Public License 3.0 (GPLv3) 协议开源。</p>
                <section class="mt-2 flex gap-4">
                    <CompButton class="w-36" @click="$jsWindow.open('https://github.com/PCL-Community/PCL2.Nova.App')">
                        查看源代码
                    </CompButton>
                    <CompButton class="w-36" @click="$jsWindow.open('https://www.gnu.org/licenses/gpl-3.0.html')">查看许可证</CompButton>
                </section>
            </article>
        </CompCard>
        <CompCard title="开源项目使用说明" can-swap is-swapped>
            <article class="w-full px-4 flex flex-col gap-2">
                <p>PCL2.Nova.App 开源项目使用了一些开源项目，以下是对应的项目，项目许可证和引用说明</p>
                <ul>
                    <li class="top">· Fluent UI System Icons</li>
                    <li>开源协议：MIT License</li>
                    <li>来源：<kbd class="kbd kbd-sm -translate-y-0.25 select-text!">https://icones.js.org/collection/fluent</kbd></li>
                    <li>使用了其中的部分 Icon</li>
                </ul>
                <ul>
                    <li class="top">· PCL2</li>
                    <li>开源协议：自定义许可证</li>
                    <li>来源：<kbd class="kbd kbd-sm -translate-y-0.25 select-text!">https://github.com/Hex-Dragon/PCL2</kbd></li>
                    <li>参考了其部分内容的实现思路</li>
                </ul>
                <ul>
                    <li class="top">· Firework_Simulator</li>
                    <li>开源协议：Apache 2.0</li>
                    <li>
                        来源：
                        <kbd class="kbd kbd-sm -translate-y-0.25 select-text!">https://github.com/NianBroken/Firework_Simulator</kbd>
                    </li>
                    <li>基于此库进行修改，实现了主页放烟花功能</li>
                </ul>
                <p>在此列出的内容均不属于 Nova 开源项目，版权均归对应项目或仓库作者所有。</p>
            </article>
        </CompCard>
    </main>
</template>

<style lang="scss" scoped>
    article {
        h1 {
            font-size: var(--text-xl);
            line-height: var(--tw-leading, var(--text-xl--line-height));
        }
        p,
        ul li.top {
            font-size: var(--text-lg);
            line-height: var(--tw-leading, var(--text-lg--line-height));
            padding-left: 0;
            padding-right: 0;
        }
        ul {
            padding-left: calc(var(--spacing) * 6);
            padding-right: calc(var(--spacing) * 6);
            li {
                font-size: var(--text-sm);
                line-height: var(--tw-leading, var(--text-sm--line-height));
                padding-left: calc(var(--spacing) * 6);
                padding-right: calc(var(--spacing) * 6);
            }
        }
    }
</style>
