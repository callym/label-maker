import { createApp } from 'vue';

import '@fontsource/ibm-plex-serif/latin';
import '@fontsource/ibm-plex-serif/latin-italic';
import Oruga, { type OrugaOptions } from '@oruga-ui/oruga-next';

import App from '@/App.vue';
import Icon from '@/components/Icon.vue';

import '@/assets/main.scss';

const oruga_options: OrugaOptions = {
  iconPack: 'lucide',
  customIconPacks: {
    gh: {
      iconPrefix: 'gh-',
      sizes: {
        default: '',
        small: 'sm',
        medium: 'lg',
        large: 'xl',
      },
    },
  },
  iconComponent: 'gh-icon',
};

createApp(App)
  .component('gh-icon', Icon)
  .use(Oruga, oruga_options)
  .mount('#app');
