<template>
  <span>
    <UiLabel v-if="isCore" class="ml-1">Core</UiLabel>
    <Icon v-if="isVerified" name="check" class="ml-1" title="Verified" />
  </span>
</template>

<script>
import { toBech32Address } from '@zilliqa-js/zilliqa';

export default {
  props: ['address', 'space'],
  computed: {
    isCore() {
      try {
        return this.space && this.space.members
          ? this.space.members.includes(toBech32Address(this.address))
          : false;
      } catch {
        return false;
      }
    },
    isVerified() {
      return false;
    }
  }
};
</script>
