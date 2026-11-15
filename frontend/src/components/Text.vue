<script setup lang="ts">
import { ref } from 'vue';

import { Image } from '@/api';

const emit = defineEmits<{
  upload: [Image];
}>();

const text = ref<string>('');

async function submit() {
  console.log(text);

  if (!text.value) {
    return;
  }

  const image = await Image.text(text.value);

  emit('upload', image);
}
</script>

<template>
  <form @submit.prevent="submit">
    <h2>add text</h2>
    <o-field addons>
      <o-input v-model="text" />

      <o-button type="submit">Submit</o-button>
    </o-field>
  </form>
</template>

<style lang="scss" scoped>
.o-field {
  display: flex;
  justify-content: center;
}

:deep(input) {
  border-bottom-left-radius: 0.5em;
}

.o-button {
  border-top-right-radius: 0.5em;
}

form {
  margin-bottom: 1em;
}
</style>
