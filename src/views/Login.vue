<!--
  - *Copyright (c) 2024. MosRat
  - All rights reserved.
  -
  - Project: BnuCrow3
  - File Name: Login.vue
  - Created Date: 2024/10/11 12:46
  - Author: MosRat (work@whl.moe)
  - Description:
  -->

<script setup lang="ts">
import {useRouter} from 'vue-router';
import {onMounted, ref} from "vue";
import {store} from "@lib/tauri-store";
import DCP from "@lib/dcp.ts";


const router = useRouter();
const username = ref("")
const password = ref("")
const logining = ref(false)
const errorMsg = ref("")
// const loginButton = computed(() =>
//   logining.value ? "登录中..." : "登录"
// )
const navigateTo = (path: string) => {
  console.log(`nav to ${path}`)
  router.push({path, replace: true});
};

const dcp_login = async () => {
  errorMsg.value = ""
  try {
    logining.value = true
    errorMsg.value = await DCP.login(
        username.value,
        password.value
    )
    logining.value = false
    await store.set('userinfo', {username: username.value, password: password.value});
    navigateTo('/classtable')
  } catch (e: any) {
    console.log(e)
    const m = e.match(/"([^"]*)"/)
    errorMsg.value = m ? m[1] : ""
    logining.value = false
  }

}

onMounted(async () => {
  const userinfo = await store.get<{ username: string, password: string }>('userinfo')
  username.value = userinfo?.username ? userinfo.username : ""
  password.value = userinfo?.password ? userinfo.password : ""
  if (userinfo) {
    await dcp_login()
  }
})
</script>

<template>
  <div class="container">
    <div class="login-container blur">
      <h2>Bnu Crow 3</h2>
      <input type="text" class="input" placeholder="学号" v-model="username">
      <input type="password" class="input" placeholder="数字京师密码" v-model="password">

      <button @click="dcp_login" v-if="!logining">登录</button>
      <button v-else>登录中...</button>
    </div>
    <div class="error-msg" v-if="errorMsg.length>0">{{ errorMsg }}</div>
    <div class="copyright">Copyright (c) 2024. MosRat All Rights Reserved</div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
}

.main-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.login-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  border: 1px #cacaca solid;
  border-radius: 0.75em;
  padding: 0.75em 1em;
}

h2 {
  font-style: italic;
  text-transform: uppercase;
}

.input {
  display: block;
  margin-bottom: 1.5em;
  border-radius: 999px;
  border: none;
  padding: 0.75em;
  background-color: rgba(214, 214, 214, 0.5);
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  outline: none;
  width: 80vw;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.2em 1.75em;
  font-size: 1rem;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: rgba(214, 214, 214, 0.1);
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  outline: none;
}

.copyright {
  position: fixed;
  bottom: 1em;
  font-size: 0.75rem;
  color: #888888;
  background-color: transparent;
}

.error-msg{
  margin-top: 10vh;
  background-color: rgba(255, 77, 77, 0.75);
  padding: 1em 0.75em;
  border-radius: 0.5em;
  color: #cacaca;
  font-size: 0.8rem;
  line-height: 1;
}
</style>