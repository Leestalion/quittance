<template>
  <section class="c-card">
    <header v-if="$slots.header" class="c-card__header">
      <slot name="header" />
    </header>
    <div class="c-card__body">
      <slot />
    </div>
    <footer v-if="$slots.footer" class="c-card__footer">
      <slot name="footer" />
    </footer>
  </section>
</template>

<style scoped>
.c-card__header {
  margin-bottom: var(--space-4);
}

.c-card__footer {
  margin-top: var(--space-4);
}
</style>
