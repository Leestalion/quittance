<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
  loading?: boolean
  disabled?: boolean
  type?: 'button' | 'submit' | 'reset'
}>(), {
  variant: 'primary',
  loading: false,
  disabled: false,
  type: 'button'
})

const classes = computed(() => [
  'c-button',
  `c-button--${props.variant}`,
  props.loading ? 'is-loading' : ''
])
</script>

<template>
  <button :type="type" :class="classes" :disabled="disabled || loading">
    <slot>{{ loading ? 'Chargement...' : '' }}</slot>
  </button>
</template>
