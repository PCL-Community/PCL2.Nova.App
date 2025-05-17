<script setup lang="ts">
import {current_about, current_view} from "../../../logic/changeBody";
import {onMounted, reactive, ref} from "vue";
import MyNormalButton from "../../../components/button/MyNormalButton.vue";
import MyNormalLabel from "../../../components/input/MyNormalLabel.vue";

const is_start = ref(false)
const score = ref(0)
const step = ref(0)

const td0 = reactive([
  [2, 32, 16, 2048],
  [256, 4, 64, 32],
  [64, 512, 8, 128],
  [256, 128, 1024, 16],
])
let lock_up = false
let lock_down = false
let lock_left = false
let lock_right = false
const is_success = ref("")

function genRandomChess(bc: boolean): boolean {
  let count = 0;
  for (let c = 0; c < 4; c++) {
    for (let d = 0; d < 4; d++) {
      if (td0[c][d] == 0) count++
    }
  }
  if(count == 0) return false
  let rand = Math.floor(Math.random() * count)
  count = 0;
  for (let c = 0; c < 4; c++) {
    for (let d = 0; d < 4; d++) {
      if (td0[c][d] == 0) {
        if (count == rand) {
          if(bc) {
            let rand2 = Math.floor(Math.random() * 3)
            switch (rand2) {
              case 0:
                td0[c][d] = 256
                break
              case 1:
                td0[c][d] = 512
                break
              case 2:
                td0[c][d] = 1024
                break
            }
          }else{
            td0[c][d] = 2
          }
          return true
        }
        count++
      }
    }
  }
  return false
}
function compArray(input: number[]): number[] {
  let arr = input.concat()
  let n = arr.length
  for(let i = 0; i < n;) {
    if(arr[i] == 0) { i++; continue }
    let merged = false
    if(i + 1 < n && arr[i + 1] == arr[i]) {
      //首先找一遍是否相邻
      score.value += arr[i]
      arr[i] *= 2
      if(arr[i] == 2048) {
        is_success.value = "你赢了！"
        is_start.value = false
      }
      arr[i + 1] = 0
      i += 2
      merged = true
    } else {
      //如果不相邻
      let j = i + 1
      while(j < n && arr[j] == 0) j++
      if(j < n && arr[j] == arr[i]) {
        let canMerge = true
        for(let k = i + 1; k < j; k++){
          if(arr[k] != 0) {
            canMerge = false
            break
          }
        }
        if(canMerge) {
          score.value += arr[i]
          arr[i] *= 2
          if(arr[i] == 2048) {
            is_success.value = "你赢了！"
            is_start.value = false
          }
          arr[j] = 0
          i = j + 1
          merged = true
        }
      }
    }
    if(!merged) i++
  }
  //合并数字
  let w = 0
  for(let i = 0; i < n; i++) {
    if(arr[i] != 0) arr[w++] = arr[i]
  }
  while(w < n) arr[w++] = 0
  return arr
}
const up = () => {
  if(is_start.value && !lock_up) {
    for (let c = 0; c < 4; c++) {
      let arr = []
      for(let d = 0; d < 4; d++) {
        arr.push(td0[d][c])
      }
      let p = compArray(arr)
      for(let d = 0; d < 4; d++) {
        td0[d][c] = p[d]
      }
    }
    if(genRandomChess(false)) {
      step.value++
      if(is_success.value != "你赢了！") {
        is_success.value = ""
      }
    }else{
      is_success.value = "无法往上"
    }
  }
}
const down = () => {
  if(is_start.value && !lock_down) {
    for (let c = 0; c < 4; c++) {
      let arr = []
      for(let d = 3; d >= 0; d--) {
        arr.push(td0[d][c])
      }
      let p = compArray(arr)
      for(let d = 0, e = 3; d < 4; d++, e--) {
        td0[e][c] = p[d]
      }
    }
    if(genRandomChess(false)) {
      step.value++
      if(is_success.value != "你赢了！") {
        is_success.value = ""
      }
    }else{
      is_success.value = "无法往下"
    }
  }
}
const left = () => {
  if(is_start.value && !lock_left) {
    for(let c = 0; c < 4; c++) {
      let arr = []
      for(let d = 0; d < 4; d++) {
        arr.push(td0[c][d])
      }
      let p = compArray(arr)
      for(let d = 0; d < 4; d++) {
        td0[c][d] = p[d]
      }
    }
    if(genRandomChess(false)) {
      step.value++
      if(is_success.value != "你赢了！") {
        is_success.value = ""
      }
    }else{
      is_success.value = "无法往左"
    }
  }
}
const right = () => {
  if(is_start.value && !lock_right) {
    for(let c = 0; c < 4; c++) {
      let arr = []
      for(let d = 3; d >= 0; d--) {
        arr.push(td0[c][d])
      }
      let p = compArray(arr)
      for(let d = 0, e = 3; d < 4; d++, e--) {
        td0[c][e] = p[d]
      }
    }
    if(genRandomChess(false)) {
      step.value++
      if(is_success.value != "你赢了！") {
        is_success.value = ""
      }
    }else{
      is_success.value = "无法往右"
    }
  }
}
function randomGen1() {
  if(is_start.value) {
    if(genRandomChess(true)) {
      is_success.value = "作弊成功"
    }else{
      is_success.value = "无法生成"
    }
  }
}

function startGame() {
  score.value = 0
  step.value = 0
  Object.assign(td0, [
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
  ])
  is_start.value = true;
  is_success.value = ""
  genRandomChess(false)
  genRandomChess(false)
}

