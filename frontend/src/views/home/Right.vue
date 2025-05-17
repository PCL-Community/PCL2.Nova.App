<script setup lang="ts">
import {ref} from 'vue'
import MyLoading from '../../components/card/MyLoading.vue'
import MyNormalButton from '../../components/button/MyNormalButton.vue'
import {messagebox} from '../../logic/messagebox'
import MyProgressBar from "../../components/input/MyProgressBar.vue";
import {EventsOn} from "../../../wailsjs/runtime";
import {StartDownload} from "../../../wailsjs/go/main/App";

const prog = ref(0)
const stop = ref(0)
const isLock = ref(false)

async function test_dialog() {
  console.log("召唤信息框之前")
  let miao = await messagebox('信息测试', '这是一个信息测试', 0, ['ok', 'no'])
  console.log("召唤信息测试之后，你点击了：" + (miao == 0 ? "ok" : "cancel"))
  let miao2 = await messagebox('警告测试', '这是一个<br>警告测试<br>换行测试', 1, ['ok', 'no', 'yes'])
  console.log("召唤警告测试之后，你点击了：" + (miao2 == 0 ? "ok" : miao2 == 1 ? "no" : "yes"))
  let miao3 = await messagebox('错误测试', '这是一个错误测试', 2, ['ok', 'no'])
  console.log("召唤错误测试之后，你点击了：" + (miao3 == 0 ? "ok" : "no"))
  let miao4 = await messagebox('多按钮测试', '这是一个多按钮测试', 0, ['ok', 'ok', 'ok', 'ok', 'ok', 'ok'])
  console.log("召唤错误测试之后，你点击了：" + miao4)
}

EventsOn('download_progress', (progress) => {
  prog.value = progress
})
EventsOn('download_success', () => {
  stop.value = 2
  isLock.value = false
})

async function download_start() {
  if (!isLock.value) {
    isLock.value = true
    stop.value = 0
    prog.value = 0
    await StartDownload()
  }
}
</script>
<template>
  <div style="position: absolute;">
    <MyProgressBar :max-value="100" :current-value="prog" width="100%" height="20px"/>
    <MyLoading :loading_text="stop == 1 ? '加载失败' + prog + '%' : stop == 0 ? '正在加载' + prog + '%' : '加载成功' + prog + '%'" :state="stop"
               class="ala"/>
    <MyNormalButton id="test-button" @click="download_start">{{ "开始" }}</MyNormalButton>
    <MyNormalButton id="test-dialog" @click="test_dialog">点我测试信息框</MyNormalButton>
  </div>
</template>
<style scoped>
.ala {
  position: absolute;
  margin: auto;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
  width: 200px;
  height: 200px;
}

#test-button {
  position: absolute;
  margin: auto;
  left: 0;
  top: 260px;
  right: 0;
  bottom: 0;
  width: 200px;
  height: 40px;
}

#test-dialog {
  position: absolute;
  margin: auto;
  left: 0;
  top: 380px;
  right: 0;
  bottom: 0;
  width: 200px;
  height: 40px;
}
</style>
<script lang="ts">
export default {
  name: "homeRight"
}
</script>