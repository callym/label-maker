<script setup lang="ts">
import { computed } from 'vue';

import { Icon as LucideIcon } from 'lucide-vue-next';
import { P, match } from 'ts-pattern';

import { type Size, icons } from '@/utils/icons';

const props = defineProps<{
  icon: string | [string, string];
  size?: Size;
  spin?: boolean;
}>();

const icon_name = computed(() =>
  match(props.icon)
    .with(P.string, (v) => v)
    .with([P._, P._], ([_, v]) => v)
    .exhaustive(),
);

const icon_value = computed(() => icons.get(icon_name.value));
</script>

<template>
  <template v-if="typeof icon_value === 'function'">
    <component
      :is="icon_value"
      class="icon"
      :class="{ [`o-icon--${size}`]: size, '.o-icon--spin-pulse': spin }"
    ></component>
  </template>
  <template v-else-if="Array.isArray(icon_value)">
    <LucideIcon :name="icon_name" :iconNode="icon_value"></LucideIcon>
  </template>
  <!-- <o-icon v-if="!is_custom" :icon="icon" :size="size" />
  <template v-else>
    <div class="svg" v-html="icons.get(icon)"></div>
  </template> -->
</template>

<style lang="scss" scoped>
.svg {
  display: inline-block;

  width: 1em;
  height: 1em;
  vertical-align: sub;
}

.icon {
  display: inline-block;

  width: 1em;
  max-width: unset;
  height: 1em;
  vertical-align: sub;
}
</style>
