<template>
    <div class="h-full w-full flex flex-col transition-colors duration-300 ease-in-out">
        <AppTopbar class="h-12" />
        <AppSidebar class="h-full" />
        <AppEndbar class="h-24 mt-2" />

        <Drawer v-model:open="immersionDrawerOpen" class="w-full h-full m-0">
            <DrawerContent class="w-full h-full m-0">
                <DynamicBackground :background-image="testImage" :brightness="100" :saturation="100" :contrast="100">
                    <ScreenLyrics :image-url="testImage" :lyrics="testLyric" />
                </DynamicBackground>
            </DrawerContent>
        </Drawer>

        <div class="h-0">
            <SettingsDialog v-if="appStore.getSettingsDialogState" />
            <DeveloperOptionsDialog v-if="appStore.getDeveloperOptionsDialogState" />
            <AboutDialog v-if="appStore.getAboutDialogState" />
        </div>

    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import AppEndbar from '@/components/custom/AppEndbar.vue'
import AppSidebar from '@/components/custom/AppSidebar.vue'
import AppTopbar from '@/components/custom/AppTopbar.vue'
import SettingsDialog from '@/components/custom/SettingsDialog.vue'
import DeveloperOptionsDialog from '@/components/custom/DeveloperOptionsDialog.vue'
import AboutDialog from '@/components/custom/AboutDialog.vue'
import { Drawer, DrawerContent } from '@/components/ui/drawer'
import { useAppStore } from '@/stores/appStore'
import DynamicBackground from '@/components/custom/DynamicBackground.vue'
import ScreenLyrics from '@/components/custom/ScreenLyrics.vue'


import testImage from '@/assets/test.png'
import testLyric from '@/assets/test.ttml?raw'

const appStore = useAppStore()

const immersionDrawerOpen = computed({
    get() {
        return appStore.getImmersionDrawerState
    },
    set(value) {
        if (appStore.getGlobalBackdropState) {
            setTimeout(() => {
            }, 500)
            appStore.setImmersionDrawerState(value)
        } else {
            setTimeout(() => {
            }, 500)
            appStore.setImmersionDrawerState(value)
        }
    }
})
</script>

<style scoped></style>