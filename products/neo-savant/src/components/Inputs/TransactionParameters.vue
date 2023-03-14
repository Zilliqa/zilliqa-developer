<template>
  <div class="row mb-4">
    <div class="col-12">
      <p class="font-weight-bold">Transaction parameters</p>
    </div>
    <div class="col-12 col-md-4 input-contract">
      <label class="flex-column align-items-start">
        <div class="name">Amount</div>
        <div class="type">
          <input-popover type="Uint128"></input-popover>
        </div>
      </label>
      <input type="text" v-model="amount" @input="updateAmount" class="form-control" />
    </div>
    <div class="col-12 col-md-4 input-contract">
      <label class="flex-column align-items-start">
        <div class="name">Gas Price</div>
        <div class="type">
          <input-popover type="Uint128"></input-popover>
        </div>
      </label>
      <input type="text" v-model="gasPrice" @input="updateAmount" class="form-control" />
    </div>
    <div class="col-12 col-md-4 input-contract">
      <label class="flex-column align-items-start">
        <div class="name">Gas Limit</div>
        <div class="type">
          <input-popover type="Uint128"></input-popover>
        </div>
      </label>
      <input type="text" v-model="gasLimit" @input="updateAmount" class="form-control" />
    </div>
  </div>
</template>

<script>
import InputPopover from "./InputPopover";

export default {
  data() {
    return {
      amount: 0,
      gasPrice: 2000000000,
      gasLimit: 30000
    };
  },
  components: { InputPopover },
  methods: {
    updateAmount() {
      this.$emit("input", {
        amount: this.amount,
        gasPrice: this.gasPrice,
        gasLimit: this.gasLimit
      });
    }
  }
};
</script>

<style lang="scss" scoped>
.input-contract {
  label {
    display: flex;
    align-items: center;
    justify-content: space-between;

    .name {
      color: rgb(143, 142, 142);
      font-weight: bold;
    }
    
    .type {
      font-size: 12px;
    }
  }

   input {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    background-color: #eee;
    border: 0;
    border-radius: 0;
  }
}
</style>