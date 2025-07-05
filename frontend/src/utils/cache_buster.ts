import { type Ref, computed, ref } from 'vue';

export class CacheBust {
  private buster: Ref<number> = ref(0);

  public get bust(): Ref<string> {
    return computed(() => `cache=${this.buster.value}`);
  }

  public constructor() {}

  public refresh() {
    this.buster.value += 1;
  }
}
