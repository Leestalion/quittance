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
  <div class="register-page auth-page">
    <BaseCard class="register-card auth-card">
      <h1>🏠 Inscription</h1>
      <p class="subtitle">Créez votre compte bailleur</p>

      <form @submit.prevent="handleRegister">
        <BaseField label="Nom complet" for-id="name">
          <input
            id="name"
            v-model="name"
            type="text"
            required
            placeholder="Jean Dupont"
          />
        </BaseField>

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
            minlength="8"
            placeholder="••••••••"
          />
        </BaseField>

        <BaseField label="Adresse" for-id="address">
          <textarea
            id="address"
            v-model="address"
            required
            rows="3"
            placeholder="123 Rue Example, 75001 Paris"
          />
        </BaseField>

        <BaseState v-if="error" variant="error">{{ error }}</BaseState>

        <BaseButton type="submit" class="auth-submit" :loading="loading" :disabled="loading">
          {{ loading ? 'Inscription...' : 'S\'inscrire' }}
        </BaseButton>
      </form>

      <p class="login-link">
        Déjà un compte ?
        <router-link to="/login">Se connecter</router-link>
      </p>
    </BaseCard>
  </div>
</template>
