<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import BaseButton from '../components/ui/BaseButton.vue'
import BaseCard from '../components/ui/BaseCard.vue'
import BaseField from '../components/ui/BaseField.vue'
import BaseState from '../components/ui/BaseState.vue'

const router = useRouter()
const authStore = useAuthStore()

const email = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)

async function handleLogin() {
  error.value = ''
  loading.value = true

  try {
    await authStore.login(email.value, password.value)
    router.push('/dashboard')
  } catch (err: any) {
    error.value = err.response?.data?.error || 'Échec de la connexion'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-page auth-page">
    <BaseCard class="login-card auth-card">
      <h1>🏠 Connexion</h1>
      <p class="subtitle">Accédez à votre gestion locative</p>

      <form @submit.prevent="handleLogin">
        <BaseField label="Email" for-id="email">
          <input
            id="email"
            v-model="email"
            type="email"
            required
            placeholder="votre@email.com"
          />
        </BaseField>

        <BaseField label="Mot de passe" for-id="password">
          <input
            id="password"
            v-model="password"
            type="password"
            required
            placeholder="••••••••"
          />
        </BaseField>

        <BaseState v-if="error" variant="error">{{ error }}</BaseState>

        <BaseButton type="submit" class="auth-submit" :loading="loading" :disabled="loading">
          {{ loading ? 'Connexion...' : 'Se connecter' }}
        </BaseButton>
      </form>

      <p class="register-link">
        Pas encore de compte ?
        <router-link to="/register">S'inscrire</router-link>
      </p>
    </BaseCard>
  </div>
</template>
