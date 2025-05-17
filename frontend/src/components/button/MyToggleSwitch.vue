<script setup lang="ts">
interface CheckButtonProps {
  isChecked?: boolean
}

const isCheckedProps = withDefaults(defineProps<CheckButtonProps>(), {isChecked: false})
import {ref, watch} from 'vue'

const check_value = ref(isCheckedProps.isChecked)
import {dark_mode} from "../../logic/changeBody";
const rdBack = ref(dark_mode.value ? "#303030" : "#fff")
const ncBack = ref(dark_mode ? "#909090" : "lightgray")
watch(dark_mode, v => {
  rdBack.value = v ? "#303030" : "#fff"
})
watch(() => isCheckedProps.isChecked, v => check_value.value = v)
const back = ref(ncBack)
const left = ref("4px")
watch(check_value, v => {
  ncBack.value = dark_mode.value ? "#707070" : "lightgray"
  back.value = v ? "skyblue" : ncBack.value
  left.value = v ? "28px" : "4px"
}, {immediate: true})
</script>
<template>
  <div class="switch">
    <button class="toggle-span"></button>
  </div>
</template>
<style scoped>
.switch {
  position: relative;
  width: 48px;
  height: 24px;
  background-color: v-bind(back);
  border-radius: 34px;
  cursor: pointer;
  transition: 0.2s;
  box-shadow: 2px 2px 5px gray;
}

.toggle-span {
  border-radius: 50%;
  cursor: pointer;
  width: 16px;
  height: 16px;
  margin: 4px 4px 4px v-bind(left);
  border: 0;
  background-color: v-bind(rdBack);
  transition: 0.2s;
  box-shadow: 2px 2px 5px gray;
}
</style>