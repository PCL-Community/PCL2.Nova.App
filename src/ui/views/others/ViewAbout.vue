<script setup lang="ts">
    import CompCard from "../../components/CompCard.vue";
    import CompListItem from "../../components/CompListItem.vue";
</script>

<template>
    <main class="flex flex-col gap-4">
        <CompCard title="关于">
            <section class="w-full px-4 flex flex-col gap-2">
                <div class="flex">
                    <CompListItem text="凌云" desp="Plain Craft Launcher II: Nova 的作者！" imgSrc="/avatar/lingyun.png" />
                </div>
                <div class="flex">
                    <CompListItem
                        text="Plain Craft Launcher II: Nova"
                        desp="当前版本：0.1.0-nightly.2（Dev 渠道）"
                        imgSrc="/PCLNova.Dev.png" />
                </div>
            </section>
        </CompCard>
        <CompCard title="鸣谢" can-swap>
            <section class="w-full px-4 flex flex-col gap-2">
                <CompListItem text="龙腾猫跃" desp="Plain Craft Launcher II 的原作者！" imgSrc="/avatar/ltcatt.png" />
            </section>
        </CompCard>
        <CompCard title="版权" can-swap is-swapped>
            <article class="w-full px-4 flex flex-col gap-2">
                <p>
                    PCL2.Nova.App 开源项目（以下简称 Nova 开源项目）及其<b>官方</b>衍生程序
                    <br />均遵循 GNU General Public License 3.0 (GPLv3) 协议开源，并在此基础上附加以下条款：
                </p>
                <ul>
                    1. 基于 Nova 开源项目做小修改时，可以不对修改后的代码开源，但需要明确在软件内提供指向 Nova
                    开源项目的链接（末尾附），并添加 Nova开源项目原作者的署名。
                </ul>
                <ul>
                    2. 基于 Nova 开源项目做大量修改，或者进行二次开发时，需要对修改后的代码开源，但不一定需要使用 GPLv3
                    许可证。同时，需要明确在软件内提供指向 Nova 开源项目的链接（末尾附），并添加 Nova 开源项目原作者的署名。
                </ul>
                <ul>
                    3. 如果选择在 GitHub 上开源代码，对 Nova 开源项目仓库的 Fork 可以 视为 “指向 Nova 开源项目的链接”。
                </ul>
                <ul>
                    4. 无论修改体量如何，不应当制作 Nova
                    开源项目中需要赞助才能获取的功能，或与其相似的功能。项目作者建议您在计划开发此类内容前先询 问作者以确认是否触及此条款。
                </ul>
                <ul>
                    5. 修改之后的版本不得用于商业用途，无论是否开源。
                </ul>
                <ul>
                    6. 不得以任何方式让原作者为您或您的使用做任何担保。
                </ul>
                <ul>
                    7.
                    修改之后的版本如果开源，需要包含此附加协议，并不得修改其条款细则。新项目使用的协议不同时可修改位于附加许可证开头的协议说明。
                </ul>
                <br />
                <hr />
                <br />
                <p>
                    附：Nova 开源项目链接为
                    <kbd class="kbd kbd-sm -translate-y-0.25 select-text!">https://github.com/PCL-Community/PCL2.Nova.App</kbd>
                </p>
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
