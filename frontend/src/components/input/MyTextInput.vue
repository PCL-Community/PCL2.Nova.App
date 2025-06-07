<script setup lang="ts">
import {ref, watch} from 'vue'
import {dark_mode} from '../../logic/changeBody'

interface TextInputProps {
  modelValue?: string,
  place_holder?: string
}

const props = withDefaults(defineProps<TextInputProps>(), {place_holder: '', modelValue: ''})
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()
const light = ref(dark_mode.value ? '#e6e6e6cf' : '#1a1a1acf')
const dark = ref(dark_mode.value ? '#1a1a1acf' : '#e6e6e6cf')
watch(dark_mode, value => {
  light.value = value ? '#e6e6e6cf' : '#1a1a1acf'
  dark.value = value ? '#1a1a1acf' : '#e6e6e6cf'
})
const handleInput = (e: Event) => {
  emit('update:modelValue', (e.target as HTMLInputElement).value)
}
</script>
<template>
  <input type="text" :placeholder="props.place_holder" class="text-input font-pcl" :value="modelValue"
         @input="handleInput">
</template>
<style scoped>
.text-input {
  border: 1px solid v-bind(light);
  border-radius: 5px;
  background-color: v-bind(dark);
  color: v-bind(light);
  padding-left: 10px;
  padding-right: 10px;
}
</style>