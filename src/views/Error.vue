<!--
  - *Copyright (c) 2024. MosRat
  - All rights reserved.
  -
  - Project: BnuCrow3
  - File Name: Error.vue
  - Created Date: 2024/10/18 21:54
  - Author: MosRat (work@whl.moe)
  - Description:
  -->

<script setup lang="ts">
import {getCurrentWindow} from "@tauri-apps/api/window";
import {computed, ref} from "vue";
import {DcpError} from "@lib/dcp.ts";


const curWindow = getCurrentWindow();
const errorList = ref<DcpError[]>([])
const showModal = computed(() =>
    errorList.value.length > 0
)


curWindow.listen<DcpError>("error", async ({payload}) => {
  console.error(payload)
  errorList.value.push(payload)
})
</script>

<template>
  <div v-if="showModal" class="modal-overlay">
    <div class="modal-content" @click.stop>
      <h3>错误</h3>
      <p>后端错误 [{{ errorList[0].file }} {{ errorList[0].line }}] {{ errorList[0].error }}</p>
      <button @click="errorList.shift">关闭</button>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
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

.modal-content {
  width: 80vw;
  background-color: white;
  padding: 2vh 2vw;
  border-radius: 2px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}
</style>