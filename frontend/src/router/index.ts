import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/dashboard'
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('../views/Login.vue')
    },
    {
      path: '/register',
      name: 'Register',
      component: () => import('../views/Register.vue')
    },
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: () => import('../views/Dashboard.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/properties',
      name: 'Properties',
      component: () => import('../views/PropertyList.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/properties/:id',
      name: 'PropertyDetail',
      component: () => import('../views/PropertyDetail.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/properties/:propertyId/lease/new',
      name: 'GenerateLease',
      component: () => import('../views/GenerateLease.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/properties/:propertyId/lease/:leaseId/print',
      name: 'PrintLease',
      component: () => import('../views/PrintLease.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/properties/:propertyId/receipt/new/:leaseId',
      name: 'GenerateReceipt',
      component: () => import('../views/GenerateReceipt.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/tenants',
      name: 'Tenants',
      component: () => import('../views/TenantList.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/organizations',
      name: 'Organizations',
      component: () => import('../views/OrganizationList.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/organizations/:id',
      name: 'OrganizationDetail',
      component: () => import('../views/OrganizationDetail.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/profile',
      name: 'Profile',
      component: () => import('../views/Profile.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'NotFound',
      component: () => import('../views/NotFound.vue')
    }
  ]
})

// Navigation guard for authentication
router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore()
  
  // Load user data if authenticated but user not loaded
  if (authStore.isAuthenticated && !authStore.user && to.name !== 'Login' && to.name !== 'Register') {
    try {
      await authStore.fetchCurrentUser()
    } catch (error) {
      console.error('Failed to fetch user:', error)
      authStore.logout()
      next('/login')
      return
    }
  }
  
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next('/login')
  } else if ((to.name === 'Login' || to.name === 'Register') && authStore.isAuthenticated) {
    next('/dashboard')
  } else {
    next()
  }
})

export default router
