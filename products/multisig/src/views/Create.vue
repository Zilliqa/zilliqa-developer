<template>
  <div class="create-wallet" v-if="isDeployed === false">
    <h1 class="title mb-5">Create a new wallet</h1>
    <p>
      This wallet does not allow adding or removing owners, or changing the number of required
      signatures.
    </p>
    <p class="mb-5">
      WARNING: If a sufficient number of owners lose their private keys, or for any other reason are
      unable or unwilling to sign new transactions, the funds in the wallet will be locked
      forever. It is therefore a good idea to set required_signatures to a value strictly less than
      the number of owners, so that the remaining owners can retrieve the funds should such a
      scenario occur.
    </p>

    <h2 class="subtitle">Add owners</h2>

    <div class="owners-container">
      <div class="owner" v-for="(owner, index) in owners" :key="index">
        <p class="name mb-0">Address</p>
        <span class="address">{{ owner.address }}</span>
        <div class="overlay" @click="removeOwner(index)">Click to remove</div>
      </div>
      <div class="add-owner" @click="addOwner">Add Owner</div>
    </div>

    <div class="text-info small mt-4" v-if="owners.length <= 1">
      You should add minimum 2 owners to a Multisig Wallet.
    </div>

    <div class="create-options mt-5">
      <div class="row">
        <div class="col-12 col-md-3">
          <h2 class="subtitle mb-3">Min. signatures</h2>
          <div class="signature-input">
            <div class="controller minus" @click="signatureMinus" v-if="signatures >= 3">-</div>
            <input type="number" class="mx-1" min="2" :max="owners.length" v-model="signatures" />
            <div class="controller plus" @click="signaturePlus" v-if="signatures < owners.length">
              <span>+</span>
            </div>
          </div>
        </div>
        <div class="col-12 col-md-7">
          <h2 class="subtitle mb-3 toggle-advanced-options"  @click="toggleAdvancedOptions">Advanced options <i class="fas fa-chevron-down"></i></h2>
          <div class="advanced-options d-none mb-5">
            <Gas v-model="gas"/>
          </div>
        </div>
      </div>
    </div>

    <div class="mt-5 d-flex align-items-center">
      <button class="btn btn-primary mr-4" @click="proceed" v-if="!isLoading">Create Wallet</button>

      <div class="loading text-white" v-else>
        <i class="fas fa-spinner fa-spin"></i> Please wait until wallet is deployed.
      </div>
    </div>
  </div>
  <success-screen v-else>
    <div class="subtitle text-primary mb-5">
      Your wallet was deployed with the following address
      <br />
      <span class="text-white">{{ deployedWallet.contractId }}</span>
    </div>
    <router-link
      class="btn btn-primary"
      :to="{ name: 'wallet', params: { address: deployedWallet.contractId } }"
      >Go to wallet now</router-link
    >
  </success-screen>
</template>

<script>
import Swal from 'sweetalert2';

import { Zilliqa } from '@zilliqa-js/zilliqa';
import { BN, Long, bytes, validation } from '@zilliqa-js/util';
import { fromBech32Address, toBech32Address, toChecksumAddress } from '@zilliqa-js/crypto';
import { mapGetters } from 'vuex';
import SuccessScreen from '@/components/SuccessScreen.vue';
import Gas from '@/components/Gas';

import ZIlpayMixin from '@/mixins/zilpay';

// import CODE from '@/smartcontract/multisig_wallet.scilla.js';
import CODE from '@/smartcontract/multisig_wallet_with_zrc2.scilla.js';

