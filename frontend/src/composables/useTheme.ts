import { ref, onMounted } from 'vue'

type Theme = 'light' | 'dark'

const currentTheme = ref<Theme>('light')

export function useTheme() {
  function getSystemTheme(): Theme {
    if (typeof window === 'undefined') return 'light'
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }

  function getStoredTheme(): Theme | null {
    if (typeof localStorage === 'undefined') return null
    const stored = localStorage.getItem('theme')
    return stored === 'dark' ? 'dark' : stored === 'light' ? 'light' : null
  }

  function applyTheme(theme: Theme) {
    currentTheme.value = theme
    document.documentElement.setAttribute('data-theme', theme)
    localStorage.setItem('theme', theme)
  }

  function initTheme() {
    // Priority: stored > system > light
    const storedTheme = getStoredTheme()
    const theme = storedTheme || getSystemTheme()
    applyTheme(theme)
  }

  function toggleTheme() {
    const newTheme = currentTheme.value === 'light' ? 'dark' : 'light'
    applyTheme(newTheme)
  }

  onMounted(() => {
    if (currentTheme.value === 'light' && !getStoredTheme()) {
      initTheme()
    }
  })

  return {
    currentTheme,
    initTheme,
    toggleTheme,
    applyTheme,
  }
}
