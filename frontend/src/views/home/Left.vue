<script setup lang="ts">
import {ref, watch} from 'vue'
import AccountSelect from './account/AccountSelect.vue'
import MyNormalButton from '../../components/button/MyNormalButton.vue';
import {current_account_page} from '../../logic/changeBody';
import AddAccount from "./account/AddAccount.vue";

const isTransitioning = ref(true)

function account_leave() {
  isTransitioning.value = true
}

watch(current_account_page, () => {
  isTransitioning.value = false
})
</script>
<template>
  <div style="display: flex; flex-direction: column;">
    <div id="middle">
      <transition name="account" @after-leave="account_leave">
        <AccountSelect v-if="current_account_page && isTransitioning" class="account-style"/>
      </transition>
      <transition name="account" @after-leave="account_leave">
        <AddAccount v-if="!current_account_page && isTransitioning" class="account-style"/>
      </transition>
    </div>
    <div id="bottom">
      <div id="launch">
        <MyNormalButton id="launch-button" :isDisabled="!current_account_page">
          <span id="launch-title">启动游戏</span>
          <br>
          <span id="launch-version">测试客户端</span>
        </MyNormalButton>
      </div>
      <div id="setting">
        <MyNormalButton class="core-button" :isDisabled="!current_account_page">选择核心</MyNormalButton>
        <MyNormalButton class="core-button" :isDisabled="!current_account_page">核心设置</MyNormalButton>
      </div>
    </div>
  </div>
</template>

<style scoped>
.account-style {
  width: 100%;
  height: 100%;
}
#middle {
  width: 100%;
  flex: 1;
}

#bottom {
  width: 100%;
  height: 150px;
  flex-shrink: 0;
}

#launch {
  width: 100%;
  height: 86px;
}

#setting {
  width: 100%;
  height: 64px;
}

#launch-button {
  margin-top: 6px;
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
.account-enter-active {
  animation: accountSlideIn 0.2s;
}

.account-leave-active {
  animation: accountSlideOut 0.2s;
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