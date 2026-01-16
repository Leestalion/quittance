<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

function handleLogout() {
  authStore.logout()
  router.push('/login')
}
</script>

<template>
  <header class="app-header">
    <div class="header-content" :class="{ 'centered': !authStore.isAuthenticated }">
      <router-link to="/dashboard" class="logo">
        <h1>🏠 Quittance</h1>
      </router-link>

      <nav v-if="authStore.isAuthenticated" class="nav">
        <router-link to="/dashboard">Tableau de bord</router-link>
        <router-link to="/properties">Propriétés</router-link>
        <router-link to="/tenants">Locataires</router-link>
        <router-link to="/profile">Profil</router-link>
        <button @click="handleLogout" class="logout-btn">Déconnexion</button>
      </nav>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 2rem 2rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 3rem;
}

.header-content.centered {
  justify-content: center;
}

.logo {
  text-decoration: none;
  color: white;
  transition: transform 0.2s;
}

.logo:hover {
  transform: translateY(-1px);
}

.logo h1 {
  margin: 0;
  font-size: 2rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.nav {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-wrap: wrap;
}

.nav a {
  color: white;
  text-decoration: none;
  padding: 0.625rem 1.25rem;
  border-radius: 8px;
  transition: all 0.2s;
  font-weight: 500;
  font-size: 0.95rem;
  white-space: nowrap;
}

.nav a:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateY(-1px);
}

.nav a.router-link-active {
  background: rgba(255, 255, 255, 0.25);
  font-weight: 600;
}

.logout-btn {
  background: rgba(255, 255, 255, 0.15);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.3);
  padding: 0.625rem 1.25rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.95rem;
  font-weight: 500;
  transition: all 0.2s;
  margin-left: 0.5rem;
  white-space: nowrap;
}

.logout-btn:hover {
  background: rgba(255, 255, 255, 0.25);
  border-color: rgba(255, 255, 255, 0.4);
  transform: translateY(-1px);
}

@media (max-width: 768px) {
  .app-header {
    padding: 1.5rem 1.5rem;
  }

  .header-content {
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .logo h1 {
    font-size: 1.75rem;
  }

  .nav {
    gap: 0.25rem;
    width: 100%;
    justify-content: center;
  }

  .nav a,
  .logout-btn {
    padding: 0.5rem 0.875rem;
    font-size: 0.875rem;
  }
}

@media (max-width: 480px) {
  .app-header {
    padding: 1.25rem 1rem;
  }

  .logo h1 {
    font-size: 1.5rem;
  }

  .nav a,
  .logout-btn {
    padding: 0.5rem 0.75rem;
    font-size: 0.8rem;
  }
}

@media print {
  .app-header {
    display: none !important;
  }
}
</style>
