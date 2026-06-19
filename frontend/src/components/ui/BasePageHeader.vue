<template>
  <div class="l-page__header">
    <div>
      <p v-if="$slots.eyebrow" class="u-text-muted"><slot name="eyebrow" /></p>
      <h1><slot name="title" /></h1>
      <p v-if="$slots.subtitle" class="u-text-muted"><slot name="subtitle" /></p>
    </div>
    <div v-if="$slots.actions">
      <slot name="actions" />
    </div>
  </div>
</template>
