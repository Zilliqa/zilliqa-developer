<template>
  <div>
    <div class="input-group mb-3">
      <div class="input-group-prepend">
        <span class="input-group-text" id="basic-addon1">
          {{ symbol }}
        </span>
      </div>
      <input
        class="form-control"
        placeholder="Amount"
        type="number"
        min="0"
        v-model="amount"
        @input="onInput"
      >
    </div>
    <div class="input-group mb-3">
      <div class="input-group-prepend">
        <span class="input-group-text" id="basic-addon1">zil1</span>
      </div>
      <input
        type="text"
        class="form-control"
        placeholder="recipient"
        v-model="recipient"
        @input="onInput"
      >
    </div>
  </div>
</template>

<script>
import { fromBech32Address } from '@zilliqa-js/crypto';

export default {
  name: 'Mint',
  props: {
    symbol: {
      type: String
    },
    value: {
      type: Object
    }
  },
  data() {
    return {
      amount: 0,
      recipient: null
    }
  },
  methods: {
    onInput() {
      try {
        fromBech32Address(this.recipient);

        this.$emit('input', {
          amount: this.amount,
          recipient: this.recipient
        })
      } catch {
        return null;
      }
    }
  }
}
</script>

<style lang="scss">

</style>
