<script setup lang="ts">
import MySidebarButton from '../../components/button/MySidebarButton.vue'
import {current_download} from '../../logic/changeBody'

function clickSideButton(target: string) {
  current_download.value = target
}
</script>
<template>
  <div style="display: flex; flex-direction: column;">
    <details class="grid ani1" open>
      <summary>Minecraft 客户端</summary>
    </details>
    <div>
      <MySidebarButton :isChecked="current_download == 'Auto-Install'" @click="clickSideButton('Auto-Install')">
        <svg
            role="img"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" class="side-icon">
          <path d="M3 5H21V21H3V5Z"/>
          <path d="M21 9H3"/>
          <path d="M7 5V3"/>
          <path d="M17 5V3"/>
          <path d="M15 15H9"/>
          <path d="M12 12V18"/>
        </svg>
        自动安装
      </MySidebarButton>
      <MySidebarButton :isChecked="current_download == 'Manual-Install'" @click="clickSideButton('Manual-Install')">
        <svg
            viewBox="0 0 24 24"
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" class="side-icon">
          <path
              d="M9.74292939,13.7429294 C9.19135019,13.9101088 8.60617271,14 8,14 C4.6862915,14 2,11.3137085 2,8 C2,7.07370693 2.20990431,6.19643964 2.58474197,5.4131691 L6.94974747,9.77817459 L9.77817459,6.94974747 L5.4131691,2.58474197 C6.19643964,2.20990431 7.07370693,2 8,2 C11.3137085,2 14,4.6862915 14,8 C14,8.88040772 13.8103765,9.71652648 13.4697429,10.4697429 L20.5858636,17.5858636 C21.3669122,18.3669122 21.3669122,19.6332422 20.5858636,20.4142907 L19.9142907,21.0858636 C19.1332422,21.8669122 17.8669122,21.8669122 17.0858636,21.0858636 L9.74292939,13.7429294 Z"/>
        </svg>
        手动安装
      </MySidebarButton>
    </div>
    <details class="grid ani2" open>
      <summary>附加内容</summary>
    </details>
    <div>
      <MySidebarButton :isChecked="current_download == 'Extension-Mod'" @click="clickSideButton('Extension-Mod')">
        <svg
            role="img"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" class="side-icon">
          <path
              d="M9 4C9 2.89543 9.89543 2 11 2C12.1046 2 13 2.89543 13 4V6H18V11H20C21.1046 11 22 11.8954 22 13C22 14.1046 21.1046 15 20 15H18V20H13V18C13 16.8954 12.1046 16 11 16C9.89543 16 9 16.8954 9 18V20H4V15H6C7.10457 15 8 14.1046 8 13C8 11.8954 7.10457 11 6 11H4V6H9V4Z"/>
        </svg>
        Mod
      </MySidebarButton>
      <MySidebarButton :isChecked="current_download == 'Extension-Modpack'"
                       @click="clickSideButton('Extension-Modpack')">
        <svg
            role="img"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" class="side-icon">
          <path d="M3 13H9V16H15V13H21"/>
          <path fill-rule="evenodd" clip-rule="evenodd" d="M3 13L5 4H19L21 13V20H3V13Z"/>
          <path d="M7 7H17"/>
          <path d="M6.5 10H17.5"/>
        </svg>
        整合包
      </MySidebarButton>
      <MySidebarButton :isChecked="current_download == 'Extension-Resourcepack'"
                       @click="clickSideButton('Extension-Resourcepack')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" class="side-icon">
          <path d="M3 10H9V13H15V10H21"/>
          <path d="M3 10L6 4H18L21 10V20H3V10Z"/>
        </svg>
        资源包
      </MySidebarButton>
      <MySidebarButton :isChecked="current_download == 'Extension-Shaderpack'"
                       @click="clickSideButton('Extension-Shaderpack')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" class="side-icon">
          <path
              d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"/>
          <g stroke-linecap="round">
            <path d="M3 16H14.3164"/>
            <path d="M4.03589 6.20575L9.68257 15.9861"/>
            <path d="M13.0359 2.20575L7.37891 12.004"/>
            <path d="M10.9641 21.7942L16.6146 12.0074"/>
            <path d="M19.9641 17.7942L14.3086 7.99866"/>
            <path d="M21 7.98721H9.71844"/>
          </g>
        </svg>
        光影包
      </MySidebarButton>
    </div>
  </div>
</template>
<style scoped>
.grid {
  margin-top: 5px;
}

.grid summary {
  /* list-style: none; */
  font-size: 12px;
  color: gray;
  margin-bottom: 7px;
  margin-left: 20px;
}

.grid:nth-child(2) {
  margin-top: 30px;
}

.grid + div {
  max-height: 0;
  transition: max-height 0.5s;
  margin: 0 0 1rem;
  overflow: hidden;
}

.ani1[open] + div {
  max-height: 80px;
}

.ani2[open] + div {
  max-height: 160px;
}
</style>
<script lang="ts">
export default {
  name: 'downloadLeft'
}
</script>