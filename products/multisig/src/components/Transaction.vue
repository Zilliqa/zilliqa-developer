<template>
  <div class="transaction mb-4" v-if="!isLoading">
    <div class="item">
      <div class="font-weight-bold">ID</div>
      <div class="address-text">{{ transaction.key }}</div>
    </div>
    <div class="item transfer">
      <div class="font-weight-bold">{{ transaction.type }} <span class="badge badge-info">{{ transaction.subType }}</span> </div>
      <div class="details d-flex align-items-center">
        <div class="mr-2 address-text">{{ wallet }}</div>
        <i class="fas fa-arrow-right"></i>
        <div class="address-text ml-2">{{ destination }}</div>
      </div>
    </div>
    <div class="item amount">
      <div class="font-weight-bold">Amount</div>
      <div class="font-weight-bold">{{ amount }} {{ transaction.token.symbol }}</div>
    </div>
    <div class="actions">
      <div>
        <div class="secondary-actions" v-if="canExecute">
          <div class="unsign" v-if="hasSigned" @click="onUnsign">
            <img src="@/assets/Unsign.svg" />
          </div>
          <div class="sign" v-if="!hasSigned" @click="onSign">
            <img src="@/assets/Sign.svg" />
          </div>
        </div>
      </div>
      <div
        class="signatures font-weight-bold"
      >{{ transaction.signatures_count }}/{{ owners.length }}</div>

      <div class="main-actions" v-if="isOwner">
        <div class="action sign" v-if="!hasSigned && !canExecute && !loadingTx" @click="onSign">
          <img src="@/assets/Sign.svg" />
          Sign
        </div>
        <div class="p-1 ml-3 mr-3" v-if="loadingTx">
          <i class="fas fa-spinner fa-spin" />
        </div>
        <div class="action unsign" @click="onUnsign" v-if="hasSigned && !canExecute && !loadingTx">
          <img src="@/assets/Unsign.svg" />
          Unsign
        </div>

        <div class="action execute" @click="onExecute" v-if="canExecute && !loadingTx">
          <img src="@/assets/Execute.svg" />
          Execute
        </div>
      </div>
    </div>
  </div>
  <div class="transaction-loading mb-4" v-else>
    <i class="fas fa-spinner fa-spin"></i>
  </div>
</template>

<script>
import Big from 'big.js';

import numbro from "numbro";
import Swal from "sweetalert2";

import { BN, Long, bytes, units } from "@zilliqa-js/util";
import { fromBech32Address, toBech32Address } from "@zilliqa-js/crypto";
import { mapGetters } from "vuex";

Big.PE = 99;

