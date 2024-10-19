<!--
  - *Copyright (c) 2024. MosRat
  - All rights reserved.
  -
  - Project: BnuCrow3
  - File Name: ActionList.vue
  - Created Date: 2024/10/11 12:45
  - Author: MosRat (work@whl.moe)
  - Description:
  -->

<script setup lang="ts">

import DCP from "@lib/dcp.ts";
import {Info, update} from "@lib/dcp.ts";
import {computed, onMounted} from "vue";
import {useRouter} from "vue-router";
import {open} from "@tauri-apps/plugin-shell";

const router = useRouter();

// const info = ref<Info>({})
// const avatarUrl = ref("/avatar.png")
// const soap = ref<{ soap: string, balance: number }>({soap: "0.0", balance: 0.0})

const info = computed<Info>(() => update.value && DCP?.info || {})
const avatarUrl = computed<string>(() => update.value && DCP?.avatar || "/avatar.png")
const soap = computed<{ soap: string, balance: number }>(() => update.value && DCP?.soap || {soap: "0.0", balance: 0.0})

const navigateTo = (path: string) => {
  console.log(`nav to ${path}`)
  router.push({path, replace: true});
};

// const webviewTo = (path: string) => {
//   window.open(path, "_blank")
// }

const exitLogin = async () => {
  await DCP.reset()
  navigateTo("/login")
}

onMounted(async () => {
  await DCP.syncData()
  // info.value = await DCP.get_info()
  // console.log(JSON.stringify(info.value))
  // avatarUrl.value = await DCP.get_avatar()
  // let s = await DCP.get_soap_info()
  //
  // soap.value = {soap: s.soap.toFixed(2), balance: s.balance}
})

</script>

<template>
  <div class="main-container">
    <div class="userinfo">
      <div class="avatar-container gradient-border">
        <img class="avatar" :src="avatarUrl" alt="avatar">
      </div>
      <div class="username">{{ info.xm }}
        <span class="gender-male" v-if="info.xb==='男'">♂</span>
        <span class="gender-female" v-else>♀</span>
      </div>
      <div class="week-container">
        <div class="year">{{ info.xn }}-{{ Number(info.xn) + 1 }} 学年</div>
        <div class="term">{{ info.xqm }}</div>
        <div class="week">第 {{ info.week }} / 20 周</div>
      </div>
      <div class="id">{{ info.yhxh }}</div>
      <div class="school">{{ info.yxb }}</div>
      <div class="class-name">{{ info.bjmc }}</div>
      <div class="balance">
        <div class="card">校园卡信息暂不支持</div>
        <div class="divider"></div>
        <div class="soap">
          <div>已用流量 {{ Number(soap.soap).toFixed(2) }}GB</div>
          <div>余额 ￥{{ soap.balance }}</div>
        </div>
      </div>
    </div>
    <div class="acts">
      <div class="block-title">我的羁绊</div>
      <div class="block-body">
        <div @click="router.push({path:'/exam'});">考试安排</div>
        <div @click="router.push({path:'/score'});">成绩·学分·GPA</div>
      </div>
      <div class="block-title">每日任务</div>
      <div class="block-body">
        <div @click="async ()=>{
          DCP.webviewTo( (await DCP.get_inner())?'http://zyfw.prsc.bnu.edu.cn/public/dykb.kxjsi.html':
        'https://onevpn.bnu.edu.cn/http/77726476706e69737468656265737421eaee478b69207a437d468ba28d1b26314f9ba11c/public/dykb.kxjsi.html',
        undefined,
        true
        )
        }">空闲教室
        </div>
        <div @click="async ()=>{
          DCP.webviewTo(
            'https://jsty.bnu.edu.cn/content/vab/#/65628701',
        undefined,
        true
        )
        }">锻炼打卡
        </div>
        <div @click="open('http://532movie.bnu.edu.cn')">532 Movie （打开浏览器）</div>
        <div>精彩活动</div>
      </div>
      <div class="block-title">图书馆</div>
      <div class="block-body">
        <div
            @click="async ()=>{
              DCP.webviewTo('http://seat.lib.bnu.edu.cn/login?targetUri=%2F',
              undefined,
        true)
            }">
          预约座位
        </div>
        <div @click="DCP.webviewTo('http://www.lib.bnu.edu.cn/zy/index.htm')">馆藏查询</div>
        <div @click="DCP.webviewTo('http://www.lib.bnu.edu.cn/zy/index.htm')">借阅查询</div>
      </div>
      <div class="block-title">在你身边</div>
      <div class="block-body">
        <div @click="DCP.webviewTo('https://www.bnu.edu.cn/map/images/map.jpg')">校园平面图</div>
        <div @click="DCP.webviewTo('https://www.bnu.edu.cn/xysh/syxx/xl/')">校历</div>
        <div>常用电话</div>
      </div>
      <div class="exit" @click="exitLogin">退出登录</div>
    </div>
  </div>
