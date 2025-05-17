<script setup lang="ts">
import {current_view, dark_mode} from '../logic/changeBody'
import {ref, watch} from 'vue'
import homeLeft from './home/Left.vue'
import downloadLeft from './download/Left.vue'
import aboutLeft from './about/Left.vue'
import onlineLeft from './online/Left.vue'
import settingLeft from './setting/Left.vue'
import homeRight from './home/Right.vue'
import downloadRight from './download/Right.vue'
import aboutRight from './about/Right.vue'
import settingRight from './setting/Right.vue'

const isTransitioning = ref(true)

const dark = ref(dark_mode.value ? '#151515' : '#f8f8f8')

function control_leave() {
  isTransitioning.value = true
}

watch(current_view, () => isTransitioning.value = false)
watch(dark_mode, () => dark.value = dark_mode.value ? '#151515' : '#f8f8f8')
</script>
<template>
  <div id="body-all">
    <transition name="slide" @after-leave="control_leave()">
      <homeLeft v-if="current_view == 'home' && isTransitioning" class="component" style="width: 33%;"/>
    </transition>
    <transition name="slide-right" @after-leave="control_leave">
      <homeRight v-if="current_view == 'home' && isTransitioning" class="component-right"
                 style="width: 67%; left: 33%"/>
    </transition>
    <transition name="slide" @after-leave="control_leave()">
      <downloadLeft v-if="current_view == 'download' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide-right" @after-leave="control_leave">
      <downloadRight v-if="current_view == 'download' && isTransitioning" class="component-right"/>
    </transition>
    <transition name="slide" @after-leave="control_leave()">
      <onlineLeft v-if="current_view == 'online' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide" @after-leave="control_leave()">
      <settingLeft v-if="current_view =='setting' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide-right" @after-leave="control_leave">
      <settingRight v-if="current_view == 'setting' && isTransitioning" class="component-right"/>
    </transition>
    <transition name="slide" @after-leave="control_leave()">
      <aboutLeft v-if="current_view == 'about' && isTransitioning" class="component"/>
    </transition>
    <transition name="slide-right" @after-leave="control_leave">
      <aboutRight v-if="current_view == 'about' && isTransitioning" class="component-right"/>
    </transition>
  </div>
</template>
<style scoped>
.component {
  position: absolute;
  top: 0;
  left: 0;
  width: 144px;
  height: 100%;
  background-color: v-bind(dark);
  transition: background-color 0.2s;
}

.slide-enter-active {
  animation: leftSlideIn 0.2s;
}

.slide-leave-active {
  animation: leftSlideOut 0.2s;
}

.component-right {
  position: absolute;
  top: 0;
  left: 144px;
  width: calc(100% - 144px);
  height: 100%;
  overflow-y: auto;
}

.slide-right-enter-active {
  animation: rightSlideIn 0.2s;
}

.slide-right-leave-active {
  animation: rightSlideOut 0.2s;
}

@keyframes leftSlideIn {
  from {
    transform: translateX(-100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes leftSlideOut {
  from {
    transform: translateX(0);
    opacity: 1;
  }
  to {
    transform: translateX(-100%);
    opacity: 0;
  }
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