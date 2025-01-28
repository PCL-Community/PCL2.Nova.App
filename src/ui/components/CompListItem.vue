<script lang="ts" setup>
    const props = defineProps<{
        text: string;
        desp?: string;
        imgSrc?: string;
    }>();
</script>

<template>
    <section role="button" class="w-full h-12 px-2 rounded-sm flex items-center gap-4">
        <img v-if="props.imgSrc" :src="props.imgSrc" class="w-11 rounded-full"/>
        <div class="w-full h-full flex flex-col justify-center items-start">
            <span class="text-[1.1rem]">{{ props.text }}</span>
            <span class="text-xs opacity-70">{{ props.desp }}</span>
        </div>
    </section>
</template>
