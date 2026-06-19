<script setup lang="ts">
import { useAuthStore } from '../stores/auth'
import BaseCard from '../components/ui/BaseCard.vue'

const authStore = useAuthStore()

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    alert('UUID copié dans le presse-papiers !')
  })
}
</script>

<template>
  <div class="profile-page">
    <BaseCard class="profile-card">
      <h1>Mon profil</h1>
      
      <div v-if="authStore.user" class="profile-info">
        <div class="info-group uuid-group">
          <label>Identifiant unique (UUID)</label>
          <div class="uuid-container">
            <code class="uuid-code">{{ authStore.user.id }}</code>
            <button @click="copyToClipboard(authStore.user.id)" class="copy-btn c-button c-button--secondary" title="Copier">
              📋
            </button>
          </div>
          <small class="hint">Cet identifiant est nécessaire pour vous ajouter à une organisation</small>
        </div>

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
    </BaseCard>
  </div>
</template>

<style scoped>
.profile-page {
  max-width: 700px;
  margin: 0 auto;
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

.uuid-group {
  background: #f5f5f5;
  padding: 1rem;
  border-radius: 8px;
  border: 2px solid #e0e0e0;
}

.uuid-container {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.uuid-code {
  flex: 1;
  background: #fff;
  padding: 0.75rem;
  border-radius: 6px;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  color: #333;
  border: 1px solid #ddd;
  word-break: break-all;
}

.copy-btn {
  padding: 0.75rem 1rem;
  font-size: 1.2rem;
  flex-shrink: 0;
}

.copy-btn:hover {
  transform: scale(1.05);
}

.hint {
  display: block;
  margin-top: 0.5rem;
  color: #666;
  font-size: 0.85rem;
  font-style: italic;
}

</style>
