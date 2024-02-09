<template>
  <div class="add-funds-form" v-if="!isSuccess">
    <h2 class="subtitle mb-5">Import Wallet</h2>
    <div class="input-group mb-3">
      <div class="input-group-prepend">
        <span class="input-group-text" id="basic-addon1">
          zil1
        </span>
      </div>
      <input
        class="form-control"
        placeholder="Wallet address"
        type="text"
        v-model="address"
      >
    </div>
    <div class="buttons">
      <div v-if="isLoading" class="text-white">
        <i class="fas fa-spinner fa-spin"></i> Please wait while we verify and import the contract.
      </div>
      <div v-if="!isLoading && !isSuccess">
        <button class="btn btn-primary mr-4" @click="proceed">Import wallet</button>
        <button class="btn btn-outline-secondary" @click="cancelImport">
          Cancel
        </button>
      </div>
    </div>
  </div>

  <success-screen v-else>
    <div class="subtitle text-primary mb-5">
      Wallet successfully imported
      <br />
      <span class="text-white">{{ address }}</span>
    </div>
    <router-link class="btn btn-primary" :to="{ name: 'wallet', params: { address: address } }"
      >Go to wallet now</router-link
    >
  </success-screen>
</template>

<script>
import Swal from 'sweetalert2';
import { mapGetters } from 'vuex';
import { Zilliqa } from '@zilliqa-js/zilliqa';
import { validation } from '@zilliqa-js/util';
import { toBech32Address, fromBech32Address } from '@zilliqa-js/crypto';
import SuccessScreen from '@/components/SuccessScreen.vue';

export default {
  name: 'ImportWallet',
  data() {
    return {
      address: '',
      isSuccess: false,
      isLoading: false,
      deployedWallet: null,
      owners_list: [],
      zilliqa: null
    };
  },
  components: {
    SuccessScreen
  },
  computed: {
    ...mapGetters('general', {
      network: 'selectedNetwork',
      personalAddress: 'personalAddress'
    })
  },
  methods: {
    cancelImport() {
      return this.$router.push('/');
    },
    constructOwners(list) {
      if (list.arguments && list.arguments.length === 2) {
        this.owners_list.push({
          address: toBech32Address(list.arguments[0])
        });

        return this.constructOwners(list.arguments[1]);
      } else if(list.length && toBech32Address(list[0])) {
        list.map((row)=>{
          this.owners_list.push({
            address: toBech32Address(row)
          });
        })
        return true;
      }
      return null;
    },
    async proceed() {
      try {
        this.isLoading = true;

        let address = this.address;

        if (validation.isBech32(address)) {
          address = fromBech32Address(address);
        }

        if (this.network.name === "ZilPay") {
          this.zilliqa = window['zilPay'];
        } else {
          this.zilliqa = new Zilliqa(this.network.url);
        }

        const init = await this.zilliqa.blockchain.getSmartContractInit(address);
        const signatures = init.result.find(item => item.vname === 'required_signatures');
        const owners = init.result.find(item => item.vname === 'owners_list');

        this.constructOwners(owners.value);

        this.deployedWallet = {
          transId: null,
          contractId: toBech32Address(address),
          owners_list: this.owners_list,
          signatures: signatures.value,
          network: this.network.url
        };

        try {
          this.$store.dispatch('wallets/addWallet', this.deployedWallet);
          this.isSuccess = true;
          this.isLoading = false;
        } catch (error) {
          console.error(error);
          Swal.fire({
            type: 'error',
            text: error
          });
        }
      } catch (error) {
        this.isLoading = false;
        Swal.fire({
          type: 'error',
          text: 'Wallet could not be imported. Please check input address.'
        });
      }
    }
  }
};
</script>