<script setup lang="ts">
    defineProps<{
        title: string;
        description: string;
        slug: string;
        project_id: string;
        icon_url?: string;
        downloads: number;
    }>();
</script>

<template>
    <div
        class="w-full h-22 bg-[color-mix(in_oklch,_var(--color-base-100)_90%,_var(--color-base-content)_10%)] rounded-lg flex gap-4 items-center px-4 mb-2 last:mb-0 hover:bg-[color-mix(in_oklch,_var(--color-primary)_20%,_var(--color-base-100)))] cursor-pointer">
        <img :src="icon_url" class="size-16 rounded-lg" />
        <div class="w-full flex flex-col justify-center">
            <span class="text-md font-medium">{{ title }}</span>
            <span class="truncate max-w-17/20 text-sm text-[color-mix(in_oklch,_var(--color-base-100)_90%,_var(--color-base-content)_50%)]" :title="description">
                {{ description }}
            </span>
        </div>
    </div>
</template>
