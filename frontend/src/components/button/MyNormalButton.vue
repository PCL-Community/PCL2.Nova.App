<script setup lang="ts">
import {dark_mode} from '../../logic/changeBody'
import {ref, watch} from 'vue'

const dark = ref(dark_mode.value ? '#1a1a1a' : '#f6f6f6')
const light = ref(dark_mode.value ? '#f6f6f6' : '#1a1a1a')
const hov = ref(dark_mode.value ? '#0a0a0a' : '#d6d6d6')
watch(dark_mode, value => {
  dark.value = value ? '#1a1a1a' : '#e6e6e6'
  light.value = value ? '#e6e6e6' : '#1a1a1a'
  hov.value = value ? '#0a0a0a' : '#d6d6d6'
})
</script>
<template>
  <button class="button-style cursor-pointer">
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

.button-style:hover {
  background-color: v-bind(hov);
}

.button-style:active {
  transform: scale(0.96);
}
</style>