<script setup lang="ts">
import { useAuthStore } from '../stores/auth'

const authStore = useAuthStore()
</script>

<template>
  <div class="profile-page">
    <div class="profile-card">
      <h1>Mon profil</h1>
      
      <div v-if="authStore.user" class="profile-info">
        <div class="info-group">
          <label>Nom</label>
          <p>{{ authStore.user.name }}</p>
        </div>

        <div class="info-group">
          <label>Email</label>
          <p>{{ authStore.user.email }}</p>
        </div>

        <div class="info-group">
          <label>Adresse</label>
          <p>{{ authStore.user.address }}</p>
        </div>

        <div v-if="authStore.user.phone" class="info-group">
          <label>Téléphone</label>
          <p>{{ authStore.user.phone }}</p>
        </div>

        <div v-if="authStore.user.birth_date" class="info-group">
          <label>Date de naissance</label>
          <p>{{ new Date(authStore.user.birth_date).toLocaleDateString() }}</p>
        </div>

        <div v-if="authStore.user.birth_place" class="info-group">
          <label>Lieu de naissance</label>
          <p>{{ authStore.user.birth_place }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.profile-page {
  max-width: 600px;
  margin: 0 auto;
}

.profile-card {
  background: white;
  padding: 2rem;
  border-radius: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.profile-card h1 {
  margin: 0 0 2rem;
}

.profile-info {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.info-group label {
  display: block;
  font-weight: 600;
  color: #666;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.info-group p {
  margin: 0;
  font-size: 1.1rem;
}

@media (prefers-color-scheme: dark) {
  .profile-card {
    background: #1a1a1a;
  }

  .info-group label {
    color: #aaa;
  }
}
</style>
