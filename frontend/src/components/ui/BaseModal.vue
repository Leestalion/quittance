<script setup lang="ts">
defineProps<{
  modelValue: boolean
  title?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

function close() {
  emit('update:modelValue', false)
}
</script>

<template>
  <div v-if="modelValue" class="c-modal-overlay" @click.self="close">
    <div class="c-modal">
      <h2 v-if="title">{{ title }}</h2>
      <slot />
    </div>
  </div>
</template>
