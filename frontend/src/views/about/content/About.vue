<script setup lang="ts">
import MySelectCard from '../../../components/card/MySelectCard.vue'
import MyNormalLabel from "../../../components/input/MyNormalLabel.vue";
import MyNormalButton from "../../../components/button/MyNormalButton.vue";
import {OpenCustomURL} from "../../../logic/functions";
</script>
<template>
  <div style="display: flex; flex-direction: column;">
    <MySelectCard :maxHeight="130" :isExpand="false" :title="'关于'" :isLast="false">
      <div style="margin: 10px">
        <div class="row">
          <img src="../../../assets/images/Heads/xphost.jpg" alt="现作者头像">
          <div class="column">
            <MyNormalLabel class="normal-label">xphost008</MyNormalLabel>
            <p style="font-size: 13px; color: #606060">Plain Craft Launcher II: Nova 的现作者</p>
          </div>
        </div>
        <div class="row">
          <img src="../../../assets/images/PCLNova.Dev.png" alt="Nova Logo" style="border-radius: 0">
          <div class="column">
            <MyNormalLabel class="normal-label">Plain Craft Launcher II: Nova</MyNormalLabel>
            <p style="font-size: 13px; color: #606060">当前版本：Dev 0.0.10</p>
          </div>
          <MyNormalButton class="afdian" @click="OpenCustomURL('https://github.com/PCL-Community/PCL2.Nova.App')">给项目点 Star</MyNormalButton>
        </div>
      </div>
    </MySelectCard>
    <MySelectCard :maxHeight="130" :isExpand="false" :title="'鸣谢'" :isLast="false">
      <div style="margin: 10px">
        <div class="row">
          <img src="../../../assets/images/Heads/ltcatt.jpg" alt="PCL作者头像">
          <div class="column">
            <MyNormalLabel class="normal-label">龙腾猫跃</MyNormalLabel>
            <p style="font-size: 13px; color: #606060">Plain Craft Launcher II 的原作者</p>
          </div>
          <MyNormalButton class="afdian" @click="OpenCustomURL('https://afdian.com/a/LTCat')">赞助原作者</MyNormalButton>
        </div>
        <div class="row">
          <img src="../../../assets/images/Heads/moyuan.jpg" alt="原作者头像">
          <div class="column">
            <MyNormalLabel class="normal-label">墨渊</MyNormalLabel>
            <p style="font-size: 13px; color: #606060">Plain Craft Launcher II: Nova 的原作者</p>
          </div>
          <MyNormalButton class="afdian" @click="OpenCustomURL('https://afdian.com/a/NEXORA-Studios')">赞助原作者</MyNormalButton>
        </div>
      </div>
    </MySelectCard>
    <MySelectCard :maxHeight="160" :isExpand="true" :title="'版权'" :isLast="false">
      <div style="margin: 10px; padding: 0 30px">
        <MyNormalLabel class="normal-label">PCL2.Nova.App 开源项目及其官方衍生程序均遵循 GNU General Public License 3.0 (GPLv3) 协议开源。本项目主导社区为 PCL-Community，任何人都可以提 Pull Request！</MyNormalLabel><br><br>
        <MyNormalButton class="normal-button" @click="OpenCustomURL('https://github.com/PCL-Community/PCL2.Nova.App')">查看源代码</MyNormalButton>&nbsp;&nbsp;&nbsp;
        <MyNormalButton class="normal-button" @click="OpenCustomURL('https://www.gnu.org/licenses/gpl-3.0.html')">查看许可证</MyNormalButton>
      </div>
    </MySelectCard>
    <MySelectCard :maxHeight="300" :isExpand="true" :title="'许可与版权声明'" :isLast="true">
      <div style="margin: 10px; padding-left: 30px">
        <MyNormalLabel class="normal-label">PCL2.Nova.App</MyNormalLabel>
        <p class="normal-p">GNU GPL v3 <code data="url">https://github.com/PCL-Community/PCL2.Nova.App</code></p>
        <MyNormalLabel class="normal-label">Wails</MyNormalLabel>
        <p class="normal-p">MIT <code data="url">https://github.com/wailsapp/wails</code></p>
        <MyNormalLabel class="normal-label">Golang</MyNormalLabel>
        <p class="normal-p">BSD 3 <code data="url">https://github.com/golang/go</code></p>
        <MyNormalLabel class="normal-label">Vue.js</MyNormalLabel>
        <p class="normal-p">MIT <code data="url">https://github.com/vuejs/core</code></p>
        <MyNormalLabel class="normal-label">TypeScript</MyNormalLabel>
        <p class="normal-p">Apache License 2.0 <code data="url">https://github.com/microsoft/TypeScript</code></p>
      </div>
    </MySelectCard>
  </div>
</template>
<style scoped>
.row {
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 50px;
  margin: 10px 0;
  font-family: PCL, 'Microsoft JhengHei', serif;
  position: relative;
}
.row .afdian {
  position: absolute;
  top: 5px;
  bottom: 5px;
  right: 20px;
  width: 120px;
  height: 40px;
}
.row img {
  margin-left: 30px;
  margin-right: 10px;
  border-radius: 50%;
  width: 50px;
  height: 50px;
}
.column {
  display: flex;
  flex-direction: column;
}
.normal-label {
  font-size: 18px;
  font-weight: bold;
  font-family: PCL, 'Microsoft JhengHei', serif;
}
.normal-button {
  font-size: 14px;
  width: 150px;
  height: 40px;
}
.normal-p {
  font-size: 14px;
  color: #606060;
}
code[data="url"] {
  user-select: all;
  font-family: PCL, 'Microsoft JhengHei', serif;
  color: black;
  padding: 2px 0.4em;
  background-color: rgb(240, 240, 240);
  border-radius: 0.3em;
  font-size: 0.9em;
  margin: 0 0.2em;
  box-shadow: 0 0 3px black;
  position: relative;
  top: -1px;
}
</style>