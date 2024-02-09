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
        placeholder="Spender"
        v-model="spender"
        @input="onInput"
      >
    </div>
  </div>
</template>

<script>
import { fromBech32Address } from '@zilliqa-js/crypto';

export default {
  name: 'Allowance',
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
      spender: null
    }
  },
  methods: {
    onInput() {
      try {
        fromBech32Address(this.spender);

        this.$emit('input', {
          amount: this.amount,
          spender: this.spender
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
