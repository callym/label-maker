<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';

import { Image, Printer } from '@/api';
import ImageList from '@/components/ImageList.vue';
import Upload from '@/components/ImageUpload.vue';
import PrinterInfo from '@/components/PrinterInfo.vue';

import { CacheBust } from './utils/cache_buster';

const bust = new CacheBust();

const images = ref<Map<string, Image>>(new Map());
const printer = ref<Printer>();

function add_image(image: Image) {
  images.value.set(image.id, image);
  bust.refresh();
}

function remove_image(image: Image) {
  images.value.delete(image.id);
  bust.refresh();
}

async function load_all() {
  for (const image of await Image.get_images()) {
    add_image(image);
  }
}

onMounted(async () => {
  await load_all();

  printer.value = await Printer.get();
});

async function refresh() {
  printer.value = await Printer.refresh();
}

const image_arr = computed(() => [...images.value.values()]);
const current_width = computed(() =>
  image_arr.value.reduce((curr, img) => curr + img.length_mm, 0),
);

async function print() {
  await Printer.print();
  images.value.clear();
  bust.refresh();
}
</script>

<template>
  <header>
    <h1>label maker!</h1>
    <PrinterInfo
      v-if="printer"
      :printer="printer"
      @refresh="refresh"
    ></PrinterInfo>
  </header>
  <div id="content">
    <Upload @upload="add_image" />

    <ImageList
      v-if="image_arr.length > 0"
      class="image-list"
      :images="image_arr"
      @delete="remove_image"
      @refresh="bust.refresh()"
    ></ImageList>
    <div class="preview" v-if="image_arr.length > 0">
      <h2>preview:</h2>
      <div class="width">
        Queued {{ current_width }}mm ({{ image_arr.length }}
        labels)
      </div>
      <div class="preview-img">
        <img :src="`${Image.preview}?${bust.bust.value}`" />
      </div>
    </div>
    <o-button
      class="print-button"
      @click="print"
      variant="primary"
      :disabled="image_arr.length === 0"
    >
      print!
    </o-button>
  </div>
</template>

<style lang="scss">
#app {
  min-height: 100%;
  padding: 1em;
}
</style>

<style lang="scss" scoped>
header {
  margin-bottom: 0.75rem;
}

.image-list {
  margin-bottom: 0.75rem;
}

.preview {
  display: grid;
  grid:
    'header'
    'info'
    'preview'
    / auto;
  justify-content: center;
  margin-bottom: 0.75rem;
  padding: 1em;
  border: 1px solid var(--oruga-info);
  border-top-right-radius: 1em;
  border-bottom-left-radius: 1em;

  & > h2 {
    grid-area: header;
    margin-bottom: 0.75rem;
  }

  & > .width {
    grid-area: info;

    margin-bottom: 0.75rem;
    text-align: center;
  }

  & > .preview-img {
    display: flex;
    grid-area: preview;
    justify-content: center;

    & > img {
      border: 1px solid gray;
    }
  }
}

.print-button {
  width: 100%;
  border-top-right-radius: 1rem;
  border-bottom-left-radius: 1rem;
  color: black;
  font-style: italic;
  font-weight: bolder;
  font-size: 3em;
}
</style>
