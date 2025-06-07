<script setup lang="ts">
    defineProps<{ rounded: boolean; flexCenter?: boolean }>();
</script>

<template>
    <button
        style="--noise: 0"
        class="btn rounded-full scale-75 border-0 shadow-none bg-transparent hover:bg-white/20"
        :class="{ 'btn-circle': rounded, 'flex justify-center items-center': flexCenter }">
        <slot></slot>
    </button>
</template>
