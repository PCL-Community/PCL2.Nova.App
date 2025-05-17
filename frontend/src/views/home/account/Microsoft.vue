<script setup lang="ts">
import MyNormalButton from "../../../components/button/MyNormalButton.vue";
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
      <MyNormalButton id="add-account">添加新账号</MyNormalButton>
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

#nologin-img {
  margin-top: 20px;
  margin-left: calc(50% - 40px);
  stroke: rgb(0, 186, 254);
  width: 80px;
  height: 80px;
}

#add-account {
  margin-top: 20px;
  margin-left: 50px;
  width: calc(100% - 100px);
  height: 35px;
  border: 1px solid skyblue;
}
</style>
<script lang="ts">
export default {
  name: "Microsoft"
}
</script>