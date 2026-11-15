import { createApp } from 'vue';

import '@fontsource/ibm-plex-serif/latin';
import '@fontsource/ibm-plex-serif/latin-italic';
import {
  Button,
  Field,
  Input,
  Icon as OIcon,
  type OrugaConfig,
  Slider,
  Tooltip,
  Upload,
  createOruga,
} from '@oruga-ui/oruga-next';

import App from '@/App.vue';
import Icon from '@/components/Icon.vue';

import '@/assets/main.scss';

const oruga_options: OrugaConfig = {
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

const oruga = createOruga();

oruga.use(Button);
oruga.use(Field);
oruga.use(Input);
oruga.use(OIcon);
oruga.use(Slider);
oruga.use(Tooltip);
oruga.use(Upload);

createApp(App)
  .component('gh-icon', Icon)
  .use(oruga, oruga_options)
  .mount('#app');
