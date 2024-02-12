<template>
  <div>
    <div class="input-group mb-3">
      <div class="input-group-prepend">
        <span class="input-group-text" id="basic-addon1">
          GasPrice
        </span>
      </div>
      <input
        type="number"
        class="form-control"
        placeholder="GasPrice"
        aria-label="GasPrice"
        :min="minGasPrice"
        v-model="gasPrice"
        @input="onInput"
      >
    </div>
    <div class="input-group mb-3">
      <div class="input-group-prepend">
        <span class="input-group-text" id="basic-addon1">
          GasLimit
        </span>
      </div>
      <input
        type="number"
        class="form-control"
        placeholder="GasLimit"
        aria-label="GasLimit"
        min="1"
        v-model="gasLimit"
        @input="onInput"
      >
    </div>
  </div>
</template>

<script>
const minGasPrice = 2000000000;

export default {
  name: "Gas",
  props: {
    value: {
      type: Object,
      required: false
    }
  },
  data() {
    return {
      minGasPrice: minGasPrice,
      gasPrice: minGasPrice,
      gasLimit: 5000
    }
  },
  methods: {
    onInput() {
      this.$emit('input', {
        gasPrice: this.gasPrice,
        gasLimit: this.gasLimit
      })
    }
  }
}
</script>

<style lang="scss">

</style>
