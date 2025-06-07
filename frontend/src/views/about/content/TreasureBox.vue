<script setup lang="ts">
import MySelectCard from '../../../components/card/MySelectCard.vue'
</script>
<template>
  <div style="display: flex; flex-direction: column; overflow-y: auto;">
    <MySelectCard :maxHeight="1100" :isExpand="true" :title="'百宝箱'" :isLast="true">
      <div v-for="() in 100" style="padding: 10px; text-align: center">
        测试：可以合上的超长卡片~
      </div>
    </MySelectCard>
  </div>
</template>
<style scoped>
</style>