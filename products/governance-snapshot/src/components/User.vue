<template>
  <span>
    <a @click="modalOpen = true" class="no-wrap">
      <Avatar :address="address" size="16" class="mr-1" />
      {{ name }}
      <Badges :address="address" :space="space" />
    </a>
    <ModalUser
      :open="modalOpen"
      @close="modalOpen = false"
      :space="space"
      :address="address"
    />
  </span>
</template>

<script>
import { toBech32Address } from '@zilliqa-js/zilliqa';

export default {
  props: ['address', 'space'],
  data() {
    return {
      modalOpen: false
    };
  },
  computed: {
    name() {
      try {
        return this.web3.account.base16 &&
          this.address.toLowerCase() === this.web3.account.base16.toLowerCase()
          ? 'You'
          : this._shorten(toBech32Address(this.address));
      } catch {
        return this._shorten(this.address);
      }
    }
  }
};
</script>
