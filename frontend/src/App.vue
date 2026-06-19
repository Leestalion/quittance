<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView } from 'vue-router'
import { useAuthStore } from './stores/auth'
import { useTheme } from './composables/useTheme'
import AppHeader from './components/AppHeader.vue'

const authStore = useAuthStore()
const { initTheme } = useTheme()

onMounted(() => {
  initTheme()
  if (authStore.isAuthenticated) {
    authStore.fetchCurrentUser()
  }
})
</script>

<template>
  <div class="app-shell">
    <AppHeader />
    <main class="app-main l-page">
      <RouterView />
    </main>
  </div>
</template>
