<script setup lang="ts">


const rf = async () => {
  window.location.reload()
  // console.log(await DCP.get_info())
  // try {
  //   DCP.fetch_info()
  // } catch (e) {
  //   console.log(e, e.toString())
  // }
}
</script>

<template>
  <div class="button blur" @click="rf">
    ...
  </div>
</template>

<style scoped>
.button {
  border-radius: 9999px;
  position: fixed;
  bottom: 20%;
  right: 35px;
  width: 3.5em;
  height: 3.5em;
  text-align: center;
  padding-block: 0.75em;
}
</style>