<!--
  - *Copyright (c) 2024. MosRat
  - All rights reserved.
  -
  - Project: BnuCrow3
  - File Name: Score.vue
  - Created Date: 2024/10/19 12:52
  - Author: MosRat (work@whl.moe)
  - Description:
  -->

<script setup lang="ts">
import {computed, onMounted, ref, watch} from "vue";
import DCP, {Score} from "@lib/dcp.ts";

const scoreList = ref<Score[]>([])
const selectorActivated = ref(false)
const xn = ref(<string>DCP.info?.xn)
const xq = ref(<string>DCP.info?.xq)
const xqm = computed(() => xq.value?.toString() === "0" ? "秋季" : "春季")
const noScore = computed(() => scoreList.value.length == 0)
const calTotalScore = computed(() => {
  let totalWeightedScore = 0;
  let totalCredits = 0;
  scoreList.value.forEach(score => {
    if (score.total_score.length > 0) {
      const scoreValue = parseFloat(score.total_score);
      if (!scoreValue) {
        return
      }
      totalWeightedScore += scoreValue * score.credits;
      totalCredits += score.credits;
    } else {
      const scoreValue = parseFloat(score.total_score) || 0; // 将字符串转为数字，空字符串则为0
      totalWeightedScore += scoreValue * score.credits;
      totalCredits += score.credits;
    }
  });
  return (totalCredits > 0 ? (totalWeightedScore / totalCredits) : 0).toFixed(2);
})
const calMajorScore = computed(() => {
  let totalWeightedScore = 0;
  let totalCredits = 0;
  scoreList.value.forEach(score => {
    if (score.category === "专业教育课程类/学科基础课程模块") {
      if (score.total_score.length > 0) {
        const scoreValue = parseFloat(score.total_score);
        if (!scoreValue) {
          return
        }
        totalWeightedScore += scoreValue * score.credits;
        totalCredits += score.credits;
      } else {
        const scoreValue = parseFloat(score.total_score) || 0; // 将字符串转为数字，空字符串则为0
        totalWeightedScore += scoreValue * score.credits;
        totalCredits += score.credits;
      }

    }
  });
  return (totalCredits > 0 ? (totalWeightedScore / totalCredits) : 0).toFixed(2);
})

onMounted(async () => {
  // examList.value = await DCP.get_exam(<string>DCP.info?.xn,<string>DCP.info?.xq)
  console.log(<string>DCP.info?.xq)
  scoreList.value = await DCP.get_score("2024", "1")
  console.log(JSON.stringify(scoreList.value))
})

watch([xn, xq], async ([newVal1, newVal2]) => {
  scoreList.value = await DCP.get_score(newVal1.toString(), newVal2.toString())
  console.log(JSON.stringify(scoreList.value))
})
</script>

<template>
  <div class="container">
    <div class="prompt">{{ xn }}学年 {{ xqm }}学期 学分绩 {{ calTotalScore }} 专业课 {{ calMajorScore }}</div>
    <div v-if="noScore" class="no-score"> 暂无成绩</div>
    <div v-for="(score,i) in scoreList" :key="i" class="score-item">
      <div class="score-total">{{ score.total_score ?? "0" }}</div>
      <div class="score-info">
        <div class="score-name">
          <div class="score-course" :style="{
            color:score?.course_type==='必修'?'#f692ee':'#9495fd'
          }">{{ score.course }}
          </div>
          <div class="score-remark" v-if="score?.remarks?.length>0">{{ score.remarks }}</div>
        </div>
        <span class="score-category">{{ score.category }}</span>
        <div class="score-scores">
          <span class="score-credits">学分：{{ score.credits }}</span>
          <span class="score-usual">平时分：{{ score?.usual_score?.toFixed(2) }}</span>
          <span class="score-final">期末分：{{ score?.final_score?.toFixed(2) }}</span>
        </div>
      </div>
    </div>
    <div class="select blur" @click="()=>selectorActivated=true">📆</div>
    <div class="selector-bg" v-if="selectorActivated" @click.stop>
      <div class="selector blur">
        <div>学年</div>
        <select v-model=xn>
          <option v-for="x in 14" :key="x" :value="2014+x"> {{ 2014 + x }}</option>
        </select>

        <div>学期</div>
        <select v-model="xq">
          <option v-for="x in 2" :key="x" :value="x-1"> {{ x === 2 ? "春季" : "秋季" }}</option>
        </select>
        <button @click="selectorActivated=false">确定</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container > * + * {
  border-bottom: rgba(173, 173, 173, 0.5) 2px solid;
}

.container {
  padding-bottom: 20vh;
}

.prompt {
  background-color: #d6d6d6;
  font-size: 0.8rem;
  padding: 0.5em 1em;
  color: #888888;
}

.no-score {
  text-align: center;
}

.score-item {
  padding: 1.5vh 2vw;
  display: flex;
}

.score-total {
  padding: 1em 0.5em;
  text-align: center;
  width: 15vw;
  color: #ff9494;
}

.score-info {
  font-size: 0.7rem;
  line-height: 1;
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.score-name {
  display: flex;
}


.score-course {
  font-size: 1rem;
  line-height: 1;
  font-weight: bolder;
}

.score-remark {
  color: #ff5959;
  padding-inline: 0.5em;
  font-size: 0.9rem;
  border-radius: 1em;
  margin-left: 0.5em;
  background-color: rgba(253, 164, 164, 0.1);
}

.score-category {
  border-radius: 1em;
  background-color: rgba(136, 136, 136, 0.1);
  align-self: flex-start;
  padding: 0.4em;
}

.score-scores, .score-category {
  color: #888888;
}

.score-scores span {
  margin-right: 1em;
}


.select {
  position: fixed;
  bottom: 20vh;
  right: 15vw;
  border-radius: 50%;
  background-color: rgba(136, 136, 136, 0.3);
  //padding: 2.5vw;
  text-align: center;
  display: flex; /* 使用 Flexbox 布局 */
  justify-content: center; /* 水平居中 */
  align-items: center; /* 垂直居中 */
  width: 15vw;
  height: 15vw;
  font-size: 1rem;
  z-index: 99999;
}

.selector-bg {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.selector {
  padding: 1em 2em;
  border-radius: 0.5em;
  display: flex;
  flex-direction: column;
  width: 60vw;
}

.selector div {
  margin: 0.5em 0;
}

.selector button {
  margin: 1.5em 0;
  border: none;
  box-shadow: none;
}


.selector button, .selector selector {
  display: block;
  background-color: rgba(255, 255, 255, 0.3);
  height: 4vh;
  border-radius: 8px;
}
</style>