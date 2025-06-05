<script setup lang="ts">
import {onMounted, onUnmounted, ref, watch} from 'vue'
import {dark_mode} from '../../logic/changeBody'

interface CheckButtonProps {
  isExpand: boolean,
  // maxHeight: number,
  title: string,
  isLast?: boolean,
}

const light = ref(dark_mode.value ? '#f8f8f8' : '#151515')
const dark = ref(dark_mode.value ? '#151515' : '#f8f8f8')
watch(dark_mode, value => {
  light.value = value ? '#f8f8f8' : '#151515'
  dark.value = value ? '#151515' : '#f8f8f8'
})
const isCheckedProps = withDefaults(defineProps<CheckButtonProps>(), {
  isExpand: false,
  // maxHeight: 0,
  title: "",
  isLast: false
})
// const mh = ref(isCheckedProps.maxHeight + 'px')
const cardHeight = ref(60)  // 默认标题区域高度
// watch(() => isCheckedProps.maxHeight, value => mh.value = value + 'px')
const isExpandComp = ref(!isCheckedProps.isExpand)
// const border = ref("6px")
const isLastComp = ref(isCheckedProps.isLast ? "15px" : "0")
const touchColor = ref(dark_mode.value ? '#0a0a0a' : '#d6d6d6')

let observer: ResizeObserver | null = null
const mycardInnerRef = ref<HTMLElement>()
function style_cancel() {
  // border.value = isExpandComp.value ? "0" : "6px"
  let hovColor = dark_mode.value ? "#0a0a0a" : "#d6d6d6"
  let Color = dark_mode.value ? "#151515" : "#f8f8f8"
  touchColor.value = isExpandComp.value ? Color : hovColor
}

function changeProps() {
  if (!isCheckedProps.isExpand) return;
  isExpandComp.value = !isExpandComp.value
  // border.value = "0"
  touchColor.value = dark_mode.value ? "#151515" : "#f8f8f8"
  if(!isExpandComp) cardHeight.value = 40
}

onMounted(() => {
    observer = new ResizeObserver(() => {
        // 为了方便使高度动画具有回弹动效，使用observer侦测内容来赋值而非让其自动撑开
        cardHeight.value = mycardInnerRef.value!.offsetHeight + 26
    })
    observer.observe(mycardInnerRef.value!)
})

onUnmounted(() => observer?.disconnect())
</script>

<template>
  <div class="card-container">
    <div :class="'grid' + (isCheckedProps.isExpand ? ' cursor-pointer' : '')"
         :isOpen="(isExpandComp ? 'expand' : 'close')" @click="changeProps">
      {{ isCheckedProps.title }}
      <svg
          role="img"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          stroke-width="3"
          stroke-linecap="round"
          stroke-linejoin="round"
          fill="none" :class="isExpandComp ? 'card-icon-expand' : 'card-icon'"
          :style="isCheckedProps.isExpand ? 'display: flow' : 'display: none'">
        <polyline points="6 10 12 16 18 10"/>
      </svg>
    </div>
    <Transition name="card-content">
      <div @transitionend="style_cancel()" v-show="isExpandComp" ref="mycardInnerRef">
        <slot/>
      </div>
    </Transition>
  </div>
</template>
<style scoped>
.card-content-enter-active,
.card-content-leave-active {
    transition: opacity 0.2s;
}

.card-content-enter-from,
.card-content-leave-to {
    opacity: 0;
}

.card-container{
  flex-shrink: 0;
  height: v-bind("cardHeight +'px'");
  background-color: v-bind(dark);
  margin: 22px;
  padding: 10px 20px;
  border-radius: 6px;
  transition: height 0.4s cubic-bezier(.4, 1.4, .6, 1);
  overflow: hidden;
}

.grid[isOpen="close"],
.grid[isOpen="expand"] {
  /* background-color: v-bind(dark); */
  font-size: 16px;
  font-weight: bold;
  transition: all 0.2s;
  color: v-bind(light);
}

.grid[isOpen="close"]:hover {
  background-color: v-bind(touchColor);
}

.grid + div {
  /* max-height: 0; */
  /* overflow: hidden; */
  /* border-bottom-left-radius: 6px; */
  /* border-bottom-right-radius: 6px; */
  /* margin-left: 22px;
  margin-right: 22px; */
  /* background-color: v-bind(dark); */
  transition: all 0.2s;
  margin-bottom: v-bind(isLastComp);
  color: v-bind(light);
}

.card-icon,
.card-icon-expand {
  width: 20px;
  height: 20px;
  vertical-align: middle;
  transition: all 0.2s;
  float: right;
  stroke: v-bind(light);
}

.card-icon-expand {
  transform: rotate(180deg);
}

.grid[isOpen="expand"] + div {
  height: fit-content;
  /* max-height: 100%; */
  /* max-height: v-bind(mh); */
}
</style>