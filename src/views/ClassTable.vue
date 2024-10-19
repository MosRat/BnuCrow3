<!--
  - *Copyright (c) 2024. MosRat
  - All rights reserved.
  -
  - Project: BnuCrow3
  - File Name: Grid.vue
  - Created Date: 2024/10/1 20:38
  - Author: MosRat (work@whl.moe)
  - Description:
  -->

<script setup lang="ts">
import {Course, Info, update} from "@lib/dcp.ts"
import DCP from "@lib/dcp.ts";
import {computed, onMounted} from "vue";

interface CourseWithColor {
  name: string;
  teacher: string;
  location: string;
  time: string;
  weekday: string;
  backgroundColor: string;
}

//
const colors = [
  "#FFEE99",
  "#CCCCFF",
  "#FFB7DD",
  "#FFCCCC",
  "#AAFFEE",
  "#99FFFF",
  "#CCEEFF",
  "#FFC8B4",
  "#FFDDAA",
  "#FFFFBB",
  "#EEFFBB",
  "#CCBBFF",
  "#CCFF99",
  "#99FF99",
  "#BBFFEE",
  "#D1BBFF",
  "#E8CCFF",
  "#F0BBFF",
  "#CCDDFF",
  "#FFB3FF"
];

function assignColorsToCourses(courses: Course[][]): CourseWithColor[][] {
  const nameToColorMap = new Map<string, string>();
  let colorIndex = 0;

  return courses.map(courseArray =>
      courseArray.map(course => {
        if (!nameToColorMap.has(course.name)) {
          nameToColorMap.set(course.name, colors[colorIndex]);
          colorIndex = (colorIndex + 1) % colors.length;
        }

        return {
          ...course,
          backgroundColor: nameToColorMap.get(course.name) as string
        };
      })
  );
}

function truncateString(str: string, maxLength: number) {
  if (str.length <= maxLength) {
    return str;
  }
  return str.slice(0, maxLength) + '...';
}


const weekdays = [' ', '周一', '周二', '周三', '周四', '周五', '周六', '周日']
const info = computed<Info>(() => update.value && DCP?.info || {})
// const current_week = ref(6)
const classes = computed<CourseWithColor[][]>(() => assignColorsToCourses(update.value && DCP?.class_table || [
      [],
      [
        {
          name: '数学',
          teacher: '张老师',
          location: '教室101',
          time: '1-4',
          weekday: "3",
        },
        {
          name: '语文',
          teacher: '他老师',
          location: '教室121',
          time: '9-10',
          weekday: "1",
        }
      ]
    ])
)

onMounted(async () => {
  // classes.value = assignColorsToCourses(await DCP.get_class_table())
  // info.value = await DCP.get_info()
})
</script>

<template>
  <div>
    <div class="button blur">
      {{ info.week }} / 20 周
    </div>

    <div class="body">
      <div class="table">
        <div v-for="i in 8" :key="i" class="col-header header" :style="{
        gridColumn:`${i} / span 1`
      }">
          {{ weekdays[i - 1] }}
        </div>

        <div v-for="i in 12" :key="i" class="row-header header" :style="{
        gridRow:`${i + 1} / span 1`
      }">
          {{ i }}
        </div>

        <div v-for="(classS,i) in classes[info.week as any]" :key="i" class="class-item" :style="{
        gridColumn: `${Number(classS.weekday) + 1}`,
        gridRow: `${Number(classS.time.split('-')[0]) + 1} / ${Number(classS.time.split('-')[1]) + 2}`,
        backgroundColor: classS.backgroundColor
      }">
          <div class="class-name">{{ classS.name }}</div>
          <div class="class-location">{{ classS.location }}</div>
          <div class="class-teacher">{{ truncateString(classS.teacher, 3) }}</div>
        </div>
      </div>
    </div>
  </div>

</template>

<style scoped>

.body {
  display: inline-block;
  overflow: auto;
  border-radius: 2px;
  padding-top: 1vh;
}

.table {
  margin: 0;
  font-size: 0.8rem;
  line-height: 1;
  display: grid;
  grid-template-rows: 2vh repeat(12, 6.5vh);
  grid-template-columns: 5vw repeat(7, 15vw);
  grid-gap: 0.5em;
  overflow: auto;
  width: auto;
}


.table > * {
  text-align: center;
  text-decoration: none;
}

.header {
  //background-color: rgba(136, 136, 136, 0.5);
}

.table:first-child {
  padding: 0;
}

.col-header {
  //padding : .5% .75em;
  border-radius: 0;
}

.row-header {
  padding: 2.5vh 0.05em;
  //place-self: center;
  //border-bottom: 1px rgba(202, 202, 202, 0.5) solid;
  //outline: 1px solid black;
  border-radius: 0;
  position: sticky;
  left: 0;
  z-index: 9999;
}


.class-item {
  font-size: 0.65rem;
  border: 1px #cacaca solid;
  border-radius: 0.75em;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  filter: drop-shadow(0 0 2px rgba(72, 71, 71, 0.5));
  padding: 1vh 2vw;
}

.class-name {
  font-size: 0.85em;
  font-weight: bold;
  font-style: italic;
}

.class-location {
  margin-top: auto;
  font-size: 0.65em;
}

.class-teacher {
  font-size: 0.65em;
}

.button {
  border-radius: 9999px;
  position: fixed;
  bottom: 30%;
  right: 35px;
  width: 7em;
  height: 7em;
  text-align: center;
  padding-block: 3em;
  font-size: 0.5rem;
  line-height: 1;
  z-index: 999;
}


</style>