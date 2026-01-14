<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

function goHome() {
  router.push('/dashboard')
}

function goBack() {
  router.back()
}
</script>

<template>
  <div class="not-found">
    <div class="error-container">
      <h1 class="error-code">404</h1>
      <h2>Page non trouvée</h2>
      <p>Désolé, la page que vous recherchez n'existe pas.</p>
      
      <div class="actions">
        <button @click="goBack" class="secondary-btn">
          ← Retour
        </button>
        <button @click="goHome" class="primary-btn">
          🏠 Accueil
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.not-found {
  min-height: 80vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.error-container {
  text-align: center;
  max-width: 600px;
}

.error-code {
  font-size: 8rem;
  margin: 0 0 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  font-weight: 900;
  line-height: 1;
}

.error-container h2 {
  font-size: 2.5rem;
  margin: 0 0 1rem;
  color: #1a1a1a;
  font-weight: 700;
  letter-spacing: -0.02em;
}

.error-container p {
  font-size: 1.2rem;
  color: #666;
  margin-bottom: 2.5rem;
  line-height: 1.6;
  font-weight: 400;
}

.actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

.primary-btn,
.secondary-btn {
  padding: 0.75rem 2rem;
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  border: none;
  transition: transform 0.2s;
}

.primary-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.secondary-btn {
  background: #f5f5f5;
  color: #333;
  border: 2px solid #e0e0e0;
}

.primary-btn:hover,
.secondary-btn:hover {
  transform: translateY(-2px);
}

@media (prefers-color-scheme: dark) {
  .error-container h2 {
    color: #f0f0f0;
  }

  .error-container p {
    color: #b0b0b0;
  }

  .secondary-btn {
    background: #333;
    color: #ddd;
    border-color: #555;
  }
}

@media (max-width: 600px) {
  .error-code {
    font-size: 5rem;
    margin-bottom: 1rem;
  }

  .error-container h2 {
    font-size: 1.75rem;
  }

  .error-container p {
    font-size: 1rem;
  }

  .actions {
    flex-direction: column;
  }

  .primary-btn,
  .secondary-btn {
    width: 100%;
  }
}
</style>
