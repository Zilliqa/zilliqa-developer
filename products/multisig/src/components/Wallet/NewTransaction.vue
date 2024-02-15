<template>
  <div class="add-funds-form" v-if="!isSuccess">
    <h2 class="subtitle mb-4">New Transaction</h2>
    <b-dropdown
      id="dropdown-aria"
      :text="selectedMethod"
      class="mb-2"
      v-if="token.symbol !== 'ZIL'"
    >
      <b-dropdown-item
        v-for="(m, index) of token.methods"
        :key="index"
        :active="selectedMethod === m"
        @click="selectedMethod = m"
      >
        {{ m }}
      </b-dropdown-item>
    </b-dropdown>
     <Transfer
      v-if="selectedMethod === methods.Transfer"
      v-model="tranferModel"
      :symbol="token.symbol"
    />
   <TransferFrom
      v-if="selectedMethod === methods.TransferFrom"
      v-model="tranferFromModel"
      :symbol="token.symbol"
    />
    <Mint
      v-if="selectedMethod === methods.Mint"
      v-model="mintModel"
      :symbol="token.symbol"
    />
    <Burn
      v-if="selectedMethod === methods.Burn"
      v-model="burnModel"
      :symbol="token.symbol"
    />
    <Allowance
      v-if="selectedMethod === methods.DecreaseAllowance"
      v-model="decreaseAllowanceModel"
      :symbol="token.symbol"
    />
    <Allowance
      v-if="selectedMethod === methods.IncreaseAllowance"
      v-model="increaseAllowanceModel"
      :symbol="token.symbol"
    />
    <h2 class="subtitle toggle-advanced-options mb-4" @click="toggleAdvancedOptions">
      Advanced options <i class="fas fa-chevron-down" />
    </h2>
    <div class="advanced-options d-none mb-5">
      <Gas v-model="gas"/>
      <div
        v-if="token.symbol === 'ZIL'"
        class="input-group mb-3"
      >
        <div class="input-group-prepend">
          <span class="input-group-text" id="basic-addon1">
            Tag
          </span>
        </div>
        <input
          type="text"
          class="form-control"
          placeholder="Tag transition"
          aria-label="Tag transition"
          v-model="tag"
        >
      </div>
    </div>
    <div class="buttons">
      <div v-if="isLoading" class="text-white"><i class="fas fa-spinner fa-spin"></i>Please wait while the transaction is deployed.</div>
      <div v-if="!isLoading && !isSuccess">
        <button
          class="btn btn-primary mr-4"
          @click="proceed"
        >
          Submit
        </button>
        <button class="btn btn-outline-secondary" @click="$emit('cancel-new-transaction')">
          Cancel
        </button>
      </div>
    </div>
  </div>
  <success-screen v-else>
    <div class="subtitle text-primary mb-5">
      Transaction has been successfully generated:
      <br />
      <viewblock-link :txid="tx.id" :network="network" />
    </div>
  </success-screen>
</template>

<script>
import Swal from 'sweetalert2';
import Big from 'big.js';

import { bytes, BN, Long } from '@zilliqa-js/util';
import { fromBech32Address } from '@zilliqa-js/crypto';

import { mapGetters } from 'vuex';

import { BDropdown, BDropdownItem } from 'bootstrap-vue';
import SuccessScreen from '@/components/SuccessScreen';
import ViewblockLink from '@/components/ViewblockLink';
import Gas from '@/components/Gas';
import Transfer from '@/components/transfer/Transfer';
import TransferFrom from '@/components/transfer/TransferFrom';
import Mint from '@/components/transfer/Mint';
import Burn from '@/components/transfer/Burn';
import Allowance from '@/components/transfer/Allowance';

Big.PE = 99;

const methods = {
  IncreaseAllowance: 'IncreaseAllowance',
  DecreaseAllowance: 'DecreaseAllowance',
  Transfer: 'Transfer',
  TransferFrom: 'TransferFrom',
  Burn: 'Burn',
  Mint: 'Mint'
};

