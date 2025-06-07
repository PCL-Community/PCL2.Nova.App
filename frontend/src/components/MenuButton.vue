<script setup lang="ts">
    defineProps<{ text: string; selected?: boolean; reverse?: boolean }>();
</script>

<template>
    <button
        style="--noise: 0"
        class="btn rounded-full scale-75 border-0 shadow-none flex justify-center items-center"
        :class="{
            'bg-white! text-primary!': selected && !reverse,
            'bg-transparent hover:bg-white/20 text-white!': !selected && !reverse,
            'bg-primary! text-white!': selected && reverse,
            'bg-transparent hover:bg-primary/20 text-primary!': !selected && reverse,
        }">
        <slot></slot>
        <span class="text-lg">{{ $props.text }}</span>
    </button>
</template>
