<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const email = ref('')
const password = ref('')
const name = ref('')
const address = ref('')
const error = ref('')
const loading = ref(false)

async function handleRegister() {
  error.value = ''
  loading.value = true

  try {
    await authStore.register(email.value, password.value, name.value, address.value)
    router.push('/dashboard')
  } catch (err: any) {
    error.value = err.response?.data?.error || 'Échec de l\'inscription'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="register-page">
    <div class="register-card">
      <h1>🏠 Inscription</h1>
      <p class="subtitle">Créez votre compte propriétaire</p>

      <form @submit.prevent="handleRegister">
        <div class="form-group">
          <label for="name">Nom complet</label>
          <input
            id="name"
            v-model="name"
            type="text"
            required
            placeholder="Jean Dupont"
          />
        </div>

        <div class="form-group">
          <label for="email">Email</label>
          <input
            id="email"
            v-model="email"
            type="email"
            required
            placeholder="votre@email.com"
          />
        </div>

        <div class="form-group">
          <label for="password">Mot de passe</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            minlength="8"
            placeholder="••••••••"
          />
        </div>

        <div class="form-group">
          <label for="address">Adresse</label>
          <textarea
            id="address"
            v-model="address"
            required
            rows="3"
            placeholder="123 Rue Example, 75001 Paris"
          />
        </div>

        <div v-if="error" class="error">{{ error }}</div>

        <button type="submit" class="submit-btn" :disabled="loading">
          {{ loading ? 'Inscription...' : 'S\'inscrire' }}
        </button>
      </form>

      <p class="login-link">
        Déjà un compte ?
        <router-link to="/login">Se connecter</router-link>
      </p>
    </div>
  </div>
</template>

<style scoped>
.register-page {
  min-height: 80vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.register-card {
  background: white;
  padding: 3rem;
  border-radius: 16px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  width: 100%;
}

.register-card h1 {
  margin: 0 0 0.5rem;
  font-size: 2rem;
  text-align: center;
}

.subtitle {
  color: #666;
  text-align: center;
  margin: 0 0 2rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #333;
}

input,
textarea {
  width: 100%;
  padding: 0.75rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 1rem;
  font-family: inherit;
  transition: border-color 0.2s;
}

input:focus,
textarea:focus {
  outline: none;
  border-color: #667eea;
}

.error {
  background: #fee;
  color: #c33;
  padding: 0.75rem;
  border-radius: 8px;
  margin-bottom: 1rem;
  text-align: center;
}

.submit-btn {
  width: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 1rem;
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s;
}

.submit-btn:hover:not(:disabled) {
  transform: translateY(-2px);
}

.submit-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.login-link {
  text-align: center;
  margin-top: 1.5rem;
  color: #666;
}

.login-link a {
  color: #667eea;
  text-decoration: none;
  font-weight: 600;
}

@media (prefers-color-scheme: dark) {
  .register-card {
    background: #1a1a1a;
  }

  label {
    color: #ddd;
  }

  input,
  textarea {
    background: #2a2a2a;
    border-color: #444;
    color: white;
  }
}
</style>
