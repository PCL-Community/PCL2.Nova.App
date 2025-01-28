<script lang="ts" setup>
    const props = defineProps<{
        text: string;
        checked: boolean;
        reset?: boolean;
    }>();
</script>

<template>
    <div
        :class="{
            'text-info': props.checked,
        }"
        class="w-full h-auto py-2 hover:bg-info/10 rounded-none flex gap-2 items-center cursor-pointer transition-all duration-200 ease-in-out"
        role="button">
        <section class="w-1 h-[calc(100%+calc(var(--spacing)*1))]" :class="{ 'bg-info': props.checked }"></section>
        <section class="w-5 h-5 flex justify-center items-center">
            <slot></slot>
        </section>
        <span class="leading-4.5]">{{ props.text }}</span>
        <slot name="end"></slot>
    </div>
</template>
