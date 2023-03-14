<template>
  <div class="contracts-list mb-4">
    <div class="header d-flex align-items-center mb-2">
      CONTRACTS
      <img
        src="@/assets/contract-import.svg"
        class="img-button ml-3"
        @click="importContract"
      />
    </div>
    <div class="list">
      <div
        class="list-item"
        v-for="contract in list"
        :key="contract.id"
        :class="{'selected' : (selected && contract.contractId === selected.contractId)}"
      >
        <contract-name
          :contract="contract"
          v-on:select-contract="handleSelect"
          :selected="selected && contract.contractId === selected.contractId"
        />
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
import ContractName from "./ContractName";

export default {
  name: "ContractsList",
  data() {
    return {};
  },
  components: { ContractName },
  computed: {
    ...mapGetters("contracts", ["selected", "list"])
  },
  created() {
    
  },
  methods: {
    importContract() {
      // this.$store.dispatch('incrementBy', amount)
      window.EventBus.$emit("open-import-contract");
    },
    handleSelect({ contractId }) {
      window.EventBus.$emit("open-editor-contract", { contractId });
      window.EventBus.$emit("open-call-contract", { contractId });
      //this.$store.dispatch("contracts/SelectContract", { contractId });
    }
  }
};
</script>

<style lang="scss" scoped>
.header {
  background-color: #fff;
  font-weight: 700;
  text-transform: uppercase;
  color: #ea8b0c;
  font-family: "Montserrat";
  font-size: 1rem;
  padding-left: 1rem;
}

.contracts-list {
  .list-item {
    padding-left: 1rem;
    &:hover,
    &.selected {
      background-color: lighten($primary, $amount: 40);
    }
  }
}

.context-menu {
  li {
    a {
      font-size: 14px;
    }
  }
}

.img-button {
  height: 20px;

  &:hover {
    cursor: pointer;
  }
}
</style>