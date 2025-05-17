<script setup lang="ts">
import {ref} from 'vue'
import Microsoft from './account/Microsoft.vue'
import Offline from './account/Offline.vue'
import MyCheckButton from '../../components/button/MyCheckButton.vue';
import MyNormalButton from '../../components/button/MyNormalButton.vue';
import {current_account} from '../../logic/changeBody';

const isTransitioning = ref(true)

function account_leave() {
  isTransitioning.value = true
}

function account_click(target: string) {
  if (target == current_account.value) {
    return
  }
  isTransitioning.value = false
  current_account.value = target
}
</script>
<template>
  <div style="display: flex; flex-direction: column;">
    <div id="top">
      <MyCheckButton :isChecked="current_account == 'Microsoft'"
                     :class="current_account == 'Microsoft' ? 'login-button-active' : ('login-button-style cursor-pointer')"
                     @click="account_click('Microsoft')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="2"
            stroke-linecap="square"
            stroke-linejoin="miter"
            fill="none" :class="current_account == 'Microsoft' ? 'login-button-icon-active' : 'login-button-icon'">
          <path
              d="M19,14.7368421 C19,17.122807 16.6666667,19.2105263 12,21 C7.33333333,19.2105263 5,17.122807 5,14.7368421 C5,12.3508772 5,9.36842105 5,5.78947368 C8.13611482,4.59649123 10.4694481,4 12,4 C13.5305519,4 15.8638852,4.59649123 19,5.78947368 C19,9.36842105 19,12.3508772 19,14.7368421 Z"/>
        </svg>
        正版
      </MyCheckButton>
      <MyCheckButton :isChecked="current_account == 'Offline'"
                     :class="current_account == 'Offline' ? 'login-button-active' : ('login-button-style cursor-pointer')"
                     @click="account_click('Offline')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="2"
            stroke-linecap="square"
            stroke-linejoin="miter"
            fill="none" :class="current_account == 'Offline' ? 'login-button-icon-active' : 'login-button-icon'">
          <path
              d="M7.93517 13.7796L15.1617 6.55304C16.0392 5.67631 17.4657 5.67631 18.3432 6.55304C19.2206 7.43052 19.2206 8.85774 18.3432 9.73522L8.40091 19.5477C6.9362 21.0124 4.56325 21.0124 3.09854 19.5477C1.63382 18.0837 1.63382 15.7093 3.09854 14.2453L12.9335 4.53784C14.984 2.48739 18.3094 2.48739 20.3569 4.53784C22.4088 6.58904 22.4088 9.91146 20.3584 11.9619L13.239 19.082"/>
        </svg>
        离线
      </MyCheckButton>
    </div>
    <div id="middle">
      <transition name="account" @after-leave="account_leave">
        <Microsoft v-if="current_account == 'Microsoft' && isTransitioning" class="account-style"/>
      </transition>
      <transition name="account" @after-leave="account_leave">
        <Offline v-if="current_account == 'Offline' && isTransitioning" class="account-style"/>
      </transition>
    </div>
    <div id="bottom">
      <div id="launch">
        <MyNormalButton id="launch-button">
          <span id="launch-title">启动游戏</span>
          <br>
          <span id="launch-version">测试客户端</span>
        </MyNormalButton>
      </div>
      <div id="setting">
        <MyNormalButton class="core-button">选择核心</MyNormalButton>
        <MyNormalButton class="core-button">核心设置</MyNormalButton>
      </div>
    </div>
  </div>
</template>

<style scoped>
.account-style {
  width: 100%;
  height: 100%;
}

.account-enter-active {
  animation: accountSlideIn 0.2s;
}

.account-leave-active {
  animation: accountSlideOut 0.2s;
}

#top {
  width: 100%;
  height: 100px;
}

#middle {
  width: 100%;
  flex: 1;
}

#bottom {
  width: 100%;
  height: 200px;
}

#launch {
  width: 100%;
  height: 126px;
}

#setting {
  width: 100%;
  height: 74px;
}

#launch-button {
  margin-top: 46px;
  margin-left: 26px;
  height: 75px;
  width: calc(100% - 52px);
  border: 1px solid rgb(0, 191, 255);
}

.core-button {
  width: calc(50% - 31px);
  height: 40px;
  margin-top: 2px;
}

.core-button:nth-child(1) {
  margin-left: 26px;
}

.core-button:nth-child(2) {
  margin-left: 10px;
}

#launch-title {
  background: linear-gradient(to right, rgb(63, 207, 255), rgb(96, 96, 255));
  font-weight: normal;
  color: transparent;
  background-clip: text;
  font-size: 20px;
}

#launch-version {
  font-weight: bold;
  font-size: 12px;
}

.login-button-style {
  width: 90px;
  height: 30px;
  margin-top: 32px;
}

.login-button-active {
  width: 90px;
  height: 30px;
  margin-top: 32px;
}

.login-button-icon-active {
  width: 16px;
  height: 16px;
  vertical-align: middle;
  margin-right: 3px;
  transition: all 0.2s;
}

.login-button-icon {
  width: 16px;
  height: 16px;
  vertical-align: middle;
  margin-right: 3px;
  transition: all 0.2s;
}

.login-button-style:nth-child(1),
.login-button-active:nth-child(1) {
  margin-left: calc(50% - 100px);
}

.login-button-style:nth-child(2),
.login-button-active:nth-child(2) {
  margin-left: 30px;
}

@keyframes accountSlideIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes accountSlideOut {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

</style>

<script lang="ts">
export default {
  name: 'homeLeft'
}
</script>