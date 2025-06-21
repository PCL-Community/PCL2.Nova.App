<script setup lang="ts">
    defineProps<{ more: boolean }>();
</script>

<template>
    <button
        class="cursor-pointer w-full flex items-center rounded-lg px-2 py-1 transition-color duration-150 ease-in-out hover:bg-base-100/50 group/togglebtn"
        style="--noise: 0">
        <div class="flex items-center gap-2">
            <i
                class="icon-[material-symbols--keyboard-arrow-up-rounded] size-5 -mx-0.5 opacity-75 transition-all duration-150 ease-in-out group-hover/togglebtn:opacity-100"
                :class="{ 'rotate-180': !$props.more }"></i>
            <span class="translate-y-0.25 opacity-75 group-hover/btn-all:opacity-100">
                显示更{{ $props.more ? "少" : "多" }}
            </span>
        </div>
    </button>
</template>
