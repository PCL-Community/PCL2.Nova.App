<script setup lang="ts">
    import { ref } from "vue";

    const props = defineProps<{
        excludeable?: boolean;
        selected?: boolean;
        selectedExclude?: boolean;
    }>();
    const emit = defineEmits(["update:select", "update:exclude"]);

    const isSelected = ref<boolean>(props.selected);
    const isSelectedExclude = ref<boolean>(props.selectedExclude);

    const toggle = (bypass: boolean) => {
        if (isSelectedExclude.value && !bypass) {
            toggleExclude();
            return;
        }
        isSelected.value = !isSelected.value;
        emit("update:select", isSelected.value);
    };
    const toggleExclude = () => {
        isSelectedExclude.value = !isSelectedExclude.value;
        if (isSelectedExclude.value && isSelected.value) toggle(true);
        emit("update:exclude", isSelectedExclude.value);
    };
</script>

<template>
    <div class="w-full flex group/btn-all" :class="{ 'bg-error/25 rounded-lg': isSelectedExclude }">
        <button
            @click="toggle(false)"
            class="cursor-pointer w-full flex items-center rounded-lg px-2 py-1 transition-color duration-150 ease-in-out group/btn-self1"
            :class="{ 'bg-success/25!': isSelected, 'hover:bg-base-100/50': !isSelectedExclude }"
            style="--noise: 0">
            <span
                class="opacity-75 group-hover/btn-all:opacity-100"
                :class="{ 'opacity-100!': isSelected || isSelectedExclude }">
                <slot></slot>
            </span>
            <i
                class="icon-[ic--outline-check] ml-auto opacity-0 group-hover/btn-self1:text-success group-hover/btn-all:opacity-100 transition-opacity duration-150 ease-in-out"
                :class="{ 'opacity-100! text-white!': isSelected, 'opacity-0!': isSelectedExclude }"></i>
        </button>
        <button
            @click="toggleExclude"
            v-if="$props.excludeable"
            class="cursor-pointer w-8 flex justify-center items-center rounded-lg px-2 py-1 transition-color duration-150 ease-in-out group/btn-self2"
            :class="{
                'hover:bg-base-100/50': !isSelectedExclude,
            }"
            style="--noise: 0">
            <i
                class="icon-[ic--baseline-do-not-disturb] translate-x-[0.25px] opacity-0 group-hover/btn-self2:text-error group-hover/btn-all:opacity-100 transition-opacity duration-150 ease-in-out"
                :class="{ 'opacity-100! text-white!': isSelectedExclude }"></i>
        </button>
    </div>
</template>
