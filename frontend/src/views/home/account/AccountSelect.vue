<script setup lang="ts">
import MyNormalButton from "../../../components/button/MyNormalButton.vue";
import {AccountPart, current_account_page} from "../../../logic/changeBody";
import {current_select_account} from "../../../logic/changeBody";
import MyNormalLabel from "../../../components/input/MyNormalLabel.vue";
import {GetAccountConfig, SetAccountConfig} from "../../../../wailsjs/go/launcher/Account";
import {onMounted} from "vue";
import {GetOtherIniPath, ReadConfig, WriteConfig} from "../../../../wailsjs/go/launcher/ReaderWriter";
async function selectAccount(index: number) {
  if(current_select_account.value != index) {
    current_select_account.value = index
    await WriteConfig(await GetOtherIniPath(), "Account", "SelectAccount", index.toString())
  }
}
async function deleteAccount(index: number) {
  if(index < current_select_account.value) {
    current_select_account.value -= 1
  }else if(index == current_select_account.value) {
    current_select_account.value = -1
  }
  AccountPart.accounts.splice(index, 1)
  await SetAccountConfig(AccountPart)
}
onMounted(async () => {
  Object.assign(AccountPart.accounts, (await GetAccountConfig()).accounts)
  let index = await ReadConfig(await GetOtherIniPath(), "Account", "SelectAccount")
  current_select_account.value = Number(index)
})
</script>
<template>
  <div id="main-style">
    <div id="center-style">
      <svg
          role="img"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
          fill="none" id="nologin-img"
          v-if="AccountPart.accounts.length <= 0">
        <path
            d="M4,20 C4,17 8,17 10,15 C11,14 8,14 8,9 C8,5.667 9.333,4 12,4 C14.667,4 16,5.667 16,9 C16,14 13,14 14,15 C16,17 20,17 20,20"/>
      </svg>
      <br>
      <div id="all-account">
        <template v-for="(atype, index) in AccountPart.accounts" :key="index">
          <div class="a-account" :title="'玩家 ID: ' + atype.name + '\nUUID: ' + atype.uuid" @click="selectAccount(index)" :style="'border: ' + (index === current_select_account ? '1px solid skyblue' : '0') + ';'">
            <img :src="'data:image/png;base64,' + atype.head_skin" alt="头像" class="a-avatar">
            <div>
              <MyNormalLabel class="a-name">{{atype.name}}</MyNormalLabel><br>
              <MyNormalLabel class="a-uuid">{{atype.type == 'Offline' ? '离线登录' : atype.type == 'Microsoft' ? '正版登录' : '第三方登录'}}</MyNormalLabel>
            </div>
            <MyNormalLabel class="a-delete cursor-pointer" @click.stop="deleteAccount(index)"><svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke-linecap="round"
                stroke-width="1.5" style="">
              <path
                  d="M9.17 4a3.001 3.001 0 0 1 5.66 0m5.67 2h-17m15.333 2.5l-.46 6.9c-.177 2.654-.265 3.981-1.13 4.79s-2.196.81-4.856.81h-.774c-2.66 0-3.991 0-4.856-.81c-.865-.809-.954-2.136-1.13-4.79l-.46-6.9M9.5 11l.5 5m4.5-5l-.5 5"/>
            </svg></MyNormalLabel>
          </div>
        </template>
      </div>
      <br>
      <div id="control">
        <MyNormalButton class="add-account" @click="current_account_page = false">添加新账号</MyNormalButton>
        <MyNormalButton class="add-account">修改选中账号</MyNormalButton>
      </div>
    </div>
  </div>
</template>
<style scoped>
#main-style {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
#all-account {
  max-height: 260px;
  overflow-y: auto;
  overflow-x: hidden;
  width: calc(100% - 100px);
  display: flex;
  flex-direction: column;
  align-items: center;
}
#center-style {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
#nologin-img {
  stroke: rgb(0, 186, 254);
  width: 80px;
  height: 80px;
}
.a-account {
  transition: all 0.2s;
  border-radius: 10px;
  height: 50px;
  width: calc(100% - 2px);
  flex-shrink: 0;
  box-sizing: border-box;
  margin-bottom: 5px;
  display: flex;
  position: relative;
}
.a-account:last-child {
  margin-bottom: 0;
}
.a-account:hover {
  background-color: rgba(128, 128, 128, 0.5);
}
.a-account:active {
  transform: scale(98%);
}
.a-avatar {
  width: 40px;
  height: 40px;
  image-rendering: pixelated;
  border-radius: 10px;
  box-shadow: 0 0 5px gray;
  margin-left: 10px;
  position: relative;
  top: 4px;
}
.a-delete {
  position: absolute;
  top: 10px;
  width: 20px;
  height: 20px;
  padding: 6px;
  right: 10px;
  transition: all 0.2s;
  border-radius: 50%;
}
.a-delete:hover {
  background-color: rgba(0, 0, 0, 0.2);
  stroke: red;
}
.a-name {
  font-size: 16px;
  margin-left: 10px;
  position: relative;
  top: 3px;
}
.a-uuid {
  font-size: 14px;
  margin-left: 10px;
  position: relative;
  top: -2px;
  color: gray;
}
.add-account {
  height: 35px;
  border: 1px solid skyblue;
  width: calc(50% - 10px);
}
#control {
  width: calc(100% - 100px);
  display: flex;
  justify-content: space-around;
}
</style>
<script lang="ts">
export default {
  name: "Microsoft"
}
</script>