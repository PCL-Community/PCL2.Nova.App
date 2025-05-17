<script setup lang="ts">
import {ref, watch} from 'vue'
import {b_button, b_content, b_level, b_resolve, b_show, b_show_all, b_title} from '../../logic/messagebox'
import {dark_mode} from '../../logic/changeBody'

const dark = ref(dark_mode.value ? '#1a1a1a' : '#f6f6f6')
const light = ref(dark_mode.value ? '#f6f6f6' : '#1a1a1a')
const hov = ref(dark_mode.value ? '#0a0a0a' : '#d6d6d6')
const bc = ref(dark_mode.value ? "#282828" : "aliceblue")
watch(dark_mode, value => {
  dark.value = value ? '#1a1a1a' : '#e6e6e6'
  light.value = value ? '#e6e6e6' : '#1a1a1a'
  hov.value = value ? '#0a0a0a' : '#d6d6d6'
  bc.value = value ? "#282828" : "aliceblue"
})
const m_font_color = ref("#3142b7")
const m_back_color = ref("#0000007f")
watch(b_level, value => {
  m_font_color.value = value == 0 ? "#3142b7" : value == 1 ? "#c7ad2a" : "#ff4c4c"
  m_back_color.value = value == 0 ? '#0000007f' : value == 1 ? '#7f7f007f' : '#7f00007f'
});
const m_resolve = ref(0)

function traLeave() {
  b_title.value = ""
  b_content.value = ""
  b_level.value = 0
  b_button.value = ["ok"]
  b_resolve.value!(m_resolve.value)
  b_show.value = false
  b_resolve.value = null
}

function buttonClick(index: number) {
  m_resolve.value = index
  b_show_all.value = false
}
</script>
<template>
  <transition name="opet">
    <div id="back" :class="b_show_all ? 'back-class' : 'back-class-hide'" v-if="b_show"></div>
  </transition>
  <transition name="slide" @after-leave="traLeave">
    <div v-if="b_show_all" class="content-box-class">
      <div id="content-title">{{ b_title }}</div>
      <div id="content" v-html="b_content"></div>
      <button class="return-button cursor-pointer" v-for="(b, i) in b_button" @click="buttonClick(i)">
        {{ b == "ok" ? "确认" : b == "cancel" ? "取消" : b == "yes" ? "是" : b == "no" ? "否" : b }}
      </button>
    </div>
  </transition>
</template>
<style scoped>
.opet-enter-active {
  animation: OpetIn 0.33s;
}

.opet-leave-active {
  animation: OpetOut 0.33s;
}

.slide-enter-active {
  animation: SlideIn 0.33s;
}

.slide-leave-active {
  animation: SlideOut 0.33s;
}

@keyframes OpetIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes OpetOut {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

@keyframes SlideIn {
  0% {
    opacity: 0;
    transform: translate(-50%, -20%) scale(0.9) rotate(-20deg);
  }
  80% {
    opacity: 0.8;
    transform: translate(-50%, -50%) scale(1.1);
  }
  100% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1) rotate(0deg);
  }
}

@keyframes SlideOut {
  0% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1) rotate(0deg);
  }
  20% {
    opacity: 0.8;
    transform: translate(-50%, -50%) scale(1.1);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -20%) scale(0.9) rotate(-20deg);
  }
}

#content-title {
  margin: 10px;
  font-size: 30px;
  font-weight: bold;
  border-bottom: 3px solid v-bind(m_font_color);
  padding: 5px;
  color: v-bind(m_font_color);
}

#content {
  margin: 10px;
  font-size: 16px;
  padding: 5px;
  color: v-bind(light)
}

#back {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1000;
}

.back-class {
  background-color: v-bind(m_back_color);
  backdrop-filter: blur(4px);
  transition: all 0.33s;
}

.back-class-hide {
  backdrop-filter: blur(0);
  background-color: rgba(0, 0, 0, 0);
  transition: all 0.33s;
}

.return-button {
  width: 60px;
  height: 30px;
  margin: 10px;
  float: right;
  background-color: v-bind(dark);
  border-radius: 6px;
  border: 1px solid v-bind(light);
  font-weight: bold;
  transition: all 0.2s;
  color: v-bind(light)
}

.return-button:hover {
  background-color: v-bind(hov);
}

.return-button:active {
  transform: scale(0.96);
}

.content-box-class {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  min-width: 200px;
  min-height: 50px;
  background-color: v-bind(bc);
  border-radius: 20px;
  box-shadow: 5px 5px 10px gray;
  border: 1px solid black;
  z-index: 1001;
}
</style>