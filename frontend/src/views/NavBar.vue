<script setup lang="ts">
import {current_view, dark_mode} from '../logic/changeBody'
import {Quit, WindowMinimise} from '../../wailsjs/runtime'
import {ref, watch} from 'vue'

function changeButtonState(name: string) {
  current_view.value = name
}

const light = ref(dark_mode.value ? '#e6e6e6' : '#1a1a1a')
const dark = ref(dark_mode.value ? '#1a1a1a' : '#e6e6e6')
const hov = ref(dark_mode.value ? '#0a0a0a' : '#d6d6d6')
watch(dark_mode, value => {
  light.value = value ? '#e6e6e6' : '#1a1a1a'
  dark.value = value ? '#1a1a1a' : '#e6e6e6'
  hov.value = value ? '#0a0a0a' : '#d6d6d6'
})
</script>
<template>
  <div id="all-nav" style="--wails-draggable: drag">
    <div id="icon-title" class="font-pcl">
      PCL II
      <div class="icon-next back-next font-pcl">Nova Imitation</div>
      <div class="icon-next back-dev font-pcl">Dev</div>
    </div>
    <div id="nav-button-group">
      <button :class="current_view == 'home' ? 'active-button' : ('nav-button cursor-pointer')"
              @click="changeButtonState('home')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" :class="current_view == 'home' ? 'active-button-icon' : 'nav-button-icon'">
          <path
              d="M12 2L12 11M18.363961 5.63603897C21.8786797 9.15075759 21.8786797 14.8492424 18.363961 18.363961 14.8492424 21.8786797 9.15075759 21.8786797 5.63603897 18.363961 2.12132034 14.8492424 2.12132034 9.15075759 5.63603897 5.63603897"/>
        </svg>
        主页
      </button>
      <button :class="current_view == 'download' ? 'active-button' : ('nav-button cursor-pointer')"
              @click="changeButtonState('download')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" :class="current_view == 'download' ? 'active-button-icon' : 'nav-button-icon'">
          <path d="M12,3 L12,16"/>
          <polyline points="7 12 12 17 17 12"/>
          <path d="M20,21 L4,21"/>
        </svg>
        下载
      </button>
      <button :class="current_view == 'online' ? 'active-button' : ('nav-button cursor-pointer')"
              @click="changeButtonState('online')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" :class="current_view == 'online' ? 'active-button-icon' : 'nav-button-icon'">
          <path
              d="M21.2279221 11.0355339C16.2377345 6.04534632 8.14704183 6.04534632 3.15685425 11.0355339M17.6137085 14.6497475C14.6195959 11.6556349 9.76518036 11.6556349 6.77106781 14.6497475M13.9994949 18.263961C13.0014574 17.2659235 11.3833189 17.2659235 10.3852814 18.263961"/>
        </svg>
        联机
      </button>
      <button :class="current_view == 'setting' ? 'active-button' : ('nav-button cursor-pointer')"
              @click="changeButtonState('setting')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" :class="current_view == 'setting' ? 'active-button-icon' : 'nav-button-icon'">
          <path
              d="M5.03506429,12.7050339 C5.01187484,12.4731696 5,12.2379716 5,12 C5,11.7620284 5.01187484,11.5268304 5.03506429,11.2949661 L3.20577137,9.23205081 L5.20577137,5.76794919 L7.9069713,6.32070904 C8.28729123,6.0461342 8.69629298,5.80882212 9.12862533,5.61412402 L10,3 L14,3 L14.8713747,5.61412402 C15.303707,5.80882212 15.7127088,6.0461342 16.0930287,6.32070904 L18.7942286,5.76794919 L20.7942286,9.23205081 L18.9649357,11.2949661 C18.9881252,11.5268304 19,11.7620284 19,12 C19,12.2379716 18.9881252,12.4731696 18.9649357,12.7050339 L20.7942286,14.7679492 L18.7942286,18.2320508 L16.0930287,17.679291 C15.7127088,17.9538658 15.303707,18.1911779 14.8713747,18.385876 L14,21 L10,21 L9.12862533,18.385876 C8.69629298,18.1911779 8.28729123,17.9538658 7.9069713,17.679291 L5.20577137,18.2320508 L3.20577137,14.7679492 L5.03506429,12.7050339 Z"/>
          <circle cx="12" cy="12" r="1"/>
        </svg>
        设置
      </button>
      <button :class="current_view == 'about' ? 'active-button' : ('nav-button cursor-pointer')"
              @click="changeButtonState('about')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" :class="current_view == 'about' ? 'active-button-icon' : 'nav-button-icon'">
          <path
              d="M9 4C9 2.89543 9.89543 2 11 2C12.1046 2 13 2.89543 13 4V6H18V11H20C21.1046 11 22 11.8954 22 13C22 14.1046 21.1046 15 20 15H18V20H13V18C13 16.8954 12.1046 16 11 16C9.89543 16 9 16.8954 9 18V20H4V15H6C7.10457 15 8 14.1046 8 13C8 11.8954 7.10457 11 6 11H4V6H9V4Z"/>
        </svg>
        更多
      </button>
    </div>
    <div id="icon-control">
      <button class="icon-control-button cursor-pointer" @click="WindowMinimise()">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke="#000000"
            stroke-width="4"
            fill="none" class="nav-control-button-icon">
          <path d="M20,12 L4,12"/>
        </svg>
      </button>
      <button class="icon-control-button cursor-pointer" @click="Quit()">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke="#000000"
            stroke-width="4"
            fill="none" class="nav-control-button-icon">
          <path d="M6.34314575 6.34314575L17.6568542 17.6568542M6.34314575 17.6568542L17.6568542 6.34314575"/>
        </svg>
      </button>
    </div>
  </div>
