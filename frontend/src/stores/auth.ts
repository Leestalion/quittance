import { defineStore } from 'pinia'
import { ref } from 'vue'
import { authAPI } from '../api'
import type { User } from '../types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('auth_token'))
  const isAuthenticated = ref(!!token.value)

  async function login(email: string, password: string) {
    const response = await authAPI.login(email, password)
    token.value = response.token
    user.value = response.user
    localStorage.setItem('auth_token', response.token)
    isAuthenticated.value = true
  }

  async function register(email: string, password: string, name: string, address: string) {
    const response = await authAPI.register(email, password, name, address)
    token.value = response.token
    user.value = response.user
    localStorage.setItem('auth_token', response.token)
    isAuthenticated.value = true
  }

  async function fetchCurrentUser() {
    if (!token.value) return
    try {
      user.value = await authAPI.getCurrentUser()
    } catch (error) {
      console.error('Failed to fetch current user:', error)
      logout()
    }
  }

  function logout() {
    user.value = null
    token.value = null
    isAuthenticated.value = false
    localStorage.removeItem('auth_token')
  }

  return {
    user,
    token,
    isAuthenticated,
    login,
    register,
    fetchCurrentUser,
    logout
  }
})
