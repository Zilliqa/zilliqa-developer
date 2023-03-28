<template>
  <div class="panel">
    <div class="form-group mb-4">
      <label class="d-flex align-items-center">
        Bech32 Address
        <v-popover offset="16" trigger="hover" placement="top" class="ml-2">
          <!-- This will be the popover target (for the events and position) -->
          <a class="tooltip-target b3">
            <img src="@/assets/question.svg" />
          </a>

          <!-- This will be the content of the popover -->
          <template slot="popover">
            To avoid confusion amongst users, the bech32 checksum address format shall be used in the GUI of any user-facing application built on Zilliqa in order to cearly distinguish a Zilliqa-based address (zil1) from an Ethereum-based address (0x).
            <br />For more details, please read
            <a
              href="https://github.com/Zilliqa/ZIP/blob/master/zips/zip-1.md"
            >[ZIP-1]</a>.
          </template>
        </v-popover>
      </label>
      <input type="text" class="form-control mb-2" :value="bech32" @change="handleBech32Change" />
      <div class="text-danger text-small" v-if="bech32Error">{{ bech32Error }}</div>
    </div>

    <div class="form-group">
      <label class="d-flex align-items-center">
        Base16 Address
        <v-popover offset="16" trigger="hover" placement="top" class="ml-2">
          <!-- This will be the popover target (for the events and position) -->
          <a class="tooltip-target b3">
            <img src="@/assets/question.svg" />
          </a>

          <!-- This will be the content of the popover -->
          <template
            slot="popover"
          >The Zilliqa protocol and Scilla uses base16 checksummed addresses (0x) in its global state and messages. This is the address format to be used by developers when interacting with the Zilliqa protocol.</template>
        </v-popover>
      </label>
      <input type="text" class="form-control mb-2" :value="base16" @change="handleBase16Change" />
      <div class="text-danger text-small" v-if="base16Error">{{ base16Error }}</div>
    </div>
  </div>
</template>

<script>
import { toBech32Address, fromBech32Address } from "@zilliqa-js/crypto";
import { validation } from "@zilliqa-js/util";

export default {
  data() {
    return {
      bech32: "",
      bech32Error: false,
      base16: "",
      base16Error: false
    };
  },
  methods: {
    handleBech32Change(ev) {
      this.bech32Error = false;
      this.bech32 = ev.target.value;

      if (!validation.isBech32(this.bech32)) {
        return (this.bech32Error = "The string is not a valid Bech32 address.");
      } else {
        this.base16 = fromBech32Address(this.bech32);
      }
    },

    handleBase16Change(ev) {
      this.bech32Error = false;
      this.base16 = ev.target.value;

      if (!validation.isAddress(this.base16)) {
        return (this.base16Error = "The string is not a valid Base16 address.");
      } else {
        this.bech32 = toBech32Address(this.base16);
      }
    }
  }
};
</script>

<style lang="scss" scoped>
.tooltip-target {
  img {
    height: 20px;
  }
}
</style>