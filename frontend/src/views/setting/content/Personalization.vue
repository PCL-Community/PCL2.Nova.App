<script setup lang="ts">
import { ref } from 'vue'
import MySelectCard from '../../../components/card/MySelectCard.vue'
import MyToggleSwitch from '../../../components/button/MyToggleSwitch.vue'
import {dark_mode, theme_mode} from '../../../logic/changeBody'
import MyRadioButton from "../../../components/button/MyRadioButton.vue";
import {GetConfigIniPath, WriteConfig} from "../../../../wailsjs/go/launcher/ReaderWriter";
import MyNormalLabel from "../../../components/input/MyNormalLabel.vue";
import MyNormalButton from "../../../components/button/MyNormalButton.vue";
import {OpenCustomURL} from "../../../logic/functions";
const theme = ref(1)
async function changeDarkMode() {
  dark_mode.value = !dark_mode.value
  await WriteConfig(await GetConfigIniPath(), "Misc", "DarkMode", dark_mode.value ? "1" : "0")
}
async function changeTheme(themeString: number) {
  theme_mode.value = themeString
  await WriteConfig(await GetConfigIniPath(), "Misc", "ThemeMode", theme_mode.value.toString())
}
</script>
<template>
  <div style="display: flex; flex-direction: column;">
    <MySelectCard :isExpand="false" :title="'主题'" :isLast="false" style="position: relative">
      <div style="margin: 10px; position: relative;">
        <div id="mask">
          <MyNormalLabel>请支持官方版本以使用主题功能</MyNormalLabel>
          <MyNormalButton style="width: 200px; height: 40px" @click="OpenCustomURL('https://afdian.com/a/LTCat')">点我进入龙猫的爱发电</MyNormalButton>
        </div>
        <div class="radio-group">
          <MyRadioButton :is-checked="theme_mode === 1" @click="changeTheme(1)">龙猫蓝</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 2" @click="changeTheme(2)">甜柠青</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 3" @click="changeTheme(3)">小草绿</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 4" @click="changeTheme(4)">菠萝黄</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 5" @click="changeTheme(5)">橡木棕</MyRadioButton>
        </div>
        <div class="radio-group">
          <MyRadioButton :is-checked="theme_mode === 6" @click="changeTheme(6)">玄素黑</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 7" @click="changeTheme(7)">滑稽彩</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 8" @click="changeTheme(8)">铁杆粉</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 9" @click="changeTheme(9)">神秘紫</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 10" @click="changeTheme(10)">欧皇彩</MyRadioButton>
        </div>
        <div class="radio-group">
          <MyRadioButton :is-checked="theme_mode === 11" @click="changeTheme(11)">秋仪金</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 12" @click="changeTheme(12)">活跃橙</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 13" @click="changeTheme(13)">跳票红</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 14" @click="changeTheme(14)">极客蓝</MyRadioButton>
          <MyRadioButton :is-checked="theme_mode === 15" @click="changeTheme(15)">自定义</MyRadioButton>
        </div>
      </div>
    </MySelectCard>
    <MySelectCard :isExpand="true" :title="'个性化'" :isLast="true">
      <div style="margin: 10px; display: flex; justify-content: space-between;">
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
#mask {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  backdrop-filter: blur(2px);
  z-index: 2;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
#dark-mode-title {
  /* float: left; */
  margin: 10px;
  font-weight: bold;
}
.radio-group {
  margin-top: 10px;
  display: flex;
  justify-content: space-around;
}
#dark-mode-toggle {
  margin: 10px;
  /* float: right; */
}
</style>