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
import {useRouter} from 'vue-router';
import {ref} from "vue";

interface View {
  title:string,
  path:string,
  icon:string,
}

const router = useRouter();
const views = ref<View[]>([
  {
    title:"课表",
    path:"/home",
    icon:""
  },
  {
    title:"功能",
    path:"/home",
    icon:""
  }
])
const navigateTo = (path: string) => {
  console.log(`nav to ${path}`)
  router.push({path, replace: true});
};


</script>

<template>
  <div class="bar">
    <div v-for="(view,i) in views" :key="i" @click="navigateTo(view.path)">
      <div class="icon">{{view.icon}}</div>
      <div class="title">{{view.title}}</div>
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
  box-shadow: 0 -1px rgba(198, 197, 197, 0.5);
}
</style>