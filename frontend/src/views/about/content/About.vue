<script setup lang="ts">
import MySelectCard from '../../../components/card/MySelectCard.vue'
</script>
<template>
  <div style="display: flex; flex-direction: column;">
    <MySelectCard :maxHeight="40" :isExpand="false" :title="'关于'" :isLast="true">
      <div style="padding: 10px">
        <center>测试：无法合上的卡片</center>
      </div>
    </MySelectCard>
  </div>
</template>
<style scoped>
</style>