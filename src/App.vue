<script setup lang="ts">
import NavBar from "@cp/NavBar.vue";
import Refresher from "@cp/Refresher.vue";
import {useRoute} from 'vue-router'
import Error from "@view/Error.vue";
import {ref, watch} from 'vue'


const route = useRoute()
const transitionName = ref('fade')

watch(
    () => route.path,
    (to, from) => {
      if (to === '/classtable' && from === '/actlist') {
        transitionName.value = 'slide-left'
      } else if (to === '/actlist' && from === '/classtable') {
        transitionName.value = 'slide-right'
      } else {
        transitionName.value = 'fade'
      }
    }
)


</script>

<template>
  <div class="bg" :style="{
    backgroundImage: route.meta.bg as string
  }">
    <Error/>
    <div class="main-container">
      <transition :name="transitionName">
        <router-view/>
      </transition>
    </div>
    <NavBar v-if="!route.meta.hideComponents as boolean"/>
    <Refresher/>
  </div>
</template>

<style scoped>

.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active,
.fade-enter-active,
.fade-leave-active {
  transition: all 0.2s ease-out;
}

.slide-left-enter-from {
  transform: translateX(100%);
}

.slide-left-leave-to {
  transform: translateX(-100%);
}

.slide-right-enter-from {
  transform: translateX(-100%);
}

.slide-right-leave-to {
  transform: translateX(100%);
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.router-view-wrapper {
  position: absolute;
  width: 100%;
  transition: all 0.2s ease-out;
}


.bg {
  width: 100vw;
  height: 100vh;
}

.main-container {
  width: 100vw;
  height: 92.5vh;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  //align-items: center;
  overflow: auto;
  //border: 1px red solid;
  padding: 0;
  //background: linear-gradient(120deg, #e0c3fc 0%, #8ec5fc 100%);
}

.navi {

}


</style>
