<script setup lang="ts">
interface CheckButtonProps {
  isChecked?: boolean
}

const isCheckedProps = withDefaults(defineProps<CheckButtonProps>(), {isChecked: false})
import {dark_mode} from '../../logic/changeBody'
import {ref, watch} from 'vue'

const light = ref(dark_mode.value ? '#e6e6e6' : '#1a1a1a')
const dark = ref(dark_mode.value ? '#0a0a0a' : '#d6d6d6')
watch(dark_mode, value => {
  light.value = value ? '#e6e6e6' : '#1a1a1a'
  dark.value = value ? '#0a0a0a' : '#d6d6d6'
})
</script>
<template>
  <div style="position: relative;">
    <button :class="isCheckedProps.isChecked ? 'button-active' : ('button-style cursor-pointer')">
      <slot/>
    </button>
  </div>
</template>
<style scoped>
.button-style,
.button-active {
  background-color: transparent;
  width: 144px;
  height: 40px;
  border: 0;
  font-size: 16px;
  transition: all 0.2s;
  stroke: v-bind(light);
  color: v-bind(light);
}

.button-active {
  color: rgb(0, 186, 254);
  stroke: rgb(0, 186, 254);
}

.button-active::before,
.button-style::before {
  position: absolute;
  content: '';
  width: 4px;
  height: 0px;
  background-color: rgb(0, 186, 254);
  left: 0;
  top: 0px;
  transition: all 0.2s;
}

.button-active::before {
  top: 6px;
  height: 28px;
}

.button-style:hover,
.button-active:hover {
  background-color: v-bind(dark);
}
</style>