onMounted(() => {
  document.addEventListener("keyup", (e: KeyboardEvent) => {
    if (current_view.value === "about" && current_about.value === "P2048") {
      if ((e.code as string) === "ArrowUp" || (e.code as string) === "KeyW") {
        up()
      } else if ((e.code as string) === "ArrowLeft" || (e.code as string) === "KeyA") {
        left()
      } else if ((e.code as string) === "ArrowRight" || (e.code as string) === "KeyD") {
        right()
      } else if ((e.code as string) === "ArrowDown" || (e.code as string) === "KeyS") {
        down()
      }
    }
  })
})
</script>

<template>
  <div id="P2048">
    <div id="score">
      <MyNormalButton id="start-button" @click="startGame">
        点我开始
      </MyNormalButton>
      <MyNormalLabel id="score-label">
        分数：{{ score }}
      </MyNormalLabel>
      <MyNormalLabel id="score-label">
        步数：{{ step }}
      </MyNormalLabel>
      <MyNormalButton id="start-button" @click="randomGen1">
        作弊生成
      </MyNormalButton>
      <MyNormalLabel id="score-label">
        {{ is_success }}
      </MyNormalLabel>
    </div>
    <div id="chess">
      <div id="chess-content">
        <table>
          <tbody>
          <tr>
            <td></td>
            <td class="chess-direction chess-direction-tb" @click="up">
              <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="1.5">
                <path d="M12 20V4m0 0l6 6m-6-6l-6 6"/>
              </svg>
            </td>
            <td></td>
          </tr>
          <tr>
            <td class="chess-direction chess-direction-lr" @click="left">
              <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="1.5">
                <path d="M20 12H4m0 0l6-6m-6 6l6 6"/>
              </svg>
            </td>
            <td class="chess-content-center font-pcl">
              <table>
                <tbody>
                <tr v-for="td1 in td0">
                  <td v-for="td2 in td1"
                      :style="'background-color: ' + (
                              td2 == 0 ? '#cdc1b3' :
                              td2 == 2 ? '#efe5db' :
                              td2 == 4 ? '#eddcbe' :
                              td2 == 8 ? '#f3b079' :
                              td2 == 16 ? '#f7925c' :
                              td2 == 32 ? '#f57656' :
                              td2 == 64 ? '#f4522c' :
                              td2 == 128 ? '#edce71' :
                              td2 == 256 ? '#e6d151' :
                              td2 == 512 ? '#1200ff' :
                              td2 == 1024 ? '#cc00ff' :
                              td2 == 2048 ? '#000000' : 'white'
                          ) + '; font-size: ' + (
                              td2 == 2 || td2 == 4 || td2 == 8 ? '50px' :
                              td2 == 16 || td2 == 32 || td2 == 64 ? '40px' :
                              td2 == 128 || td2 == 256 || td2 == 512 ? '30px' :
                              td2 == 1024 || td2 == 2048 ? '25px' : '1000px'
                          ) + '; color: ' + (
                              td2 == 2 || td2 == 4 || td2 == 512 || td2 == 128 ? '#7a6d65' :
                              td2 == 8 || td2 == 16 || td2 == 32 || td2 == 64 || td2 == 256 || td2 == 1024 || td2 == 2048 ? '#ffffff' : '#000000'

                          )">
                    {{ td2 == 0 ? "" : td2 }}
                  </td>
                </tr>
                </tbody>
              </table>
            </td>
            <td class="chess-direction chess-direction-lr" @click="right">
              <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="32"
                  height="32"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="1.5">
                <path d="M4 12h16m0 0l-6-6m6 6l-6 6"/>
              </svg>
            </td>
          </tr>
          <tr>
            <td></td>
            <td class="chess-direction chess-direction-tb" @click="down">
              <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="1.5">
                <path d="M12 4v16m0 0l6-6m-6 6l-6-6"/>
              </svg>
            </td>
            <td></td>
          </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<style scoped>
#P2048 {
  display: flex;
  overflow: hidden;
}

#score-label {
  font-size: 30px;
  font-weight: bold;
  margin: 10px;
}

#score {
  flex-shrink: 0;
  width: 200px;
  height: 100%;
  border-right: 2px solid gray;
  display: flex;
  flex-direction: column;
  align-items: center;
}

#start-button {
  width: calc(100% - 20px);
  height: 100px;
  font-size: 30px;
  font-weight: bold;
  margin: 10px;
}

#chess {
  height: 100%;
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

#chess-content {
  width: 500px;
  height: 500px;
}

#chess-content table {
  width: 100%;
  height: 100%;
}

#chess-content table tr td {
  text-align: center;
}

.chess-direction {
  stroke: gray;
  cursor: pointer;
  transition: stroke 0.2s;
}

.chess-direction:hover {
  stroke: dimgray;
}

.chess-direction-lr {
  width: 74px;
  height: 350px;
}

.chess-direction-tb {
  width: 350px;
  height: 74px;
}

.chess-direction svg {
  width: 32px;
  height: 32px;
}

.chess-content-center {
  width: 350px;
  height: 350px;
  border: 2px solid black;
  background-color: rgb(192, 175, 157);
  border-radius: 10px;
  position: relative;
  padding: 3px;
}

.chess-content-center table tr td {
  border-radius: 10px;
  font-weight: bold;
  width: 65px;
  height: 65px;
}
</style>