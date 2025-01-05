<template>
  <div class="player-page">
    <div data-vjs-player>
      <video
        ref="videoElement"
        class="video-js vjs-big-play-centered"
        controls
      ></video>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useRoute } from 'vue-router'
import videojs from 'video.js'
import 'video.js/dist/video-js.css'

const route = useRoute()
const videoElement = ref<HTMLVideoElement | null>(null)
const player = ref<videojs.Player | null>(null)

onMounted(() => {
  if (!videoElement.value) return

  player.value = videojs(videoElement.value, {
    autoplay: true,
    controls: true,
    responsive: true,
    fluid: true,
    sources: [{
      src: route.query.url as string,
      type: 'application/x-mpegURL'
    }]
  }, function onPlayerReady() {
    console.log('播放器已就绪')
  })
})

onBeforeUnmount(() => {
  if (player.value) {
    player.value.dispose()
  }
})
</script>

<style scoped>
.player-page {
  width: 100%;
  height: 100vh;
  background: #000;
  display: flex;
  justify-content: center;
  align-items: center;
}

:deep(.video-js) {
  width: 100%;
  height: 100%;
  max-width: 1200px;
  max-height: 80vh;
}
</style> 