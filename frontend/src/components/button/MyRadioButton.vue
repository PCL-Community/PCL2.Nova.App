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
  <div :class="['radio', isCheckedProps.isChecked ? '' : 'cursor-pointer', isCheckedProps.isChecked ? 'button-active' : 'button-style']"><div class="circle"><div></div></div><div class="slot"><slot /></div></div>
</template>

<style scoped>
.radio {
  color: v-bind(light);
  height: 30px;
  width: max-content;
  display: flex;
  align-items: center;
}
.slot {
  position: relative;
  top: -1px;
}
.circle {
  margin-right: 5px;
  transition: all 0.2s;
  border-radius: 50%;
  width: 17px;
  height: 17px;
}
.button-active .circle {
  border: 1px solid deepskyblue;
  position: relative;
}
.button-style .circle div {
  background-color: transparent;
  transition: background-color 0.2s;
}
.button-active .circle div {
  position: absolute;
  background-color: deepskyblue;
  border-radius: 50%;
  top: 2px;
  left: 2px;
  width: 13px;
  height: 13px;
  transition: background-color 0.2s;
}
.button-style .circle {
  border: 1px solid #A0A0A0;
}
.button-style:hover .circle {
  border: 1px solid deepskyblue;
}
.button-style:active .circle {
  transform: scale(86%);
}
</style>