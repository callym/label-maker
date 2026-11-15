<script setup lang="ts">
import { computed } from 'vue';

import { Image } from '@/api';
import ImageView from '@/components/ImageView.vue';

defineProps<{ images: Image[] }>();
defineEmits<{ delete: [Image]; refresh: [Image] }>();
</script>

<template>
  <div class="images">
    <ImageView
      v-for="image of images"
      :key="image.id"
      :image="image"
      @delete="$emit('delete', image)"
      @refresh="$emit('refresh', image)"
    />
  </div>
</template>

<style lang="scss" scoped>
.images {
  display: grid;
  grid-template-columns: repeat(4, 25fr [col-start]);
  gap: 1em;
}

img {
  border: 1px solid black;
}
</style>
