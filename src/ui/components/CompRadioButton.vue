<script lang="ts" setup>
    const props = defineProps<{
        text: string,
        checked: boolean,
    }>();
</script>

<template>
    <div :class="{'btn-ghost': !props.checked, ' hover:bg-black/20': !props.checked, 'text-white': !props.checked}"
         class="btn h-full py-1 rounded-full text-primary-content"
         role="button">
        <span class="translate-x-[.5px]">{{ props.text }}</span>
    </div>
</template>

<style lang="scss" scoped>

</style>