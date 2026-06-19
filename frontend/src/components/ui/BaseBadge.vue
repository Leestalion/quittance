<script setup lang="ts">
const props = withDefaults(defineProps<{
  tone?: 'default' | 'success' | 'warning' | 'error'
}>(), {
  tone: 'default'
})
</script>

<template>
  <span :class="['c-badge', `c-badge--${props.tone}`]">
    <slot />
  </span>
</template>

<style scoped>
.c-badge--success {
  background: var(--color-success-bg);
  color: var(--color-success-text);
}

.c-badge--warning {
  background: var(--color-warning-bg);
  color: var(--color-warning-text);
}

.c-badge--error {
  background: var(--color-error-bg);
  color: var(--color-error-text);
}
</style>
