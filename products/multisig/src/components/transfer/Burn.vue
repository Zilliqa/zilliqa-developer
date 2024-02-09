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
        placeholder="Burn account"
        v-model="burnAccount"
        @input="onInput"
      >
    </div>
  </div>
</template>

<script>
import { fromBech32Address } from '@zilliqa-js/crypto';

export default {
  name: 'Burn',
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
      burnAccount: this.value.burnAccount
    }
  },
  methods: {
    onInput() {
      try {
        fromBech32Address(this.burnAccount);

        this.$emit('input', {
          amount: this.amount,
          burnAccount: this.burnAccount
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
