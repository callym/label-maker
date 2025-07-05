import type { FunctionalComponent } from 'vue';

import {
  ArrowUp,
  ChevronDown,
  ChevronLeft,
  ChevronRight,
  ChevronUp,
  CircleAlert,
  CircleCheckBig,
  CircleX,
  Eye,
  EyeOff,
  type IconNode,
  ImageUp,
  Info,
  Loader,
  type LucideProps,
  Paintbrush,
  Trash2,
  X,
} from 'lucide-vue-next';

export type Size = 'small' | 'medium' | 'large';

type IconComponent = FunctionalComponent<LucideProps>;

export type Icon = string | IconComponent | IconNode;

export const icons = new Map<string, Icon>([
  // oruga internal icons
  ['check', CircleCheckBig],
  ['information', Info],
  ['alert', CircleAlert],
  ['alert-circle', CircleAlert],
  ['arrow-up', ArrowUp],
  ['chevron-right', ChevronRight],
  ['chevron-left', ChevronLeft],
  ['chevron-down', ChevronDown],
  ['eye', Eye],
  ['eye-off', EyeOff],
  ['caret-down', ChevronDown],
  ['caret-up', ChevronUp],
  ['loading', Loader],
  ['times', X],
  ['close-circle', CircleX],
  // image uploader
  ['upload', ImageUp],
  ['delete', Trash2],
  // other
  ['paintbrush', Paintbrush],
]);