</template>

<style scoped>
.userinfo {
  background-image: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  border-radius: 0 0 0.5em 0.5em;
  display: grid;
  grid: 1fr 0.5fr 0.5fr 0.5fr / 45px 1.25fr 1fr;
  padding: 1em 1.5em;
  column-gap: 2em;
  row-gap: 0;
  position: fixed;
  top: 0;
  height: 20vh;
  width: 100%;
  z-index: 999;
}

.avatar-container {
  grid-row: span 4;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 4em;
  padding: 0; /* 内边距，形成边框的距离 */
  border-radius: 999px; /* 圆形边框 */
}

.avatar {
  max-width: 100%; /* 限制图片最大宽度 */
  max-height: 100%; /* 限制图片最大高度 */
  object-fit: contain; /* 保持图片比例 */
  border-radius: 999px;
}

.gradient-border {
  --borderWidth: 2px;
  background: transparent;
  position: relative;
  border-radius: 999px;
}

.gradient-border:after {
  content: '';
  position: absolute;
  //top: calc(-1 * var(--borderWidth));
  //left: calc(-1 * var(--borderWidth));
  height: calc(100% + var(--borderWidth) * 2);
  width: calc(100% + var(--borderWidth) * 2);
  background: linear-gradient(60deg, #f79533, #f37055, #ef4e7b, #a166ab, #5073b8, #1098ad, #07b39b, #6fba82);
  border-radius: 999px;
  z-index: -1;
  animation: animatedgradient 3s ease alternate infinite;
  background-size: 300% 300%;
}


@keyframes animatedgradient {
  0% {
    background-position: 0 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0 50%;
  }
}

.username {
  font-weight: bolder;
  padding-top: 1vh;
  line-height: 1;
}

.gender-male {
  color: #8ec5fc;
  font-weight: 1000;
}

.gender-female {
  color: pink;
  font-weight: 1000;
}


.id, .school, .class-name {
  padding-top: 0.25vh;
  font-size: 0.6rem;
  padding-right: 0;
  color: #888888;
  line-height: 1;
}

.id {
  padding-top: 0.75vh;
}

.week-container {
  grid-row: span 4;
  font-size: 0.65rem;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  padding: 0.5vh 0;
  line-height: 1;
}

.balance {
  font-size: 0.75rem;
  grid-column: span 3;
  display: flex;
  width: 100%;
  justify-content: flex-start;
  align-items: center;
}

.card {
  flex: 1;
}

.soap {
  flex: 1;
  //border-left: 1px rgba(168, 168, 168, 0.5) solid;
  padding: 1em .75em;
  display: flex;
  flex-direction: column;
}

.divider {
  width: 1px;
  height: 70%;
  //flex-grow: 1;
  background: linear-gradient(to top, transparent, rgba(130, 130, 130, 0.5), transparent);
  margin: 0 10px;
}

.acts {
  background-image: linear-gradient(to top, #d5d4d0 0%, #d5d4d0 1%, #eeeeec 31%, #efeeec 75%, #e9e9e7 100%);
  padding-block: 20vh 10vh;
  height: 100vh;
  overflow: auto;
}

.acts *, .acts {
  z-index: 0;
}

.block-title {
  color: rgba(87, 87, 87, 0.73);
  font-size: 0.75rem;
  padding: 0.5em 1em;
}

.block-body {
  margin-inline: 0.5em;
  border-radius: 1em;
  overflow: hidden;
}

.block-body > * {
  padding: 0.5em 0.75em;
  background: linear-gradient(to right, rgba(255, 255, 255, 0.5), rgba(228, 228, 228, 0.5));
  backdrop-filter: blur(4px) brightness(0.8);
  position: relative;
}

.block-body > *::after {
  content: ">";
  position: absolute;
  right: 1em;
  top: 50%;
  transform: translateY(-50%);
  color: rgba(136, 136, 136, 0.4); /* 箭头颜色 */
  font-size: 1.25rem;
}

.block-body * + * {
  border-top: 1px rgba(118, 117, 117, 0.2) dashed;
}

.exit {
  padding: 0.5em 0.75em;
  background: linear-gradient(to right, rgba(255, 255, 255, 0.5), rgba(228, 228, 228, 0.5));
  backdrop-filter: blur(4px) brightness(0.8);
  color: red;
  margin-top: 2em;
  margin-inline: 0.5em;
  border-radius: 1em;
  text-align: center;
}

</style>