<script setup lang="ts">
interface CheckButtonProps {
  isChecked?: boolean
}

const isCheckedProps = withDefaults(defineProps<CheckButtonProps>(), {isChecked: false})
import {dark_mode} from '../../logic/changeBody'
import {ref, watch} from 'vue'

const light = ref(dark_mode.value ? '#e6e6e6' : '#1a1a1a')
watch(dark_mode, value => {
  light.value = value ? '#e6e6e6' : '#1a1a1a'
})
</script>
<template>
  <button :class="isCheckedProps.isChecked ? 'button-active' : ('button-style cursor-pointer')">
    <slot/>
  </button>
</template>
<style scoped>
.button-style {
  font-weight: bold;
  background-color: transparent;
  border: 0;
  border-radius: 50px;
  transition: all 0.2s;
  color: v-bind(light);
  stroke: v-bind(light);
}

.button-active {
  font-weight: bold;
  background-color: rgb(19, 85, 206);
  border: 1px solid gray;
  box-shadow: 0 1px 3px gray;
  border-radius: 50px;
  transition: all 0.2s;
  color: #e6e6e6;
  stroke: #e6e6e6;
}

.button-style:hover {
  background-color: rgb(164, 191, 242);
  box-shadow: 0 1px 3px gray;
}

.button-style:active {
  transform: scale(0.96);
}

.button-active:hover {
  background-color: rgb(0, 66, 187);
}
</style>