<script setup lang="ts">
    defineProps<{
        text: string;
        selected?: boolean;
    }>();
</script>

<template>
    <div
        class="relative w-full h-9 pl-3 flex items-center gap-2 border border-transparent hover:bg-primary/25 hover:border-secondary/25 transition-color ease-in-out duration-150 cursor-pointer"
        :class="{ 'text-secondary': $props.selected }">
        <Transition name="spring">
            <i class="absolute left-0 w-1 h-2/3 bg-primary rounded-[0_1.5px_1.5px_0]" v-if="$props.selected"></i>
        </Transition>
        <slot></slot>
        <span class="translate-y-0.25 text-sm">{{ $props.text }}</span>
        <slot name="extra"></slot>
    </div>
</template>

<style lang="css" scoped>
    .spring-enter-active {
        animation: spring-in 100ms cubic-bezier(0.34, 1.56, 0.64, 1);
    }
    .spring-leave-active {
        animation: spring-out 50ms cubic-bezier(0.34, 1.56, 0.64, 1);
    }
    @keyframes spring-in {
        0% {
            transform: scale(0.8);
            opacity: 0;
        }
        60% {
            transform: scale(1.1);
            opacity: 1;
        }
        80% {
            transform: scale(0.95);
        }
        100% {
            transform: scale(1);
        }
    }
    @keyframes spring-out {
        0% {
            transform: scale(1);
        }
        60% {
            transform: scale(0.95);
        }
        80% {
            transform: scale(1.1);
            opacity: 1;
        }
        100% {
            transform: scale(0.8);
            opacity: 0;
        }
    }
</style>