export default {
  name: 'NewTransaction',
  data() {
    return {
      methods,
      selectedMethod: methods.Transfer,
      destinationError: false,
      tag: '',
      isLoading: false,
      isSuccess: false,
      gas: {
        gasPrice: 2000000000,
        gasLimit: 5000
      },
      burnModel: {
        amount: 0,
        burnAccount: null
      },
      mintModel: {
        amount: 0,
        recipient: null
      },
      tranferModel: {
        amount: 0,
        destination: null
      },
      tranferFromModel: {
        amount: 0,
        destination: null,
        from: null
      },
      decreaseAllowanceModel: {
        amount: 0,
        spender: null
      },
      increaseAllowanceModel: {
        amount: 0,
        spender: null
      }
    };
  },
  components: {
    SuccessScreen,
    Gas,
    ViewblockLink,
    Transfer,
    Allowance,
    TransferFrom,
    Mint,
    Burn,
    BDropdown,
    BDropdownItem
  },
  props: ['zilliqa', 'address', 'token','walletVersion'],
  computed: {
    ...mapGetters('general', {
      network: 'selectedNetwork'
    }),
    data() {
      let _amount;
      let value;
      const _decimals = Big(10).pow(Number(this.token.decimals));

      if (this.token.symbol === 'ZIL') {
        _amount = Big(this.tranferModel.amount);
        value = _amount.mul(_decimals).round();
        const transition = this.walletVersion && this.walletVersion === '1.0' ? 'SubmitTransaction' : 'SubmitNativeTransaction';
        return JSON.stringify({
          _tag: `${transition}`,
          params: [
            {
              vname: 'recipient',
              type: 'ByStr20',
              value: fromBech32Address(this.tranferModel.destination).toLowerCase()
            },
            {
              vname: 'amount',
              type: 'Uint128',
              value: String(value)
            },
            {
              vname: 'tag',
              type: 'String',
              value: `${this.tag}`
            }
          ]
        });
      } else {
          switch (this.selectedMethod) {
          case methods.IncreaseAllowance:
            _amount = Big(this.increaseAllowanceModel.amount);
            value = _amount.mul(_decimals).round();
            return JSON.stringify({
              _tag: 'SubmitCustomTransaction',
              params: [
                {
                  vname: 'contract_address',
                  type: 'ByStr20',
                  value: String(this.token.address).toLowerCase()
                },
                {
                  "type": `${this.address.toLowerCase()}.MultiSigTransition`,
                  "value": {
                      "argtypes": [],
                      "arguments": [
                          `${fromBech32Address(this.increaseAllowanceModel.spender).toLowerCase()}`,
                          `${value}`,
                      ],
                      "constructor": `${this.address.toLowerCase()}.IncreaseAllowance`
                  },
                  "vname": "transaction"
                }
              ]
            });
          case methods.DecreaseAllowance:
            _amount = Big(this.decreaseAllowanceModel.amount);
            value = _amount.mul(_decimals).round();
            return JSON.stringify({
              _tag: 'SubmitCustomTransaction',
              params: [
                {
                  vname: 'contract_address',
                  type: 'ByStr20',
                  value: String(this.token.address).toLowerCase()
                },
                {
                  "type": `${this.address.toLowerCase()}.MultiSigTransition`,
                  "value": {
                      "argtypes": [],
                      "arguments": [
                          `${fromBech32Address(this.decreaseAllowanceModel.spender).toLowerCase()}`,
                          `${value}`,
                      ],
                      "constructor": `${this.address.toLowerCase()}.DecreaseAllowance`
                  },
                  "vname": "transaction"
                }
              ]
            });
          case methods.Transfer:
            _amount = Big(this.tranferModel.amount);
            value = _amount.mul(_decimals).round();
            return JSON.stringify({
              _tag: 'SubmitCustomTransaction',
              params: [
                {
                  vname: 'contract_address',
                  type: 'ByStr20',
                  value: String(this.token.address).toLowerCase()
                },
                {
                  "type": `${this.address.toLowerCase()}.MultiSigTransition`,
                  "value": {
                      "argtypes": [],
                      "arguments": [
                          `${fromBech32Address(this.tranferModel.destination).toLowerCase()}`,
                          `${value}`,
                      ],
                      "constructor": `${this.address.toLowerCase()}.Transfer`
                  },
                  "vname": "transaction"
                }
              ]
            });
          case methods.TransferFrom:
            _amount = Big(this.tranferFromModel.amount);
            value = _amount.mul(_decimals).round();
            return JSON.stringify({
              _tag: 'SubmitCustomTransaction',
              params: [
                {
                  vname: 'contract_address',
                  type: 'ByStr20',
                  value: String(this.token.address).toLowerCase()
                },
                {
                  "type": `${this.address.toLowerCase()}.MultiSigTransition`,
                  "value": {
                      "argtypes": [],
                      "arguments": [
                          `${fromBech32Address(this.tranferFromModel.from).toLowerCase()}`,
                          `${fromBech32Address(this.tranferFromModel.destination).toLowerCase()}`,
                          `${value}`,
                      ],
                      "constructor": `${this.address.toLowerCase()}.TransferFrom`
                  },
                  "vname": "transaction"
                }
              ]
            });
          case methods.Burn:
            _amount = Big(this.burnModel.amount);
            value = _amount.mul(_decimals).round();
            return JSON.stringify({
              _tag: 'SubmitCustomTransaction',
              params: [
                {
                  vname: 'contract_address',
                  type: 'ByStr20',
                  value: String(this.token.address).toLowerCase()
                },
                {
                  "type": `${this.address.toLowerCase()}.MultiSigTransition`,
                  "value": {
                      "argtypes": [],
                      "arguments": [
                          `${fromBech32Address(this.burnModel.burnAccount).toLowerCase()}`,
                          `${value}`,
                      ],
                      "constructor": `${this.address.toLowerCase()}.Burn`
                  },
                  "vname": "transaction"
                }
              ]
            });
          case methods.Mint:
            _amount = Big(this.mintModel.amount);
            value = _amount.mul(_decimals).round();
            return JSON.stringify({
              _tag: 'SubmitCustomTransaction',
              params: [
                {
                  vname: 'contract_address',
                  type: 'ByStr20',
                  value: String(this.token.address).toLowerCase()
                },
                {
                  "type": `${this.address.toLowerCase()}.MultiSigTransition`,
                  "value": {
                      "argtypes": [],
                      "arguments": [
                          `${fromBech32Address(this.mintModel.recipient).toLowerCase()}`,
                          `${value}`,
                      ],
                      "constructor": `${this.address.toLowerCase()}.Mint`
                  },
                  "vname": "transaction"
                }
              ]
            });
          default:
            return '';
        }
      }
    }
  },
  methods: {
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

      try {
        const VERSION = bytes.pack(this.network.chainId, this.network.msgVersion);
        let tx = this.zilliqa.transactions.new({
          version: VERSION,
          toAddr: this.address,
          amount: new BN(0),
          gasPrice: new BN(this.gas.gasPrice),
          gasLimit: Long.fromNumber(this.gas.gasLimit),
          data: this.data
        });

        EventBus.$emit('sign-event', tx);
      } catch {
        //
      }
    },
    viewblock(txid) {
      let link = `https://viewblock.io/zilliqa/tx/${txid}`;

      if (this.network.name === "ZilPay") {
        link += `?network=${this.zilliqa.wallet.net}`;
      }
      
      if (this.network.url === 'https://dev-api.zilliqa.com') {
        link += '?network=testnet';
      }

      return link;
    }
  },
  async mounted() {
    EventBus.$on('sign-success', async tx => {
      this.isLoading = true;
      if (tx.error) {
        Swal.fire({
          type: 'Error',
          html: 'tx.error'
        }).then(() => {
          window.location.reload();
        });
      } else if (tx.ledger !== true) {
        const result = await this.zilliqa.blockchain.getTransaction(tx.id);
        tx.receipt = result.receipt;
        if (tx.id !== undefined && tx.receipt.success === true) {
          Swal.fire({
            type: 'success',
            html: `Transaction has been successfully sent <a target="_blank" href="${this.viewblock(tx.id)}">${tx.id}</a>`
          }).then(() => {
            window.location.reload();
          });
        } else {
          Swal.fire({
            type: 'error',
            html: `Transaction has been rejected <a target="_blank" href="${this.viewblock(tx.id)}">${tx.id}</a>`
          }).then(() => {
            window.location.reload();
          });
        }
      } else {
        Swal.fire({
          type: 'success',
          html: `Transaction has been successfully sent <a target="_blank" href="${this.viewblock(tx.id)}">${tx.id}</a>`
        }).then(() => {
          window.location.reload();
        });
      }
    });

    EventBus.$on('sign-error', async tx => {
      console.log(tx);
    });
  }
};
</script>

<style lang="scss" scoped>
.toggle-advanced-options {
  cursor: pointer;
  font-size: 14px;
}

.advanced-options {
  display: flex;
  flex-direction: column;
}
</style>