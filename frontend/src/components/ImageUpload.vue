<script setup lang="ts">
import { ref } from 'vue';

import { Image } from '@/api';

const emit = defineEmits<{
  upload: [Image];
}>();

async function new_file(files?: File[]) {
  if (files == null) {
    return;
  }

  for (const file of files) {
    const image = await Image.upload(file);

    emit('upload', image);
  }
}
</script>

<template>
  <o-field>
    <o-upload
      :model-value="[]"
      multiple
      drag-drop
      accept="image/*"
      @update:model-value="new_file"
      expanded
    >
      <div style="text-align: center">
        <p>
          <o-icon icon="upload" size="large" />
        </p>
        <p>Drop your files here or click to upload</p>
      </div>
    </o-upload>
  </o-field>
</template>

<style lang="scss" scoped>
:deep(.o-upload__draggable) {
  border-top-right-radius: 1em;
  border-bottom-left-radius: 1em;
  border-color: var(--oruga-info);
  color: var(--oruga-info);
}
</style>
