<script setup lang="ts">
import {computed, onMounted} from 'vue'
import { usePlayerStore } from '@/stores/playerStore'
import {useAppStore} from "@/stores/appStore"
import DefaultCover from '@/assets/default.png'
import {ListX} from 'lucide-vue-next'
import {Button} from "@/components/ui/button"

const playerStore = usePlayerStore()
const appStore = useAppStore()

const tracks = computed(() => playerStore.currentPlaylist.tracks)
const bgi = (background: string[]) => {
  if (background[0] === 'default') {
    return 'url('+DefaultCover+')'
  }
  return 'url('+background.join(',')+')'
}

const storeIndex = computed(() => {
  return  playerStore.currentIndex
})

const formatDuration = (duration: number) => {
  const minutes = Math.floor(duration / 60)
  const seconds = Math.floor(duration % 60)
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`
}

onMounted(() => {

})
</script>

<template>
  <div v-if="appStore.rightArea === 'playlist'" class="h-full w-full flex flex-col mr-1">
    <div class="flex w-full flex-row justify-between items-center pt-2 pb-2">
      <p class="text-lg font-bold">Will be played soon</p>
      <Button variant="ghost" size="icon">
        <ListX />
      </Button>
    </div>
      <ul class="w-full h-full overflow-y-auto mr-1">
        <li class="flex w-full flex-row gap-1.5 items-center border-b border-gray/60 pt-2 pb-2" v-for="track in tracks" :key="track.id">
          <div class="w-[40px] h-[40px] rounded bg-center bg-cover flex flex-row justify-center items-center relative"
          :style="{backgroundImage: bgi(track.cover_art)}">
            <div v-if="true" class="absolute inset-0 rounded bg-gray-800/40"></div>
            <div class="gap-0.5 transform -translate-y-1/4 flex flex-row items-end justify-center w-[26px] h-[25px]">
              <div class="w-[5px] h-[8px] bg-foreground rounded-[2px] animate-bar-1"></div>
              <div class="w-[5px] h-[14px] bg-foreground rounded-[2px] animate-bar-2"></div>
              <div class="w-[5px] h-[6px] bg-foreground rounded-[2px] animate-bar-3"></div>
              <div class="w-[5px] h-[9px] bg-foreground rounded-[2px] animate-bar-4"></div>
            </div>
          </div>
          <div>
            <p class="text-sm font-bold ">{{track.title}}</p>
            <p class="text-xs text-foreground/60">{{track.artist[0]}}</p>
          </div>
          <div class="flex-1"></div>
          <div class="mr-1">
            <p class="text-sm text-foreground-500">{{formatDuration(track.duration??0)}}</p>
          </div>
        </li>
      </ul>
  </div>
</template>

<style scoped>
@keyframes bar1 {
  0%, 100% { height: 8px; }
  25% { height: 12px; }
  50% { height: 4px; }
  75% { height: 10px; }
}

/* 第二个矩形动画 */
@keyframes bar2 {
  0%, 100% { height: 14px; }
  25% { height: 8px; }
  50% { height: 16px; }
  75% { height: 10px; }
}

/* 第三个矩形动画 */
@keyframes bar3 {
  0%, 100% { height: 6px; }
  25% { height: 10px; }
  50% { height: 3px; }
  75% { height: 8px; }
}

/* 第四个矩形动画 */
@keyframes bar4 {
  0%, 100% { height: 9px; }
  25% { height: 5px; }
  50% { height: 12px; }
  75% { height: 7px; }
}

/* 应用动画类 */
.animate-bar-1 {
  animation: bar1 1.2s ease-in-out infinite;
}

.animate-bar-2 {
  animation: bar2 1.4s ease-in-out infinite;
}

.animate-bar-3 {
  animation: bar3 1.1s ease-in-out infinite;
}

.animate-bar-4 {
  animation: bar4 1.3s ease-in-out infinite;
}
</style>