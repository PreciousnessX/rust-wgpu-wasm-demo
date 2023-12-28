

<template>
  <div class="app-warp">
   <div id="gpu-warp" class="gpu-warp"></div>
  </div>
</template>

<script setup>
import {onMounted}from 'vue'

import * as wasm from '../bevy-web/pkg/bevy_web'
onMounted(()=>{
    wasm.init_warp('gpu-warp','aaa');
})

</script>

<style scoped lang="scss">
.app-warp{
    width: 100%;
    height: 100%;
    .gpu-warp{
        width: 100%;
        height: 100%;
    }
}
</style>
