<template>
  <div class="contract-actions">
    <div class="content">
      <button href="#" class="btn btn-outline-primary text-primary btn-block" @click="$emit('add-funds')">
        <i class="fas fa-long-arrow-alt-down"></i> Add funds
      </button>
      {{ address }}
    </div>
  </div>
</template>

<script>
import numbro from "numbro";
import { mapGetters } from "vuex";
import { toBech32Address } from '@zilliqa-js/crypto';

export default {
  name: "ContractActions",
  props: ["address", "zilliqa", "balance", "owners_list"],
  computed: {
     ...mapGetters("general", {
      personalAddress: "personalAddress"
    }),
    formattedBalance() {
      return numbro(this.balance).format({ thousandSeparated: true, mantissa:3   });
    },
    isOwner(){
      const personal = toBech32Address(this.personalAddress);
      let found =  this.owners_list.find(owner => owner.address === personal);

      return found;
    }
  }
};
</script>


<style lang="scss" scoped>
.balance {
  font-size: 1.85rem;
  font-weight: 700;
  text-transform: uppercase;
  color: #fff;
}
.btn {
  padding: 1.25rem;
}
.btn-outline-primary {
  &:hover {
    color: #fff !important;
  }
}
</style>