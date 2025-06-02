<script setup lang="ts">
import MyNormalButton from "../../../components/button/MyNormalButton.vue";
import {current_account_page} from "../../../logic/changeBody";
import MyTextInput from "../../../components/input/MyTextInput.vue";
import {OpenCustomURL} from "../../../logic/functions";
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
          fill="none" id="nologin-img">
        <path
            d="M4,20 C4,17 8,17 10,15 C11,14 8,14 8,9 C8,5.667 9.333,4 12,4 C14.667,4 16,5.667 16,9 C16,14 13,14 14,15 C16,17 20,17 20,20"/>
      </svg>
      <br>
      <div style="display: flex; justify-content: space-around; width: calc(100% - 50px); margin-left: 25px">
        <MyNormalButton class="create-return" @click="current_account_page = true">添加新账号</MyNormalButton>
        <MyNormalButton class="create-return" style="width: 80px" @click="OpenCustomURL('https://www.xbox.com/zh-cn/games/store/minecraft-java-bedrock-edition-for-pc/9nxp44l49shj')">购买正版</MyNormalButton>
        <MyNormalButton class="create-return" style="width: 50px" @click="current_account_page = true">返回</MyNormalButton>
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
  width: 100%;
}

#nologin-img {
  margin-top: 20px;
  margin-left: calc(50% - 40px);
  stroke: rgb(0, 186, 254);
  width: 80px;
  height: 80px;
}

.create-return {
  width: 100px;
  height: 30px;
  border: 1px solid skyblue;
}
</style>
<script lang="ts">
export default {
  name: "Microsoft"
}
</script>