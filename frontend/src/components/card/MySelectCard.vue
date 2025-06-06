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
const cardHeight = ref("43px")
// watch(() => isCheckedProps.maxHeight, value => mh.value = value + 'px')
const isExpandComp = ref(!isCheckedProps.isExpand)
// const border = ref("6px")
const isLastComp = ref(isCheckedProps.isLast ? "15px" : "0")
const touchColor = ref(dark_mode.value ? '#0a0a0a' : '#d6d6d6')

const myCardInnerRef = ref<HTMLElement>()
// let observer: ResizeObserver | null = null

function style_cancel() {
  // border.value = isExpandComp.value ? "0" : "6px"
  let hovColor = dark_mode.value ? "#0a0a0a" : "#d6d6d6"
  let Color = dark_mode.value ? "#151515" : "#f8f8f8"
  touchColor.value = isExpandComp.value ? Color : hovColor
}

function changeProps() {
  if (!isCheckedProps.isExpand) return
  isExpandComp.value = !isExpandComp.value
  touchColor.value = dark_mode.value ? "#151515" : "#f8f8f8"
  cardHeight.value = isExpandComp.value ? (myCardInnerRef.value!.offsetHeight + 43) + "px" : "43px"
}
onMounted(() => {
  if (!isCheckedProps.isExpand) {
    cardHeight.value = (myCardInnerRef.value!.offsetHeight + 43) + "px"
  }
})
// onMounted(() => {
//   observer = new ResizeObserver(() => {
//     if(!isExpandComp.value) return
//     cardHeight.value = myCardInnerRef.value!.offsetHeight + 26
//   })
//   observer.observe(myCardInnerRef.value!)
// })
//
// onUnmounted(() => observer?.disconnect())

</script>
<template>
  <div class="card-container">
    <div :class="['grid', isCheckedProps.isExpand ? ' cursor-pointer' : '']"
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
    <div @transitionend="style_cancel()" ref="myCardInnerRef">
      <slot/>
    </div>
  </div>
</template>
<style scoped>
.card-container {
  flex-shrink: 0;
  height: v-bind(cardHeight);
  border-radius: 6px;
  transition: height 0.2s;
  margin: 15px 22px v-bind(isLastComp) 22px;
  background-color: v-bind(dark);
  overflow: hidden;
}
.grid[isOpen="close"],
.grid[isOpen="expand"] {
  /* border-radius: 6px 6px v-bind(border) v-bind(border); */
  border-radius: 6px;
  padding: 10px 20px;
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
  /* margin-left: 22px; */
  /* margin-right: 22px; */
  /* background-color: v-bind(dark); */
  transition: all 0.2s;
  color: v-bind(light);
  overflow: hidden;
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
  /* max-height: v-bind(mh); */
}
</style>