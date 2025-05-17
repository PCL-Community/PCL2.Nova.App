<script setup lang="ts">
import {ref, watch} from 'vue'
import {dark_mode} from '../../logic/changeBody'

interface LoadingProps {
  loading_text: string,
  state: number,
}

const dark = ref(dark_mode.value ? '#3142b7' : 'aliceblue')
watch(dark_mode, value => dark.value = value ? 'black' : 'aliceblue')
const loading_props = withDefaults(defineProps<LoadingProps>(), {loading_text: '正在加载', state: 0})
const loading_text_ref = ref('正在加载')
const current_state = ref(0)
const color = ref("blue")
watch(() => loading_props.loading_text, value => {
  loading_text_ref.value = value
}, {immediate: true})
watch(() => loading_props.state, value => {
  current_state.value = value
  color.value = value == 1 ? "red" : value == 0 ? "blue" : "green"
}, {immediate: true})
</script>
<template>
  <div class="ala">
    <svg
        role="img"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 240 240"
        width="150"
        height="150"
        stroke-width="5"
        stroke-linecap="round"
        stroke-linejoin="round"
        fill="none" class="loading-svg">
      <path
          d="M 80 120 Q 95 100 130 100 Q 140 80 150 100 Q 180 100 200 120 Q 150 100 145 110 L 145 190 L 135 190 L 135 110 M 80 120 Q 120 100 135 110"
          :class="current_state != 0 ? 'loading-stop' : 'loading'"/>
      <path d="M 30 190 L 90 190"/>
      <path d="M 50 180 L 40 170" :class="current_state != 0 ? 'loading-stop-bling' : 'loading-bling-1'"/>
      <path d="M 55 180 L 50 170" :class="current_state != 0 ? 'loading-stop-bling' : 'loading-bling-2'"/>
      <path d="M 65 180 L 70 170" :class="current_state != 0 ? 'loading-stop-bling' : 'loading-bling-3'"/>
      <path d="M 70 180 L 80 170" :class="current_state != 0 ? 'loading-stop-bling' : 'loading-bling-4'"/>
    </svg>
    <br>
    {{ loading_text_ref }}
  </div>
</template>
<style scoped>
.ala {
  width: 200px;
  height: 200px;
  background-color: v-bind(dark);
  text-align: center;
  font-weight: bold;
  border: 1px solid blue;
  border-radius: 20px;
  box-shadow: 0 0 6px gray;
  stroke: v-bind(color);
  color: v-bind(color);
  transition: all 0.2s;
}

.loading {
  animation: loading 1s infinite;
  transform-origin: 140px 180px;
}

.loading-stop {
  transform: rotate(-10deg);
  transform-origin: 140px 180px;
}

.loading-stop-bling {
  opacity: 0;
}

.loading-bling-1 {
  animation: bling-1 1s infinite;
}

.loading-bling-2 {
  animation: bling-2 1s infinite;
}

.loading-bling-3 {
  animation: bling-3 1s infinite;
}

.loading-bling-4 {
  animation: bling-4 1s infinite;
}

@keyframes loading {
  0%, 100% {
    transform: rotate(20deg);
  }
  20% {
    transform: rotate(-50deg);
  }
  70%, 80% {
    transform: rotate(-10deg);
  }
}

@keyframes bling-1 {
  0%, 19.9%, 100% {
    opacity: 0;
  }
  20%, 40% {
    opacity: 1;
  }
  20% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(-10px, -10px);
  }
}

@keyframes bling-2 {
  0%, 19.9%, 100% {
    opacity: 0;
  }
  20%, 40% {
    opacity: 1;
  }
  20% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(-6px, -10px);
  }
}

@keyframes bling-3 {
  0%, 19.9%, 100% {
    opacity: 0;
  }
  20%, 40% {
    opacity: 1;
  }
  20% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(6px, -10px);
  }
}

@keyframes bling-4 {
  0%, 19.9%, 100% {
    opacity: 0;
  }
  20%, 40% {
    opacity: 1;
  }
  20% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(10px, -10px);
  }
}
</style>