<script setup lang="ts">
import NavBar from './views/NavBar.vue';
import Body from './views/Body.vue'
import {dark_mode} from './logic/changeBody'
import {onMounted, ref, watch} from 'vue'
import MyDialog from './components/card/MyDialog.vue'

const dark = ref(dark_mode.value ? '#1a1a1a' : '#e6e6e6')
watch(dark_mode, value => dark.value = value ? '#1a1a1a' : '#e6e6e6')
onMounted(() => {
  document.addEventListener("contextmenu", (e) => {
    e.preventDefault()
  })
})
</script>

<template>
  <div id="all">
    <NavBar id="nav-bar" data-tauri-drag-region/>
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
  background: linear-gradient(to right, rgb(19, 85, 206), cyan);
  z-index: 100;
}

#main {
  position: absolute;
  width: 100%;
  height: calc(100% - 56px);
  top: 56px;
  transition: all 0.2s;
  background-color: v-bind(dark);
}

#body {
  position: absolute;
  width: 100%;
  height: calc(100%);
}
</style>