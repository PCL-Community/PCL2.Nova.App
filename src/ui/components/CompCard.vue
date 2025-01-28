<script setup lang="ts">
    import { ref } from "vue";

    const props = defineProps<{
        title?: string;
        canSwap?: boolean;
        isSwapped?: boolean;
    }>();

    const yuncard = ref<HTMLDivElement>();
    const isCardOpen = ref<boolean>(props.canSwap ? (props.isSwapped ? false : true) : true);
    const toggleCard = () => {
        if (!props.canSwap || yuncard.value == null) return;
        isCardOpen.value = !isCardOpen.value;
        switch (isCardOpen.value) {
            case true:
                yuncard.value.style.height = "auto";
                const { height } = yuncard.value.getBoundingClientRect();
                yuncard.value.style.height = "48px";
                yuncard.value.offsetHeight; // force reflow
                yuncard.value.style.height = `${height}px`;
                break;
            case false:
                yuncard.value.style.height = "48px";
        }
    };
</script>

<template>
    <div
        class="yun-card group"
        :class="{
            swapped: !isCardOpen,
        }"
        ref="yuncard">
        <div
            class="yun-card__top"
            :class="{
                canSwap: props.canSwap,
            }"
            v-if="props.title || props.canSwap"
            @click="toggleCard">
            <span class="-translate-y-0.5 group-hover:text-info transition ml-1.5">{{ props.title }}</span>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="w-6 h-6 yun-card__top-icon"
                viewBox="0 0 24 24"
                v-if="props.canSwap"
                :class="{
                    swapped: isCardOpen,
                }">
                <path fill="currentColor" d="M8.59 16.58L13.17 12L8.59 7.41L10 6l6 6l-6 6z"></path>
            </svg>
        </div>
        <slot></slot>
    </div>
</template>

<style scoped lang="scss">
    div.yun-card {
        background-color: var(--color-base-300);
        padding: 12px;
        border-radius: 8px;
        overflow: hidden;
        transition: all 0.2s ease-in-out;

        &:hover {
            box-shadow: 0 0 6px var(--color-primary);
        }

        div.yun-card__top {
            display: flex;
            margin-bottom: 12px;

            .yun-card__top-icon {
                margin-left: auto;
                transition: all 0.3s cubic-bezier(0.27, -0.51, 0.74, 1.51);
                transform: translateY(-1px) rotate(90deg);
            }

            span {
                font-weight: bold;
                transform: translateY(2px);
            }

            &.canSwap {
                cursor: pointer;
            }
        }

        &.swapped {
            height: 48px;
        }

        div.yun-card__top .yun-card__top-icon.swapped {
            transform: translateY(-1px) rotate(-90deg);
        }
    }
</style>
