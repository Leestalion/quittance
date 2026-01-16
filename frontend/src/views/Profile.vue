<script setup lang="ts">
import { useAuthStore } from '../stores/auth'

const authStore = useAuthStore()

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    alert('UUID copié dans le presse-papiers !')
  })
}
</script>

<template>
  <div class="profile-page">
    <div class="profile-card">
      <h1>Mon profil</h1>
      
      <div v-if="authStore.user" class="profile-info">
        <div class="info-group uuid-group">
          <label>Identifiant unique (UUID)</label>
          <div class="uuid-container">
            <code class="uuid-code">{{ authStore.user.id }}</code>
            <button @click="copyToClipboard(authStore.user.id)" class="copy-btn" title="Copier">
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
  background: #667eea;
  color: white;
  border: none;
  padding: 0.75rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1.2rem;
  transition: all 0.2s;
  flex-shrink: 0;
}

.copy-btn:hover {
  background: #764ba2;
  transform: scale(1.05);
}

.hint {
  display: block;
  margin-top: 0.5rem;
  color: #666;
  font-size: 0.85rem;
  font-style: italic;
}

@media (prefers-color-scheme: dark) {
  .profile-card {
    background: #1a1a1a;
  }

  .info-group label {
    color: #aaa;
  }
  
  .uuid-group {
    background: #0f0f0f;
    border-color: #333;
  }
  
  .uuid-code {
    background: #2a2a2a;
    color: #e0e0e0;
    border-color: #444;
  }
  
  .hint {
    color: #999;
  }
}
</style>
