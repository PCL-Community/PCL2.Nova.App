<script setup lang="ts">
import {dark_mode} from '../../logic/changeBody'
import {ref, watch} from 'vue'
interface NormalButtonProps {
  isDisabled?: boolean
}
const isDisabledProps = withDefaults(defineProps<NormalButtonProps>(), {isDisabled: false})

const dark = ref(dark_mode.value ? '#1a1a1acc' : '#f6f6f6cc')
const light = ref(dark_mode.value ? '#f6f6f6cc' : '#1a1a1acc')
const hov = ref(dark_mode.value ? '#0a0a0acc' : '#d6d6d6cc')
watch(dark_mode, value => {
  dark.value = value ? '#1a1a1acc' : '#e6e6e6cc'
  light.value = value ? '#e6e6e6cc' : '#1a1a1acc'
  hov.value = value ? '#0a0a0acc' : '#d6d6d6cc'
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
  font-weight: bold;
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