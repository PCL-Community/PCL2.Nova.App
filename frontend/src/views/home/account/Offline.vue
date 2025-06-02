<script setup lang="ts">
import MyTextInput from '../../../components/input/MyTextInput.vue'
import MyNormalLabel from '../../../components/input/MyNormalLabel.vue'
import {ref} from "vue";
import MyRadioButton from "../../../components/button/MyRadioButton.vue";
import MyNormalButton from "../../../components/button/MyNormalButton.vue";
import {
  AccountPart,
  Alex,
  Ari,
  current_account_page,
  Efe,
  Kai,
  Makena,
  Noor,
  Steve,
  Sunny,
  Zuri
} from "../../../logic/changeBody";
import {GenerateBukkitUUID, UUIDToAvatar} from "../../../../wailsjs/go/launcher/MainMethod";
import {messagebox} from "../../../logic/messagebox";
import {SetAccountConfig} from "../../../../wailsjs/go/launcher/Account";
import {launcher} from "../../../../wailsjs/go/models";
const uuid_standard = ref(1)
const username = ref("")
const useruuid = ref("")
const avatar = ref(Steve)
async function usernameInput() {
  if(username.value.length >= 3 && username.value.length <= 16 && uuid_standard.value == 1) {
    useruuid.value = await GenerateBukkitUUID(username.value)
    let mod = await UUIDToAvatar(useruuid.value) % 18
    let arr = [Alex, Ari, Efe, Kai, Makena, Noor, Steve, Sunny, Zuri]
    avatar.value = arr[mod >= 9 ? mod - 9 : mod]
  }
}
async function useruuidInput() {
  const re: RegExp = /^[a-f0-9]{32}$/g;
  if(re.test(useruuid.value)) {
    let mod = await UUIDToAvatar(useruuid.value) % 18
    let arr = [Alex, Ari, Efe, Kai, Makena, Noor, Steve, Sunny, Zuri]
    avatar.value = arr[mod >= 9 ? mod - 9 : mod]
  }
}
async function createAccount() {
  const re1: RegExp = /^[a-zA-Z0-9_]{3,16}/g
  const re2: RegExp = /^[a-f0-9]{32}$/g
  if(!re1.test(username.value)) {
    await messagebox("账户名称错误", "输入的账号名称错误，请输入英文状态下的英文数字和下划线。", 2, ['ok'])
    return
  }
  if(!re2.test(useruuid.value)) {
    await messagebox("账户 UUID 错误", "输入的账号 UUID 错误，请输入 32 位 16 进制的字符串，无需分隔符。", 2, ['ok'])
    return
  }
  AccountPart.accounts.push(<launcher.AccountType>{
    type: "Offline",
    name: username.value,
    uuid: useruuid.value,
    head_skin: avatar.value
  })
  await SetAccountConfig(AccountPart)
  current_account_page.value = true
}
</script>
<template>
  <div id="main-style">
    <div id="center-style">
      <img :src="'data:image/png;base64,' + avatar" id="steve-avatar" alt="头像"/>
      <div class="group">
        <MyNormalLabel>玩家 ID&nbsp;&nbsp;</MyNormalLabel>
        <MyTextInput :place_holder="'请输入用户名'" class="input" title="在 3 - 16 位之间，只能输入英文、数字和下划线。" @input="usernameInput" v-model="username"/>
        <div style="display: flex; justify-content: space-around;">
          <MyRadioButton :is-checked="uuid_standard == 1" @click="uuid_standard = 1">行业规范</MyRadioButton>
          <MyRadioButton :is-checked="uuid_standard == 2" @click="uuid_standard = 2">自定义</MyRadioButton>
        </div>
        <MyNormalLabel v-show="uuid_standard == 2">&nbsp;&nbsp;&nbsp;UUID&nbsp;&nbsp;</MyNormalLabel>
        <MyTextInput v-show="uuid_standard == 2" :place_holder="'请输入UUID'" class="input" title="应为 32 位 16 进制字符串，不含连字符。" @input="useruuidInput" v-model="useruuid"/>
        <div style="display: flex; justify-content: space-around;">
          <MyNormalButton class="create-return" @click="createAccount">创建</MyNormalButton>
          <MyNormalButton class="create-return" @click="current_account_page = true">返回</MyNormalButton>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped>
#main-style {
  display: flex;
  align-items: center;
}

#center-style {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
}
.group {
  width: calc(100% - 60px);
}
#steve-avatar {
  width: 64px;
  image-rendering: pixelated;
  box-shadow: 0 0 10px gray;
  border-radius: 4px;
  margin-bottom: 20px;
}
.input {
  margin-top: 6px;
  width: calc(100% - 80px);
  height: 30px;
  margin-bottom: 6px;
}
.create-return {
  width: 100px;
  height: 30px;
}
</style>