<script setup lang="ts">
import MySelectCard from '../../../components/card/MySelectCard.vue'
import MyToggleSwitch from '../../../components/button/MyToggleSwitch.vue'
import {dark_mode} from '../../../logic/changeBody'

function changeDarkMode() {
  dark_mode.value = !dark_mode.value
}
</script>
<template>
  <div style="display: flex; flex-direction: column;">
    <MySelectCard :maxHeight="60" :isExpand="true" :title="'个性化'" :isLast="true">
      <div style="margin: 10px;">
        <div id="dark-mode-title">
          暗色模式
        </div>
        <div id="dark-mode-toggle">
          <MyToggleSwitch @click="changeDarkMode" :isChecked="dark_mode"/>
        </div>
      </div>
    </MySelectCard>
  </div>
</template>
<style scoped>
#dark-mode-title {
  float: left;
  margin: 10px;
  font-weight: bold;
}

#dark-mode-toggle {
  margin: 10px;
  float: right;
}
</style>