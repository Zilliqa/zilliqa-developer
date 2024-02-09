<template>
  <div class="add-funds-form" v-if="!isSuccess">
    <h2 class="subtitle mb-5">Add funds</h2>

    <div class="input-group mb-3">
      <div class="input-group-prepend">
        <span class="input-group-text" id="basic-addon1">
          {{selectedToken.symbol}}
        </span>
      </div>
      <input
        class="form-control"
        placeholder="Wallet address"
        type="number"
        v-model="amount"
        @change="checkAmount"
      >
    </div>

    <h2 class="subtitle toggle-advanced-options mb-4" @click="toggleAdvancedOptions">
      Advanced options <i class="fas fa-chevron-down"></i>
    </h2>

    <div class="advanced-options d-none mb-5">
      <Gas v-model="gas"/>
    </div>

    <div class="buttons">
      <div v-if="isLoading" class="loading text-white">
        <i class="fas fa-spinner fa-spin"></i> Please wait while the transaction is confirming.
      </div>
      <div v-if="!isLoading && !isSuccess">
        <button class="btn btn-primary mr-4" @click="proceed">Submit</button>
        <button class="btn btn-outline-secondary" @click="$emit('cancel-add-funds')">Cancel</button>
      </div>

      <div class="text-success mt-5" v-if="isSuccess">
        Transaction has been successfully deployed.
        <br />
        {{ tx.id }}
      </div>
    </div>
  </div>
  <success-screen v-else>
    <div class="subtitle text-primary mb-5">
      Transaction has been successfully deployed:
      <br />
      <viewblock-link :txid="tx.id" :network="network" />
    </div>
  </success-screen>
</template>

<script>
import Swal from 'sweetalert2';
import { mapGetters } from 'vuex';
import { bytes, BN, Long, units } from '@zilliqa-js/util';
import SuccessScreen from '@/components/SuccessScreen';
import ViewblockLink from '@/components/ViewblockLink';
import Gas from '@/components/Gas';

export default {
  name: 'AddFunds',
  props: ['address', 'bech32', 'zilliqa','selectedToken'],
  data() {
    return {
      amount: 0,
      gas: {
        gasPrice: 2000000000,
        gasLimit: 5000
      },
      isLoading: false,
      isSuccess: false,
      txId: null
    };
  },
  components: {
    SuccessScreen,
    ViewblockLink,
    Gas
  },
  computed: {
    ...mapGetters('general', {
      network: 'selectedNetwork',
      personalAddress: 'personalAddress'
    })
  },
  methods: {
    toTokenFormat(num, decimal) {
      let s = num.toString();
      let i = 0;
      while (i < decimal) {
        s = s + "0";
        i++;
      }
      return s;
    },
    checkAmount() {
      if (this.amount <= -1) {
        this.amount = 0;
      }
    },
    toggleAdvancedOptions() {
      const adv = document.querySelector('.advanced-options');

      adv.classList.toggle('d-none');
    },
    async proceed() {
      this.isLoading = true;
      const VERSION = bytes.pack(this.network.chainId, this.network.msgVersion);
      let tx;
      if(this.selectedToken.symbol!='ZIL'){
        tx = this.zilliqa.transactions.new({
          version: VERSION,
          toAddr: this.selectedToken.address,
          amount: new BN(units.toQa(this.amount, units.Units.Zil)),
          gasPrice: new BN(this.gas.gasPrice),
          gasLimit: Long.fromNumber(this.gas.gasLimit),
          data: JSON.stringify({
            _tag: 'Transfer',
            params: [{
              vname: 'to',
              type: 'ByStr20',
              value: this.address.toLowerCase()
            },
            {
              vname: 'amount',
              type: 'Uint128',
              value: `${this.toTokenFormat(this.amount, this.selectedToken.decimals)}`
            }]
          })
        });
      } else {
        tx = this.zilliqa.transactions.new({
          version: VERSION,
          toAddr: this.address,
          amount: new BN(units.toQa(this.amount, units.Units.Zil)),
          gasPrice: new BN(this.gas.gasPrice),
          gasLimit: Long.fromNumber(this.gas.gasLimit),
          data: JSON.stringify({
            _tag: 'AddFunds',
            params: []
          })
        });
      }
      

      EventBus.$emit('sign-event', tx);

      // this.isLoading = false;
    },
    async checkForHash(hash) {
      const cid = await this.zilliqa.blockchain.getContractAddressFromTransactionID(hash);

      if (cid.error !== undefined && cid.error.code === -5) {
        return await this.checkForHash(hash);
      }

      return cid;
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
  async mounted() {
    EventBus.$on('sign-success', async (data) => {
      try {
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
              type: 'error',
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
      } catch {
        //
      }
      this.isLoading = false;
    });
  }
};
</script>

<style lang="scss" scoped>
.toggle-advanced-options {
  cursor: pointer;
  font-size: 14px;
}
.toggle-advanced-options {
  cursor: pointer;
}
</style>