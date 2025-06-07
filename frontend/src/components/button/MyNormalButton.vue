<script setup lang="ts">
import {dark_mode} from '../../logic/changeBody'
import {ref, watch} from 'vue'
interface NormalButtonProps {
  isDisabled?: boolean
}
const isDisabledProps = withDefaults(defineProps<NormalButtonProps>(), {isDisabled: false})

const dark = ref(dark_mode.value ? '#1a1a1acf' : '#f6f6f6cf')
const light = ref(dark_mode.value ? '#f6f6f6cf' : '#1a1a1acf')
const hov = ref(dark_mode.value ? '#0a0a0acf' : '#d6d6d6cf')
watch(dark_mode, value => {
  dark.value = value ? '#1a1a1acf' : '#e6e6e6cf'
  light.value = value ? '#e6e6e6cf' : '#1a1a1acf'
  hov.value = value ? '#0a0a0acf' : '#d6d6d6cf'
})
</script>
<template>
  <button class="button-style cursor-pointer" :disabled="isDisabled">
    <slot/>
  </button>
</template>
<style scoped>
.button-style {
  background-color: v-bind(dark);
  border-radius: 6px;
  border: 1px solid v-bind(light);
  transition: all 0.2s;
  color: v-bind(light)
}
.button-style[disabled] {
  color: #7f7f7f7f;
  border: 1px solid #7f7f7f7f;
  cursor: auto;
}
.button-style[disabled]:hover {
  background-color: v-bind(dark)
}
.button-style[disabled]:active {
  transform: scale(100%);
}
.button-style:hover {
  background-color: v-bind(hov);
}

.button-style:active {
  transform: scale(0.96);
}
</style>