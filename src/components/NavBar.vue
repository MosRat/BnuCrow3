<!--
  - *Copyright (c) 2024. MosRat
  - All rights reserved.
  -
  - Project: DesktopAnt
  - File Name: NavBar.vue
  - Created Date: 2024/10/4 21:36
  - Author: MosRat (work@whl.moe)
  - Description:
  -->

<script setup lang="ts">
import {useRoute, useRouter} from 'vue-router';
import {ref,computed} from "vue";

import {
  HomeOutlined,
  MenuOutlined,
} from "@ant-design/icons-vue";


interface View {
  title: string,
  path: string,
  icon: any,
}

const router = useRouter();
const route = useRoute();
const views = ref<View[]>([
  // {
  //   title: "登录",
  //   path: "/login",
  //   icon: HomeOutlined
  // },
  {
    title: "课表",
    path: "/classtable",
    icon: HomeOutlined
  },
  {
    title: "功能",
    path: "/actlist",
    icon: MenuOutlined
  }
])
const navigateTo = (path: string) => {
  console.log(`nav to ${path}`)
  router.push({path, replace: true});
};

const current = computed(()=>
  route.path
)


</script>

<template>
  <div class="bar blur">
    <div v-for="(view,i) in views" :key="i" @click="navigateTo(view.path)" :class="['bar-item',{'current-item':current===view.path}]" :style="{
      backgroundColor: current===view.path ? 'rgba(137,137,137,0.2)' : 'rgba(255,255,255,0)'
    }">
      <component :is="view.icon"></component>
      <div class="title">{{ view.title }}</div>
    </div>
  </div>
</template>

<style scoped>
.bar {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-around;
  box-shadow: 0 -2px rgba(198, 197, 197, 0.5);
  height: 7.5vh;

  //background-color: #cacaca;
}

.bar-item{
  display: flex;
  flex-direction: column;
  padding: 0.5em 1.5em 0;
  color: #696969;
  border-radius: 0.25em;
  cursor: pointer;
  transition: background-color 0.5s ease
}

.current-item{
  color: #252525;
}
</style>