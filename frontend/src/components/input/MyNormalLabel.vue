<script setup lang="ts">
import {ref, watch} from 'vue'
import {dark_mode} from '../../logic/changeBody'

const light = ref(dark_mode.value ? '#e6e6e6' : '#1a1a1a')
watch(dark_mode, value => {
  light.value = value ? '#e6e6e6' : '#1a1a1a'
})
</script>
<template>
  <label class="label">
    <slot/>
  </label>
</template>
<style scoped>
.label {
  font-weight: bold;
  color: v-bind(light);
}
</style>