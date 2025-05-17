<script setup lang="ts">
import {ref, watch} from 'vue'
import {current_setting} from '../../logic/changeBody'
import Game from './content/Game.vue'
import Personalization from './content/Personalization.vue'
import Launcher from './content/Launcher.vue'

const isTransitioning = ref(true)

function control_leave() {
  isTransitioning.value = true
}

watch(current_setting, () => isTransitioning.value = false)
</script>
<template>
  <div>
    <transition name="slide" @after-leave="control_leave">
      <Game v-if="current_setting == 'Game' && isTransitioning" class="component" style="background-color: red;"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <Personalization v-if="current_setting == 'Personalization' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <Launcher v-if="current_setting == 'Launcher' && isTransitioning" class="component"
                style="background-color: green;"/>
    </transition>
  </div>
</template>
<style scoped>
.component {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.slide-enter-active {
  animation: rightSlideIn 0.2s;
}

.slide-leave-active {
  animation: rightSlideOut 0.2s;
}

@keyframes rightSlideIn {
  from {
    transform: translateY(-100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes rightSlideOut {
  from {
    transform: translateY(0);
    opacity: 1;
  }
  to {
    transform: translateY(-100%);
    opacity: 0;
  }
}
</style>
<script lang="ts">
export default {
  name: "settingRight"
}
</script>