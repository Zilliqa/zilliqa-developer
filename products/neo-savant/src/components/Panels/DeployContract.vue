<template>
  <div class="panel-content">
    <div class="header">
      <div class="title">Deploy {{ file.name }}.scilla</div>
      <img
        src="@/assets/close-color.svg"
        @click="handleClose"
        class="close-button-new"
      />
    </div>
    <div class="body p-4">
      <div class="alert alert-info" v-if="abi === undefined">
        Loading contract ABI
      </div>

      <div class="deploy-form" v-if="abi && !signedTx">
        <transaction-parameters
          v-on:input="onTransactionParameters"
        ></transaction-parameters>
        <!-- Initialization parameters -->
        <div class="row mb-4">
          <div class="col-12">
            <p class="font-weight-bold">Initialization parameters</p>
          </div>
          <div
            class="col-12 mb-4"
            v-for="param in abi.params"
            :key="param.vname"
          >
            <contract-input
              :error="param.validationErrors"
              :vname="param.vname"
              :type="param.type"
              :pvalue="param.value"
              v-model="param.value"
            />
          </div>
        </div>
        <!-- Initialization parameters -> needs to be moved to own component -->

        <div class="row mb-4">
          <div class="col-12 mb-4" v-if="account.type === 'keystore'">
            <div>
              <label>Enter your passphrase</label>
              <input
                type="password"
                v-model="passphrase"
                class="form-control"
              />
            </div>
          </div>
          <div class="col-12 d-flex" v-if="!loading">
            <button
              class="btn btn-light text-danger text-small mr-2"
              @click="resetComponent"
            >
              <small>Reset</small>
            </button>
            <button class="btn btn-primary btn-block" @click="handleDeploy">
              <i class="fas fa-paper-plane"></i>
              Deploy Contract
            </button>
          </div>
        </div>
      </div>

      <div class="alert alert-info" v-if="loading">
        {{ loading }}
        <i class="fas fa-spin fa-spinner"></i>
      </div>
      <div class="alert alert-danger" v-if="error">{{ error }}</div>

      <div class="alert" v-if="signedTx">
        <p class="font-weight-bold">Transaction ID</p>
        <explorer-link :txid="signedTx.transId" />
        <p class="font-weight-bold mt-4">Contract Address</p>
        <explorer-link :address="signedTx.contractAddress" />
        <p class="font-weight-bold mt-4">Receipt</p>
        <div
          class="alert"
          :class="{
            'alert-success': signedTx.receipt.success === true,
            'alert-danger': signedTx.receipt.success === false,
          }"
          style="overflow-x: scroll"
        >
          <vue-json-pretty :data="signedTx.receipt"></vue-json-pretty>
        </div>
      </div>

      <div
        class="alert alert-danger"
        v-if="signedTx && signedTx.receipt.errors.length"
      >
        <ul>
          <li v-for="err in signedTx.receipt.errors[0]" :key="err">
            {{ possibleErrors[err] }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script>
import ContractInput from "@/components/Inputs/ContractInput";
import TransactionParameters from "@/components/Inputs/TransactionParameters";
import ExplorerLink from "@/components/UI/ExplorerLink";
import LedgerInterface from "@/utils/ledger-interface";
import TransportWebUsb from "@ledgerhq/hw-transport-webusb";
import TransportU2F from "@ledgerhq/hw-transport-u2f";
import VueJsonPretty from "vue-json-pretty";
import { BN, bytes, Long, units } from "@zilliqa-js/util";
import { Zilliqa } from "@zilliqa-js/zilliqa";
import { mapGetters } from "vuex";
import axios from "axios";

import { validateParams } from "@/utils/validation.js";
import ZilPayMixin from "@/mixins/zilpay";

const MAX_TRIES = 120;

export default {
  mixins: [ZilPayMixin],
  data() {
    return {
      abi: undefined,
      VUE_APP_ISOLATED_URL: process.env.VUE_APP_ISOLATED_URL,
      copied: false,
      init: {},
      amount: 0,
      gasPrice: 2000000000,
      gasLimit: 30000,
      startDeploy: false,
      passphrase: undefined,
      loading: false,
      ledger: false,
      validatedParams: [],
      files: undefined,
      error: false,
      zilliqa: undefined,
      signedTx: undefined,
      generatedKeys: false,
      actionHappening: false,
      txId: undefined,
      watchTries: 0,
      nonce: null,
      publicKey: null,
      possibleErrors: {
        0: "CHECKER_FAILED",
        1: "RUNNER_FAILED",
        5: "NO_GAS_REMAINING_FOUND",
        7: "CALL_CONTRACT_FAILED",
        8: "CREATE_CONTRACT_FAILED",
        9: "JSON_OUTPUT_CORRUPTED",
      },
    };
  },
  components: {
    VueJsonPretty,
    ContractInput,
    TransactionParameters,
    ExplorerLink,
  },
  props: ["file"],
  computed: {
    ...mapGetters("accounts", { account: "selected" }),
    ...mapGetters("networks", { network: "selected" }),
  },
  async mounted() {
    if (this.account === null || this.account === undefined) {
      this.error = "Please select an account first.";
      return;
    }
    this.getContractAbi();

    if (this.zilliqa === undefined) {
      this.zilliqa = new Zilliqa(this.network.url);
    }

    // get minimum gas price from network
    const minimumGasPrice = await this.zilliqa.blockchain.getMinimumGasPrice();
    this.gasPrice = minimumGasPrice.result;
  },
  methods: {
    handleClose() {
      window.EventBus.$emit("close-right-panel");
    },
    async watchTx() {
      if (this.txId !== undefined && this.watchTries <= MAX_TRIES) {
        try {
          const txn = await this.zilliqa.blockchain.getTransaction(this.txId);
          if (txn.receipt) {
            const contractAddress =
              await this.zilliqa.blockchain.getContractAddressFromTransactionID(
                this.txId
              );
            if (contractAddress.result) {
              this.loading = false;
              const contract = {
                transId: this.txId,
                txData: txn,
                contractId: "0x" + contractAddress.result,
                network: this.network.url,
                file_id: this.file.id,
                file_name: this.file.name,
                deployed_by: this.account.address,
                code: this.file.code,
              };

              await this.$store
                .dispatch("contracts/AddContract", contract)
                .then(() => {
                  this.signedTx = {
                    receipt: txn.receipt,
                    transId: this.txId,
                    contractAddress: "0x" + contractAddress.result,
                  };
                });
            } else {
              this.watchTries = this.watchTries + 1;
              await this.watchTx();
            }
          } else {
            this.watchTries = this.watchTries + 1;
            await this.watchTx();
          }
        } catch (error) {
          if (error.code === -20) {
            this.watchTries = this.watchTries + 1;
            setTimeout(async () => {
              await this.watchTx();
            }, 2000);
          }
        }
      }
    },
    async handleLedgerSign(tx) {
      try {
        let transport = null;

        this.error = false;
        this.loading = "Trying to create WebUSB transport.";

        if (await TransportWebUsb.isSupported()) {
          transport = await TransportWebUsb.create();
        } else {
          transport = await TransportU2F.create();
        }

        this.loading = "Connect your Ledger Device and open Zilliqa App.";
        this.ledger = new LedgerInterface(transport);
        this.publicKey = this.account.pubkey;

        let balance = await this.zilliqa.blockchain.getBalance(
          this.account.address
        );

        if (balance.error && balance.error.code === -5) {
          throw new Error("Account has no balance.");
        } else {
          this.nonce = balance.result.nonce;

          const zils = units.fromQa(
            new BN(balance.result.balance),
            units.Units.Zil
          );
          this.loading = `Account balance: ${zils} ZIL`;
          this.generatedKeys = true;

          let nonce = parseInt(this.nonce) + 1;
          this.loading = "";

          const oldp = tx.txParams;
          const newP = {
            version: oldp.version,
            toAddr: oldp.toAddr,
            amount: oldp.amount,
            code: oldp.code,
            data: oldp.data,
            gasLimit: oldp.gasLimit,
            gasPrice: oldp.gasPrice,
            nonce: nonce,
            pubKey: this.publicKey,
            signature: "",
          };

          this.loading = "Sign transaction from the Ledger Device";
          const signed = await this.ledger.signTxn(this.account.keystore, newP);
          const signature = signed.sig;
          this.loading = "Transaction signed, now trying to deploy...";

          const newtx = {
            id: "1",
            jsonrpc: "2.0",
            method: "CreateTransaction",
            params: [
              {
                toAddr: oldp.toAddr,
                amount: oldp.amount.toString(),
                code: oldp.code,
                data: oldp.data,
                gasLimit: oldp.gasLimit.toString(),
                gasPrice: oldp.gasPrice.toString(),
                nonce: nonce,
                pubKey: this.publicKey,
                signature: signature,
                version: oldp.version,
                priority: true,
              },
            ],
          };

          const response = await fetch(this.network.url, {
            method: "POST",
            mode: "cors",
            cache: "no-cache",
            credentials: "same-origin",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(newtx),
          });

          let data = await response.json();

          if (data.error !== undefined) {
            this.actionHappening = false;
            this.error = data.error.message;
            throw new Error(data.error.message);
          }

          if (data.result.TranID !== undefined) {
            this.loading = "Trying to deploy transaction...";
            this.txId = data.result.TranID;
            this.watchTx();
          }

          if (data.result.error !== undefined) {
            this.actionHappening = false;
            this.error = data.result.error.message;
            throw new Error(data.result.error.message);
          }

          transport.close();

          this.actionHappening = false;
        }
      } catch (error) {
        this.errorr = error.message;
      }
    },
    async handleKeystoreSign(tx) {
      try {
        this.loading = "Trying to sign and send transaction...";

        if (this.passphrase === "" || this.passphrase === undefined) {
          throw new Error("Enter your passphrase.");
        }

        await this.zilliqa.wallet.addByKeystore(
          this.account.keystore,
          this.passphrase
        );

        const txn = await this.zilliqa.blockchain.createTransaction(tx);
        this.txId = txn.id;
        this.watchTries = 0;
        await this.watchTx();
      } catch (error) {
        this.error = error.message;
      }
    },
    async handleZilPaySign(tx) {
      try {
        this.loading = "Please sign transaction on ZilPay...";
        const result = await this.signZilPayTx(tx);
        this.loading = "Waiting for transaction to reach the network...";
        this.txId = result.TranID;
        this.watchTries = 0;
        await this.watchTx();
      } catch (err) {
        this.loading = false;
        this.error = err.message;
      }
    },
    async handlePrivateKeySign(tx) {
      try {
        this.loading = "Trying to sign and send transaction...";

        await this.zilliqa.wallet.addByPrivateKey(this.account.keystore);

        const txn = await this.zilliqa.blockchain.createTransaction(tx);
        this.txId = txn.id;
        this.watchTries = 0;
        await this.watchTx();
      } catch (error) {
        this.error = error.message;
      }
    },
    async handleSign(tx) {
      switch (this.account.type) {
        case "ledger":
          this.handleLedgerSign(tx);
          break;
        case "keystore":
          this.handleKeystoreSign(tx);
          break;
        case "privatekey":
          this.handlePrivateKeySign(tx);
          break;
        case "zilpay":
          this.handleZilPaySign(tx);
          break;
        default:
          this.error = "There has been an error in account detection.";
          break;
      }
      if (this.account.type === "ledger") {
        this.handleLedgerSign(tx);
      }
    },
    async resetComponent() {
      this.abi = undefined;
      this.signedTx = undefined;
      this.error = false;
      this.loading = false;
      this.startDeploy = false;
      await this.getContractAbi();
    },
    async handleDeploy() {
      this.error = false;
      const validatedParams = validateParams([...this.abi.params]);

      if (validatedParams.errors) {
        this.abi.params = validatedParams.params;
        this.error = "Please fix the errors in your inputs";
        return false;
      }

      const init = this.abi.params.map((item) => {
        let val = item.value;

        try {
          val = JSON.parse(item.value);

          if (typeof val == "number") {
            val = item.value;
          }
          // eslint-disable-next-line no-empty
        } catch (e) {}

        return {
          vname: item.vname,
          type: item.type.startsWith("ByStr")
            ? item.type.split(" ").shift()
            : item.type,
          value: val,
        };
      });

      init.push({
        vname: "_scilla_version",
        type: "Uint32",
        value: "0",
      });

      try {
        const VERSION = bytes.pack(
          this.network.chainId,
          this.network.msgVersion
        );

        const tx = this.zilliqa.transactions.new(
          {
            version: VERSION,
            toAddr: "0x0000000000000000000000000000000000000000",
            amount: new BN(this.amount),
            gasPrice: new BN(this.gasPrice), // in Qa
            gasLimit: Long.fromNumber(this.gasLimit),
            code: this.file.code,
            data: JSON.stringify(init).replace(/\\"/g, '"'),
          },
          true
        );

        this.handleSign(tx);
      } catch (error) {
        this.loading = false;
        this.error = error.message;

        window.EventBus.$emit("refresh-balance");
      }
    },
    async getContractAbi() {
      axios
        .post(process.env.VUE_APP_SCILLA_CHECKER_URL, {
          code: this.file.code,
        })
        .then((response) => {
          if (response.data.result === "success") {
            const { contract_info } = JSON.parse(response.data.message);

            this.abi = contract_info;

            this.abi.params = this.abi.params.map((item) => {
              return {
                ...item,
                value: "",
              };
            });

            // this.checked = true;
            this.$notify({
              group: "scilla",
              type: "success",
              position: "bottom right",
              title: "Scilla Checker",
              text: "Contract has been successfully checked",
            });
          }
        })
        .catch(() => {
          this.$notify({
            group: "scilla",
            type: "error",
            position: "bottom right",
            title: "Scilla Checker",
            text: "There are errors in your contract. Check the editor.",
          });
        });
    },
    onTransactionParameters(payload) {
      this.amount = parseInt(payload.amount);
      this.gasLimit = parseInt(payload.gasLimit);
      this.gasPrice = parseInt(payload.gasPrice);
    },
  },
};
</script>

<style lang="scss" scoped>
.accounts-list {
  .item {
    border: 1px dashed #ccc;

    background-color: rgba(0, 0, 0, 0.02);
    border-radius: 8px;
    transition: all 0.2s ease-in-out;

    &:hover {
      background-color: rgba(0, 0, 0, 0.1);
      cursor: pointer;
    }
  }
}

.btn {
  border-radius: 0;
}

.address {
  display: flex;
  align-items: center;

  .copy-button {
    margin-left: 0.5rem;
    height: 20px;
    &:hover {
      cursor: pointer;
    }
  }
}
</style>