export default {
  name: "Transaction",
  data() {
    return {
      isLoading: false,
      loadingTx: false
    };
  },
  props: [
    "transaction",
    "wallet",
    "owners",
    "signatures_need",
    "network",
    "zilliqa"
  ],
  computed: {
    ...mapGetters("general", {
      personalAddress: "personalAddress"
    }),
    amount() {
      if (this.transaction.token.symbol !== 'ZIL') {
        let amount;
        if(this.transaction.amount.arguments.length === 3){
          const [,, _amount] = this.transaction.amount.arguments;
          amount = _amount;
        } else {
          const [, _amount] = this.transaction.amount.arguments;
          amount = _amount;
        }
        const _decimals = Big(10).pow(Number(this.transaction.token.decimals));
        const _amount = Big(amount);
        const value = _amount.div(_decimals);

        return numbro(value).format();
      }

      return numbro(
        units.fromQa(new BN(this.transaction.amount), units.Units.Zil)
      ).format();
    },
    destination() {
      if (this.transaction.token.symbol === 'ZIL') {
        return toBech32Address(this.transaction.destination);
      }

      const [to] = this.transaction.amount.arguments;

      return toBech32Address(to);
    },
    hasSigned() {
      const personalAddress = this.personalAddress;
      let found = Object.keys(this.transaction.signatures).find(function(item) {
        return toBech32Address(item) === toBech32Address(personalAddress);
      });

      if (found !== undefined) return true;

      return false;
    },
    canExecute() {
      return this.transaction.signatures_count >= this.signatures_need;
    },
    isOwner(){
      let found =  this.owners.find(owner => owner === this.personalAddress.toLowerCase());
      return found;
    }
  },
  methods: {
    onSign() {
      const VERSION = bytes.pack(this.network.chainId, this.network.msgVersion);

      let tx = this.zilliqa.transactions.new({
        version: VERSION,
        toAddr: fromBech32Address(this.wallet),
        amount: new BN(0),
        gasPrice: new BN(2000000000),
        gasLimit: Long.fromNumber(2000),
        data: JSON.stringify({
          _tag: "SignTransaction",
          params: [
            {
              vname: "transactionId",
              type: "Uint32",
              value: `${this.transaction.key}`
            }
          ]
        })
      });

      this.loadingTx = true;
      EventBus.$emit("sign-event", tx);
    },
    onUnsign() {
      const VERSION = bytes.pack(this.network.chainId, this.network.msgVersion);

      let tx = this.zilliqa.transactions.new({
        version: VERSION,
        toAddr: fromBech32Address(this.wallet),
        amount: new BN(0),
        gasPrice: new BN(2000000000),
        gasLimit: Long.fromNumber(2000),
        data: JSON.stringify({
          _tag: "RevokeSignature",
          params: [
            {
              vname: "transactionId",
              type: "Uint32",
              value: `${this.transaction.key}`
            }
          ]
        })
      });

      this.loadingTx = true;
      EventBus.$emit("sign-event", tx);
    },
    onExecute() {
      const VERSION = bytes.pack(this.network.chainId, this.network.msgVersion);

      let tx = this.zilliqa.transactions.new({
        version: VERSION,
        toAddr: fromBech32Address(this.wallet),
        amount: new BN(0),
        gasPrice: new BN(2000000000),
        gasLimit: Long.fromNumber(9000),
        data: JSON.stringify({
          _tag: "ExecuteTransaction",
          params: [
            {
              vname: "transactionId",
              type: "Uint32",
              value: `${this.transaction.key}`
            }
          ]
        })
      });

      this.loadingTx = true;
      EventBus.$emit("sign-event", tx);
    },
    viewblock(txid) {
      let link = `https://viewblock.io/zilliqa/tx/${txid}`;

      if (this.network.name === "ZilPay") {
        link += `?network=${this.zilliqa.wallet.net}`;
      }

      if(this.network.url === 'https://dev-api.zilliqa.com') {
        link += '?network=testnet';
      }

      return link;
    }
  },
  mounted() {
    EventBus.$on('sign-success', async (data) => {
      if (data.ledger !== true) {
        const tx = await this.zilliqa.blockchain.getTransaction(data.id);

        if (tx && tx.receipt && tx.receipt.success === true) {
          Swal.fire({
            type: 'success',
            html: `Transaction has been successfully sent <a target="_blank" href="${this.viewblock(data.id)}">${data.id}</a>`
          }).then(() => {
            window.location.reload();
          });
        } else {
          Swal.fire({
            type: 'danger',
            html: `Transaction has been rejected <a target="_blank" href="${this.viewblock(data.id)}">${data.id}</a>`
          }).then(() => {
            window.location.reload();
          });
        }
      } else {
        if (data.id) {
          Swal.fire({
            type: 'success',
            html: `Transaction has been successfully sent <a target="_blank" href="${this.viewblock(data.id)}">${data.id}</a>`
          }).then(() => {
            window.location.reload();
          });
        }
      }
      this.loadingTx = false;
    });
  }
};
</script>

<style lang="scss">
.transaction {
  max-width: 590px;
}
.transfer {
  max-width: 250px;
}
</style>
