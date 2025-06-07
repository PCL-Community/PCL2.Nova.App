<script setup lang="ts">
interface CheckButtonProps {
  isChecked?: boolean
}
const isCheckedProps = withDefaults(defineProps<CheckButtonProps>(), {isChecked: false})
import {dark_mode} from '../../logic/changeBody'
import {ref, watch} from 'vue'

const light = ref(dark_mode.value ? '#e6e6e6cf' : '#1a1a1acf')
watch(dark_mode, value => {
  light.value = value ? '#e6e6e6cf' : '#1a1a1acf'
})
</script>

<template>
  <div :class="['check cursor-pointer', isCheckedProps.isChecked ? 'button-active' : 'button-style']">
    <div class="correct">
      <svg
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          width="13"
          height="13"
          stroke="#00BFFFFF"
          stroke-width="3"
          stroke-linecap="round"
          stroke-linejoin="round"
          fill="none">
        <path d="M1.5 16.5 L7.5 22.5 M7.5 22.5 L22.5 7.5" />
      </svg>
    </div>
    <div class="slot">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.check {
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
.correct {
  margin-right: 5px;
  transition: all 0.2s;
  border-radius: 5px;
  width: 17px;
  height: 17px;
}
.button-active .correct {
  border: 2px solid deepskyblue;
  position: relative;
}
.button-style .correct {
  border: 2px solid #A0A0A0;
}
.button-style:hover .correct {
  border: 2px solid deepskyblue;
}
.check:active .correct {
  transform: scale(86%);
}
.correct svg {
  position: absolute;
  opacity: 0;
  top: 2px;
  left: 2px;
  width: 13px;
  height: 13px;
  transition: opacity 0.2s;
}
.button-style .correct svg {
  opacity: 0;
}
.button-active .correct svg {
  opacity: 1;
}
</style>