<script setup lang="ts">

</script>

<template>
  <div class="bottom-nav">
    <div class="nav-item"><a href="#">首页</a></div>
    <div class="nav-item"><a href="#">搜索</a></div>
    <div class="nav-item"><a href="#">通知</a></div>
    <div class="nav-item"><a href="#">设置</a></div>
  </div>
</template>

<style scoped>
.bottom-nav {
  display: flex;
  justify-content: space-around;
  align-items: center;
  background-color: #333;
  color: white;
  padding: 10px 0;
  position: fixed;
  bottom: 0;
  width: 100%;
  border: 1px solid beige;
}

.nav-item {
  text-align: center;
  flex: 1;
}

.nav-item a {
  color: white;
  text-decoration: none;
  font-size: 16px;
}
</style>