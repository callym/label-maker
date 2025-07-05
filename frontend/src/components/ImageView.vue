<script setup lang="ts">
import { computed, ref, watch } from 'vue';

import { Image } from '@/api';
import { CacheBust } from '@/utils/cache_buster';

const bust = new CacheBust();

const props = defineProps<{
  image: Image;
}>();

const emit = defineEmits<{
  delete: [Image];
  refresh: [Image];
}>();

async function delete_image() {
  await props.image.delete();
  emit('delete', props.image);
}

async function invert_image() {
  await props.image.invert();
  bust.refresh();
}

async function threshold_image(threshold?: number) {
  if (threshold == null) {
    return;
  }

  await props.image.set_threshold(threshold);
  bust.refresh();
}

watch(bust.bust, () => emit('refresh', props.image));
</script>

<template>
  <div class="image">
    <img :src="`${image.url}?${bust.bust.value}`" />
    <div class="controls">
      <o-field label="Inverted" horizontal>
        <div>
          <span class="invert">
            {{ image.inverted ? 'Yes' : 'No' }}
            <o-button
              variant="info"
              icon-left="paintbrush"
              @click="invert_image"
            />
          </span>
        </div>
      </o-field>
      <o-field label="Threshold" horizontal>
        <o-slider
          :model-value="image.threshold"
          @change="threshold_image"
          :min="0"
          :max="255"
        />
      </o-field>
    </div>
    <section class="info">
      <o-field label="Filename" horizontal>
        {{ image.file_name }}
      </o-field>
      <o-field label="Dimensions" horizontal>
        {{ image.length_mm.toFixed(0) }}mm
      </o-field>
      <o-field label="Pixels" horizontal>
        {{ image.original_dimensions.width }} &times;
        {{ image.original_dimensions.height }}
      </o-field>
    </section>
    <o-button
      class="delete"
      variant="danger"
      icon-left="delete"
      @click="delete_image"
      expanded
    >
      Delete
    </o-button>
  </div>
</template>

<style lang="scss" scoped>
.image {
  display: grid;
  grid:
    'image image' 1fr
    'info info' min-content
    'controls controls' min-content
    / 1fr 1fr;
  align-items: center;
  padding: 1em;
  border: 1px solid var(--oruga-primary);

  border-top-right-radius: 1em;
  border-bottom-left-radius: 1em;

  & > .controls {
    display: contents;
    grid-area: controls;
  }

  & > img {
    grid-area: image;
    justify-self: center;
    margin-bottom: 1em;
    border: 1px solid gray;
  }

  & .info {
    display: contents;
    grid-area: info;
  }

  & :deep(.o-field--horizontal) {
    display: grid;
    grid-template-columns: subgrid;
    grid-column: 1 / 3;
  }

  .delete {
    grid-column: 1 / 3;
    margin-top: 1em;
  }

  .invert {
    display: flex;

    align-items: center;
    justify-content: space-between;
  }
}
</style>
