<script setup lang="ts">
import MyTextInput from '../../../components/input/MyTextInput.vue'
import MyNormalLabel from '../../../components/input/MyNormalLabel.vue'
</script>
<template>
  <div id="main-style">
    <div id="center-style">
      <img src="../../../assets/images/steve-avatar.png" id="steve-avatar" alt="头像"/>
      <br>
      <MyNormalLabel class="username-hint">用户名</MyNormalLabel>
      <br>
      <MyTextInput :place_holder="'请输入用户名'" class="username"/>
    </div>
  </div>
</template>
<style scoped>
#main-style {
  display: flex;
  align-items: center
}

#center-style {
  height: 190px;
  width: 100%;
}

#steve-avatar {
  margin-top: 20px;
  width: 64px;
  image-rendering: pixelated;
  margin-left: calc(50% - 32px);
  box-shadow: 0 0 10px gray;
  border-radius: 4px;
  margin-bottom: 30px;
}

.username-hint {
  font-size: 12px;
  margin-left: 40px;
}

.username {
  margin-top: 6px;
  margin-left: 40px;
  width: calc(100% - 100px);
  height: 35px;
}
</style>