export default {
  name: 'CreateWallet',
  mixins: [ZIlpayMixin],
  components: {
    SuccessScreen,
    Gas
  },
  data() {
    return {
      owners: [],
      signatures: 2,
      gas: {
        gasPrice: 2000000000,
        gasLimit: 25000
      },
      isLoading: false,
      isDeployed: false,
      zilliqa: null,
      deployedWallet: {}
    };
  },
  computed: {
    ...mapGetters('general', {
      network: 'selectedNetwork',
      walletType: 'walletType',
      personalAddress: 'personalAddress',
      wallet: 'wallet',
      contractVersion: 'contractVersion'
    })
  },
  methods: {
    toggleAdvancedOptions() {
      const adv = document.querySelector('.advanced-options');

      adv.classList.toggle('d-none');
    },
    buildOwnersTree(list) {
      var nodes = [];

      if (list.length) {
        list.map((row)=>{
          let address = row.address;
          if (validation.isBech32(address)) {
              address = fromBech32Address(address);
          }
          nodes.push(address);
        })
      }

      return nodes;
    },
    signatureMinus() {
      if (this.signatures - 1 >= 2) this.signatures--;
    },
    signaturePlus() {
      if (this.signatures + 1 <= this.owners.length) this.signatures++;
    },
    async addOwner() {
      let { value: address } = await Swal.fire({
        title: 'Add Owner',
        input: 'text',
        inputPlaceholder: 'Enter address'
      });

      try {
        if (validation.isAddress(address) || validation.isBech32(address)) {
          // Validate address to bech32
          if (validation.isAddress(address)) {
            address = toBech32Address(address);
          }

          // Validate for duplicates
          const found = this.owners.find(function(item) {
            return item.address === address;
          });

          if (found !== undefined) {
            throw 'Already added to owners list.';
          }

          this.owners.push({
            address
          });
        } else {
          throw 'Invalid address';
        }
      } catch (error) {
        Swal.fire({
          type: 'error',
          text: error
        });
      }
    },
    removeOwner(index) {
      this.owners.splice(index, 1);
    },
    async proceed() {
      this.isLoading = true;

      try {
        // validations
        if (this.owners.length <= 1) throw 'You should add minimum 2 owners.';
        if (!this.gas.gasPrice) throw 'Gas Price should be set.';
        if (!this.gas.gasLimit) throw 'Gas Limit should be set.';

        const chainId = this.network.chainId; // chainId of the developer testnet
        const msgVersion = this.network.msgVersion; // current msgVersion
        const VERSION = bytes.pack(chainId, msgVersion);

        // Get Minimum Gas Price from blockchain
        const minGasPrice = await this.zilliqa.blockchain.getMinimumGasPrice();
        let myGasPrice = new BN(this.gas.gasPrice); // Gas Price that will be used by all transactions
        const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price

        if (!isGasSufficient)
          myGasPrice = new BN(minGasPrice.result);

        const ownersTree = this.buildOwnersTree([...this.owners]);

        const init = [
          // this parameter is mandatory for all init arrays
          {
            vname: '_scilla_version',
            type: 'Uint32',
            value: '0'
          },
          {
            vname: 'owners_list',
            type: 'List ByStr20',
            value: ownersTree
          },
          {
            vname: 'required_signatures',
            type: 'Uint32',
            value: `${this.signatures}`
          },
          {
            vname: 'contract_version',
            type: 'String',
            value: `${this.contractVersion}`
          }
        ];

        const code = CODE.toString();
        
        const tx = this.zilliqa.transactions.new({
          code,
          version: VERSION,
          toAddr: '0x0000000000000000000000000000000000000000',
          amount: new BN(0),
          gasPrice: myGasPrice, // in Qa
          gasLimit: Long.fromNumber(this.gas.gasLimit),
          data: JSON.stringify(init).replace(/\\"/g, '"'),
          signature: ''
        });
        EventBus.$emit('sign-event', tx);
      } catch (error) {
        Swal.fire({
          type: 'error',
          text: error
        });

        this.isLoading = false;
      }
    },
    checkForHash(hash) {
      return new Promise((resolve) => {
        const interval = setInterval(async() => {
          const cid = await this.zilliqa.blockchain.getContractAddressFromTransactionID(hash);

          if(cid.result) {
            resolve(cid.result);
            clearInterval(interval);
          }
        }, 10000);
      });
    }
  },
  beforeMount() {
    this.owners.push({ address: toBech32Address(this.personalAddress) });
  },
  async mounted() {
    if (this.network.name === "ZilPay") {
      this.zilliqa = window['zilPay'];
    } else {
      this.zilliqa = new Zilliqa(this.network.url);
    }

    EventBus.$on('sign-success', async tx => {
      if (!tx.ledger) {
        const contractId = await this.zilliqa.blockchain.getContractAddressFromTransactionID(tx.id);

        this.isDeployed = true;
        this.isLoading = false;

        const base16 = toChecksumAddress(contractId.result);
        let contractBech32 = toBech32Address(base16);

        this.deployedWallet = {
          transId: tx.id,
          contractId: contractBech32,
          owners_list: this.owners,
          signatures: this.signatures,
          network: this.network.url
        };

        try {
          this.$store.dispatch('wallets/addWallet', this.deployedWallet);
        } catch (error) {
          throw error;
        }
      } else {
        const contractId = await this.checkForHash(tx.id);

        this.isDeployed = true;
        this.isLoading = false;

        const base16 = toChecksumAddress(contractId);
        let contractBech32 = toBech32Address(base16);

        this.deployedWallet = {
          transId: tx.id,
          contractId: contractBech32,
          owners_list: this.owners,
          signatures: this.signatures,
          network: this.network.url
        };

        try {
          this.$store.dispatch('wallets/addWallet', this.deployedWallet);
        } catch (error) {
          throw error;
        }
      }
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