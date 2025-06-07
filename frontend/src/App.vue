<script setup lang="ts">
import {DarkAndThemeToConst} from "./logic/functions";
import NavBar from './views/NavBar.vue';
import Body from './views/Body.vue'
import {dark_mode, theme_mode} from './logic/changeBody'
import {onMounted, ref, watch} from 'vue'
import MyDialog from './components/card/MyDialog.vue'
import {GetConfigIniPath, ReadConfig} from "../wailsjs/go/launcher/ReaderWriter";
import {GetBackgroundImage} from "../wailsjs/go/launcher/MainMethod";
const darkNav = ref(DarkAndThemeToConst(dark_mode.value, theme_mode.value))
const dark = ref(dark_mode.value ? '#1a1a1acf' : '#e6e6e6cf')
const backImage = ref("")
watch(dark_mode, value => {
  dark.value = value ? '#1a1a1acf' : '#e6e6e6cf'
  darkNav.value = DarkAndThemeToConst(dark_mode.value, theme_mode.value)
})
watch(theme_mode, value => {
  darkNav.value = DarkAndThemeToConst(dark_mode.value, theme_mode.value)
})
onMounted(async () => {
  dark_mode.value = await ReadConfig(await GetConfigIniPath(), "Misc", "DarkMode") === "1"
  theme_mode.value = Number(await ReadConfig(await GetConfigIniPath(), "Misc", "ThemeMode"))
  let back = await GetBackgroundImage(-1)
  if (back.length != 0) {
    backImage.value = `url('data:image/${back[1]};base64,${back[0]}')`
  }else{
    backImage.value = ``
  }
  document.addEventListener("contextmenu", (e) => {
    e.preventDefault()
  })
})
</script>

<template>
  <div id="all">
    <NavBar id="nav-bar"/>
    <main id="main">
      <Body id="body"/>
    </main>
  </div>
  <MyDialog/>
</template>

<style scoped>
#all {
  position: absolute;
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
}
#nav-bar {
  position: absolute;
  width: 100%;
  height: 56px;
  background: v-bind(darkNav);
  background-size: 200%;
  animation: LineAni 30s linear infinite;
  z-index: 100;
}
#main {
  position: absolute;
  width: 100%;
  height: calc(100% - 56px);
  top: 56px;
  transition: all 0.2s;
  background-size: cover;
  background-repeat: no-repeat;
  background-position: 50% 50%;
  background-color: v-bind(dark);
  background-image: v-bind(backImage);
}
#body {
  position: absolute;
  width: 100%;
  height: calc(100%);
}
@keyframes LineAni {
  0% {
    background-position: 0 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 200% 50%;
  }
}
</style>
