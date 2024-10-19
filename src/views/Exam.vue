<!--
  - *Copyright (c) 2024. MosRat
  - All rights reserved.
  -
  - Project: BnuCrow3
  - File Name: Exam.vue
  - Created Date: 2024/10/19 12:52
  - Author: MosRat (work@whl.moe)
  - Description:
  -->

<script setup lang="ts">
import {computed, onMounted, ref, watch} from "vue";
import DCP, {Exam} from "@lib/dcp.ts";

const examList = ref<Exam[]>([])
const selectorActivated = ref(false)
const xn = ref(<string>DCP.info?.xn)
const xq = ref(<string>DCP.info?.xq)
const xqm = computed(() => xq.value.toString() === "0" ? "秋季" : "春季")
const noExam = computed(() => examList.value.length == 0)
onMounted(async () => {
  // examList.value = await DCP.get_exam(<string>DCP.info?.xn,<string>DCP.info?.xq)
  console.log(<string>DCP.info?.xq)
  examList.value = await DCP.get_exam("2024", "1")
  console.log(JSON.stringify(examList.value))
})

watch([xn, xq], async ([newVal1, newVal2]) => {
  examList.value = await DCP.get_exam(newVal1.toString(), newVal2.toString())
})
</script>


<template>

  <div class="container">
    <div class="prompt">{{ xn }}学年 {{ xqm }}学期 考试安排</div>
    <div v-if="noExam" class="no-score"> 暂无考试安排</div>
    <div v-for="(exam,i) in examList" :key="i" class="exam-item">
      <div class="exam-name">{{ exam.course_code }}</div>
      <div class="exam-info">
        <div>{{ exam.exam_time }}</div>
        <div>
          <span>{{ exam.location }}</span>
          座号：
          <span>{{ exam.seat_number }}</span>
          <span>{{ exam.assessment }}</span>
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

.exam-item {
  padding: 1.5vh 4vw;
}

.exam-name {
  font-size: 1rem;
  line-height: 1;
}

.exam-info {
  font-size: 0.8rem;
  line-height: 1;
  padding-top: 0.25vh;
}

.exam-info > * {
  padding-top: 0.75vh;
  color: #888888;
}

.exam-info span {
  margin-right: 2vw;
}

.select {
  position: fixed;
  bottom: 20vh;
  right: 15vw;
  border-radius: 50%;
  background-color: rgba(136, 136, 136, 0.3);
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