<script setup lang="ts">
interface ProgressBarProps{
  maxValue: number;
  currentValue: number;
  width: string;
  height: string;
}
const isCheckedProps = defineProps<ProgressBarProps>()
const width = ref(isCheckedProps.width);
const height = ref(isCheckedProps.height)
const maxValue = ref(isCheckedProps.maxValue)
import { ref, watch } from 'vue'
import { dark_mode } from "../../logic/changeBody";
const curwidth = ref("")
const backcolor = ref(dark_mode.value ? '#303030' : 'lightgray')
watch(dark_mode, value => {
  backcolor.value = value ? '#303030' : 'lightgray'
})
watch(() => isCheckedProps.currentValue, v => {
  curwidth.value = (v / maxValue.value) * parseInt(width.value) + "%"
}, {immediate: true})
</script>

<template>
  <div class="progress-container">
    <div class="progress-bar"></div>
  </div>
</template>

<style scoped>
.progress-container {
  width: v-bind(width);
  height: v-bind(height);
  background-color: v-bind(backcolor);
  border-radius: 1000px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  border-radius: 1000px;
  background-color: rgb(1, 114, 172);
  transition: all 0.2s;
  width: v-bind(curwidth);
}
</style>