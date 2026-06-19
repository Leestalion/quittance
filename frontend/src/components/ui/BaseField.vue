<script setup lang="ts">
defineProps<{
  label?: string
  hint?: string
  error?: string
  forId?: string
}>()
</script>

<template>
  <div class="c-form-group">
    <label v-if="label" :for="forId">{{ label }}</label>
    <slot />
    <small v-if="hint && !error" class="u-text-muted">{{ hint }}</small>
    <small v-if="error" class="c-state--error">{{ error }}</small>
  </div>
</template>
