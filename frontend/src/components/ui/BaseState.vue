<script setup lang="ts">
const props = withDefaults(defineProps<{
  variant?: 'loading' | 'empty' | 'error' | 'success' | 'warning'
}>(), {
  variant: 'loading'
})
</script>

<template>
  <div :class="['c-state', `c-state--${props.variant}`]">
    <slot />
  </div>
</template>
