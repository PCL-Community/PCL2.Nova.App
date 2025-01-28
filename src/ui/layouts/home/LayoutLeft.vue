<script setup lang="ts">
    import { ref } from "vue";
    import CompRadioButton from "../../components/CompRadioButton.vue";
    import CompInput from "../../components/CompInput.vue";
    import CompSkin from "../../components/CompSkin.vue";
    import CompButton from "../../components/CompButton.vue";

    const comp_config = ref<{
        is_multi_login: boolean;
        /**
         * @param {string} "ms" 微软登录
         * @param {string} "legacy" 离线登录
         * @param {string} "pass" Authlib-Injector | 统一通行证
         * @param {string} "skin" Authlib-Injector | 皮肤站
         */
        login_method: "ms" | "legacy" | "pass" | "skin";
        is_login: boolean;
        login_data?: {
            skin_url: string;
            name: string;
            email?: string;
            auth_server?: {
                type: "pass" | "skin";
                name: string;
            };
        };
    }>({
        is_multi_login: true,
        login_method: "legacy",
        is_login: false,
    });

    const login_details = ref<{
        email: string;
        password: string;
    }>({
        email: "",
        password: "",
    });
</script>

<template>
    <section class="w-full h-full bg-base-300/90 py-8 flex flex-col justify-around items-center">
        <div data-area="selector">
            <section class="w-full flex justify-center gap-8" v-if="comp_config.is_multi_login">
                <CompRadioButton
                    text="正版"
                    :checked="comp_config.login_method === `ms`"
                    :highlight="true"
                    @click="comp_config.login_method = `ms`">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path
                            fill="currentColor"
                            d="M16.757 9.303a.75.75 0 0 0-1.014-1.106l-5.47 5.015L8.28 11.22a.75.75 0 0 0-1.06 1.06l2.5 2.5a.75.75 0 0 0 1.037.023zM20.25 5c-2.663 0-5.258-.943-7.8-2.85a.75.75 0 0 0-.9 0C9.008 4.057 6.413 5 3.75 5a.75.75 0 0 0-.75.75V11c0 5.001 2.958 8.676 8.725 10.948a.75.75 0 0 0 .55 0C18.042 19.676 21 16 21 11V5.75a.75.75 0 0 0-.75-.75M4.5 6.478c2.577-.152 5.08-1.09 7.5-2.8c2.42 1.71 4.923 2.648 7.5 2.8V11c0 4.256-2.453 7.379-7.5 9.442C6.953 18.379 4.5 15.256 4.5 11z" />
                    </svg>
                </CompRadioButton>
                <CompRadioButton
                    text="离线"
                    :checked="comp_config.login_method === `legacy`"
                    :highlight="true"
                    @click="comp_config.login_method = `legacy`">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" class="scale-75">
                        <path
                            fill="currentColor"
                            d="M533.293176 788.841412a60.235294 60.235294 0 1 1 85.202824 85.202823l-42.616471 42.586353c-129.355294 129.385412-339.124706 129.385412-468.510117 0-129.385412-129.385412-129.385412-339.124706 0-468.510117l42.586353-42.616471a60.235294 60.235294 0 1 1 85.202823 85.202824l-42.61647 42.586352a210.823529 210.823529 0 1 0 298.164706 298.164706l42.586352-42.61647z m255.548236-255.548236l42.61647-42.586352a210.823529 210.823529 0 1 0-298.164706-298.164706l-42.586352 42.61647a60.235294 60.235294 0 1 1-85.202824-85.202823l42.616471-42.586353c129.355294-129.385412 339.124706-129.385412 468.510117 0 129.385412 129.385412 129.385412 339.124706 0 468.510117l-42.586353 42.616471a60.235294 60.235294 0 1 1-85.202823-85.202824zM192.542118 192.542118a60.235294 60.235294 0 0 1 85.202823 0l553.712941 553.712941a60.235294 60.235294 0 0 1-85.202823 85.202823L192.542118 277.744941a60.235294 60.235294 0 0 1 0-85.202823z" />
                    </svg>
                </CompRadioButton>
            </section>
            <section v-else></section>
        </div>
        <div data-area="login" class="w-full h-full px-6 flex flex-col justify-center">
            <section v-if="comp_config.login_method == `ms`" class="flex flex-col items-center gap-8">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-16" viewBox="0 0 1024 1024">
                    <path
                        fill="var(--color-info)"
                        d="M660.338 528.065c63.61-46.825 105.131-121.964 105.131-206.83 0-141.7-115.29-256.987-256.997-256.987-141.706 0-256.998 115.288-256.998 256.987 0 85.901 42.52 161.887 107.456 208.562-152.1 59.92-260.185 207.961-260.185 381.077 0 21.276 17.253 38.53 38.53 38.53 21.278 0 38.53-17.254 38.53-38.53 0-183.426 149.232-332.671 332.667-332.671 1.589 0 3.113-0.207 4.694-0.244 0.8 0.056 1.553 0.244 2.362 0.244 183.434 0 332.664 149.245 332.664 332.671 0 21.276 17.255 38.53 38.533 38.53 21.277 0 38.53-17.254 38.53-38.53 0-174.885-110.354-324.13-264.917-382.809z m-331.803-206.83c0-99.22 80.72-179.927 179.935-179.927s179.937 80.708 179.937 179.927c0 99.203-80.721 179.91-179.937 179.91s-179.935-80.708-179.935-179.91z" />
                </svg>
                <section class="w-full px-6 flex gap-2">
                    <div class="dropdown w-full">
                        <CompButton class="btn-sm w-full flex justify-start">添加新帐号</CompButton>
                        <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-1 p-1 shadow-sm w-full">
                            <li><a>添加新账号</a></li>
                        </ul>
                    </div>
                    <CompButton class="btn-sm" highlight>登录</CompButton>
                </section>
            </section>
            <section v-if="comp_config.login_method == `legacy`" class="flex flex-col items-center gap-8">
                <CompSkin />
                <CompInput class="px-4" legend="用户名" v-model="login_details.email" />
            </section>
        </div>
        <div data-area="button" class="w-full px-6">
            <div
                role="button"
                class="w-full h-full btn btn-outline border border-info hover:bg-[var(--color-btn-hover)] flex flex-col py-3">
                <h1 class="text-xl font-normal bg-gradient-to-r from-info to-primary text-transparent bg-clip-text">启动游戏</h1>
                <span class="text-xs text-base-content/85">测试客户端</span>
            </div>
        </div>
        <section class="w-full flex gap-2 mt-2 px-6">
            <CompButton class="basis-[calc(50%-calc(var(--spacing)*1))]">选择核心</CompButton>
            <CompButton class="basis-[calc(50%-calc(var(--spacing)*1))]">核心设置</CompButton>
        </section>
    </section>
</template>
