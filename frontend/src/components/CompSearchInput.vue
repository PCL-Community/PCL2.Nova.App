<script setup lang="ts">
    defineProps<{
        modelValue: string;
        placeholder?: string;
        disabled?: boolean;
        clearable?: boolean;
    }>();

    const emit = defineEmits<{
        (e: "update:modelValue", value: string): void;
    }>();

    function onInput(event: Event) {
        const target = event.target as HTMLInputElement;
        emit("update:modelValue", target.value);
    }
</script>

<template>
    <label class="input">
        <i class="icon-[ic--baseline-search] size-6 -translate-y-0.25"></i>
        <input
            type="text"
            class="grow"
            :value="modelValue"
            @input="onInput"
            :placeholder="placeholder"
            :disabled="disabled" />
    </label>
</template>
