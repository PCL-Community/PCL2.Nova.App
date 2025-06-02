<script setup lang="ts">

import {current_account} from "../../../logic/changeBody";
import ThirdParty from "./ThirdParty.vue";
import Offline from "./Offline.vue";
import MyCheckButton from "../../../components/button/MyCheckButton.vue";
import Microsoft from "./Microsoft.vue";
import {ref} from "vue";
import MyNormalButton from "../../../components/button/MyNormalButton.vue";

const isTransitioning = ref(true)

function account_leave() {
  isTransitioning.value = true
}
function account_click(account: string) {
  if (account == current_account.value) {
    return
  }
  isTransitioning.value = false
  current_account.value = account
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
      <MyCheckButton :isChecked="current_account == 'ThirdParty'"
                     :class="current_account == 'ThirdParty' ? 'login-button-active' : ('login-button-style cursor-pointer')"
                     @click="account_click('ThirdParty')">
        <svg
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            fill="none" :class="current_account == 'ThirdParty' ? 'login-button-icon-active' : 'login-button-icon'">
          <path d="M1 18C1 15.75 4 15.75 5.5 14.25 6.25 13.5 4 13.5 4 9.75 4 7.25025 4.99975 6 7 6 9.00025 6 10 7.25025 10 9.75 10 13.5 7.75 13.5 8.5 14.25 10 15.75 13 15.75 13 18M12.7918114 15.7266684C13.2840551 15.548266 13.6874862 15.3832994 14.0021045 15.2317685 14.552776 14.9665463 15.0840574 14.6659426 15.5 14.25 16.25 13.5 14 13.5 14 9.75 14 7.25025 14.99975 6 17 6 19.00025 6 20 7.25025 20 9.75 20 13.5 17.75 13.5 18.5 14.25 20 15.75 23 15.75 23 18"/> <path stroke-linecap="round" d="M12,16 C12.3662741,15.8763472 12.6302112,15.7852366 12.7918114,15.7266684"/>
        </svg>
        外置
      </MyCheckButton>
    </div>
    <div id="middle">
      <transition name="account" @after-leave="account_leave">
        <Microsoft v-if="current_account == 'Microsoft' && isTransitioning" class="account-style"/>
      </transition>
      <transition name="account" @after-leave="account_leave">
        <Offline v-if="current_account == 'Offline' && isTransitioning" class="account-style"/>
      </transition>
      <transition name="account" @after-leave="account_leave">
        <ThirdParty v-if="current_account == 'ThirdParty' && isTransitioning" class="account-style"/>
      </transition>
    </div>
  </div>
</template>

<style scoped>
.account-style {
  width: 100%;
  height: 100%;
}
#top {
  width: calc(100% - 30px);
  margin-left: 15px;
  display: flex;
  flex-direction: row;
  justify-content: space-around;
}
#middle {
  width: 100%;
  flex: 1;
}
.login-button-style,
.login-button-active {
  width: 80px;
  height: 30px;
  margin: 32px 0 0 0;
}

.login-button-style:first-child,
.login-button-active:first-child{
  margin-right: 16px;
}

.login-button-style:last-child,
.login-button-active:last-child{
  margin-left: 16px;
}

.login-button-icon-active,
.login-button-icon {
  width: 16px;
  height: 16px;
  vertical-align: middle;
  margin-right: 3px;
  transition: all 0.2s;
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