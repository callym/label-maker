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
    ></ImageView>
  </div>
</template>

<style lang="scss" scoped>
.images {
  display: flex;
  flex-wrap: wrap;
  gap: 1em;
}

img {
  border: 1px solid black;
}
</style>
