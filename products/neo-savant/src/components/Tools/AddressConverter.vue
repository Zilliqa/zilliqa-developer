<template>
    <el-dialog
    :model-value="show"
    title="Address Convertor"
    width="500"
  >
    <el-row :gutter="20">
      <el-col :span="22">
        <el-input v-model="bech32" clearable placeholder="Bech32 Address"/>
      </el-col>
      <el-col :span="2">
        <el-tooltip placement="top">
          <a class="tooltip-target b3">
            <img src="@/assets/question.svg" />
          </a>

          <template #content>
            To avoid confusion amongst users, the bech32 checksum address format<br />
            shall be used in the GUI of any user-facing application built on Zilliqa<br />
            in order to cearly distinguish a Zilliqa-based address (zil1) from an Ethereum-based address (0x).
            <br />For more details, please read
            <a
              href="https://github.com/Zilliqa/ZIP/blob/master/zips/zip-1.md"
            >[ZIP-1]</a>.
          </template>
        </el-tooltip>
      </el-col>
    </el-row> 

    <el-row :gutter="20">
      <el-col :span="22">
        <el-input v-model="base16" clearable placeholder="Base16 Address"/>
      </el-col>
      <el-col :span="2">
        <el-tooltip placement="top">
          <a class="tooltip-target">
            <img src="@/assets/question.svg" />
          </a>

          <!-- This will be the content of the popover -->
          <template #content>
          The Zilliqa protocol and Scilla uses base16 checksummed addresses (0x)<br/> 
          in its global state and messages. This is the address format to be used by developers<br/>
          when interacting with the Zilliqa protocol.</template>
        </el-tooltip>
      </el-col>
    </el-row>
    <el-button type="primary" @click="handleConversion">Convert</el-button>
  </el-dialog>
</template>

<script setup>
import { toBech32Address, fromBech32Address } from "@zilliqa-js/crypto";
import { validation } from "@zilliqa-js/util";
import { defineProps, ref} from "vue";

const bech32 = ref("");
const base16 = ref("");

defineProps(["show"]);    // Show or hide the dialog

const handleConversion = () => {
  if (bech32.value.length > 0) {
    if (!validation.isBech32(bech32.value)) {
      return "The string is not a valid Bech32 address.";
    } else {
      base16.value = fromBech32Address(bech32.value);
    }
  } else if (base16.value.length > 0) {
    if (!validation.isAddress(base16.value)) {
      return "The string is not a valid Base16 address.";
    } else {
      bech32.value = toBech32Address(base16.value);
    }
  } else {

  }
}
</script>

<style lang="scss" scoped>
.tooltip-target {
  img {
    height: 20px;
  }
}

.el-row {
  margin-bottom: 20px;
}
</style>
