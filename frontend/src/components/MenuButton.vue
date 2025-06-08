<script setup lang="ts">
    defineProps<{ text: string; selected?: boolean; reverse?: boolean }>();
</script>

<template>
    <button
        style="--noise: 0"
        class="btn rounded-full scale-75 border-0 shadow-none flex justify-center items-center"
        :class="{
            'bg-white! text-secondary!': selected && !reverse,
            'bg-transparent hover:bg-white/20 text-white!': !selected && !reverse,
            'bg-secondary! text-white!': selected && reverse,
            'bg-transparent hover:bg-secondary/20 text-secondary!': !selected && reverse,
        }">
        <slot></slot>
        <span class="text-lg translate-y-0.25">{{ $props.text }}</span>
    </button>
</template>
