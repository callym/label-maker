<script setup lang="ts">
import { match } from 'ts-pattern';

import { Printer } from '@/api';

const props = defineProps<{ printer: Printer }>();
defineEmits<{ refresh: [] }>();

function width(): string {
  return match(props.printer.media_width)
    .with('ThreePointFive', () => '3.5mm')
    .with('Six', () => '6mm')
    .with('Nine', () => '9mm')
    .with('Twelve', () => '12mm')
    .with('Eighteen', () => '18mm')
    .with('TwentyFour', () => '24mm')
    .with('ThirtySix', () => '36mm')
    .otherwise(() => 'unknown size');
}
</script>

<template>
  <div class="info">
    <div class="fields">
      <div class="field">
        <div class="label">Connected to:</div>
        <div class="message">{{ printer.ty }}</div>
      </div>
      <div class="field">
        <div class="label">Tape type:</div>
        <div class="message">{{ printer.media_width }} ({{ width() }})</div>
      </div>
      <div class="field">
        <div class="label">Tape colour:</div>
        <div class="message">{{ printer.tape_color }}</div>
      </div>
      <div class="field">
        <div class="label">Text colour:</div>
        <div class="message">{{ printer.text_color }}</div>
      </div>
    </div>
    <div class="refresh">
      <o-button variant="prinary" outlined @click="$emit('refresh')"
        >Refresh</o-button
      >
    </div>
  </div>
</template>

<style lang="scss" scoped>
.info {
  display: grid;
  grid:
    'fields fields'
    '. refresh'
    / max-content auto;

  column-gap: 1em;
  justify-content: center;
  padding: 1em;

  border: 1px solid white;
  border-top-right-radius: 1em;
  border-bottom-left-radius: 1em;
}

.fields {
  display: grid;
  grid-template-rows: repeat(auto-fill, 1fr 1fr);
  grid-template-columns: subgrid;
  grid-area: fields;
}

.field {
  display: contents;
}

.label {
  grid-column: 1 / 2;
}

.message {
  grid-column: 2 / 3;
}

.refresh {
  grid-area: refresh;
}
</style>
