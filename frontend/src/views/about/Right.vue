<script setup lang="ts">
import {ref, watch} from 'vue'
import {current_about} from '../../logic/changeBody'
import About from './content/About.vue'
import Feedback from './content/Feedback.vue'
import TreasureBox from './content/TreasureBox.vue'
import Minesweeper from './games/Minesweeper.vue'
import P2048 from './games/P2048.vue'

const isTransitioning = ref(true)

function control_leave() {
  isTransitioning.value = true
}

watch(current_about, () => isTransitioning.value = false)
</script>
<template>
  <div>
    <transition name="slide" @after-leave="control_leave">
      <TreasureBox v-if="current_about == 'Treasure-Box' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <About v-if="current_about == 'About' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <Feedback v-if="current_about == 'Feedback' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <Minesweeper v-if="current_about == 'Mine-Sweeper' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <P2048 v-if="current_about == 'P2048' && isTransitioning" class="component"/>
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
  name: "aboutRight"
}
</script>