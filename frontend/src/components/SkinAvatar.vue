<script setup lang="ts">
    defineProps<{ skin: string | URL; size: number }>();
</script>

<template>
    <div class="relative shadow-lg shadow-base-100/20" :class="`size-${size / 4}`">
        <div
            class="absolute inset-0 bg-no-repeat"
            style="image-rendering: pixelated"
            :style="`background-position: -${size}px -${size}px; background-size: ${size * 8}px ${
                size * 8
            }px; background-image: url(${skin});`"></div>
        <div
            class="absolute inset-0 bg-no-repeat"
            style="image-rendering: pixelated"
            :style="`background-position: -${size * 5}px -${size}px; background-size: ${size * 8}px ${
                size * 8
            }px; background-image: url(${skin});`"></div>
    </div>
</template>