</template>
<style scoped>
#all-nav {
  display: flex;
  z-index: 999;
}

#icon-control {
  margin-top: 14px;
  display: flex;
  margin-right: 20px;
  float: right;
}

.icon-control-button {
  border-radius: 50%;
  width: 30px;
  height: 30px;
  border: 0;
  background-color: transparent;
  margin-right: 10px;
  transition: all 0.2s;
  --wails-draggable: no-drag;
}

.nav-control-button-icon {
  width: 24px;
  height: 24px;
  vertical-align: middle;
  stroke: white;
}

#icon-title {
  margin-top: 9px;
  display: flex;
  margin-left: 24px;
  float: left;
  font-size: 25px;
  color: white;
  vertical-align: middle;
}

#nav-button-group {
  position: absolute;
  margin: 0 auto;
  left: 105px;
  right: 0;
  top: 10px;
  width: 502px;
  height: 36px;
  --wails-draggable: no-drag;
  vertical-align: middle;
}

.nav-button {
  border: 0;
  background-color: transparent;
  border-radius: 15px;
  width: 90px;
  height: 30px;
  margin-top: 3px;
  font-size: 14px;
  font-weight: bold;
  color: white;
  transition: all, 0.2s;
}

.active-button {
  border: 1px solid lightgray;
  background-color: v-bind(dark);
  border-radius: 15px;
  width: 90px;
  height: 30px;
  margin-top: 3px;
  font-size: 14px;
  font-weight: bold;
  color: v-bind(light);
  box-shadow: 0 1px 3px lightgray;
  transition: all, 0.2s;
}

.active-button:hover {
  background-color: v-bind(hov);
}

.nav-button:hover,
.icon-control-button:hover {
  background-color: rgba(0, 0, 0, 0.2);
  box-shadow: 0 1px 3px gray;
  border: 1px solid lightgray;
}

.nav-button .nav-button-icon,
.active-button .active-button-icon {
  width: 16px;
  height: 16px;
  vertical-align: middle;
  stroke: white;
  margin-right: 3px;
  transition: all 0.2s;
}

.nav-button .active-button-icon,
.active-button .active-button-icon {
  width: 16px;
  height: 16px;
  vertical-align: middle;
  stroke: v-bind(light);
  margin-right: 3px;
  transition: all 0.2s;
}

.nav-button:nth-child(2),
.active-button:nth-child(2),
.nav-button:nth-child(3),
.active-button:nth-child(3),
.nav-button:nth-child(4),
.active-button:nth-child(4),
.nav-button:nth-child(5),
.active-button:nth-child(5)
{
  margin-left: 13px;
}

.icon-next {
  border-radius: 6px;
  box-shadow: 0 0 60px gray;
  padding: 6px;
  margin-top: 6px;
  margin-left: 12px;
  font-size: 14px;
  font-weight: bold;
  text-align: center;
  line-height: 10px;
  float: left;
  color: black;
  height: 12px;
  vertical-align: middle;
}

.back-next {
  background-color: lightgray;
}

.back-dev {
  background-color: orange;
}

</style>