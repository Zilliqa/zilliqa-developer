<template>
  <div class="panel-content">
    <div class="header">
      <div class="title">
        <address-display :address="contractId" />
      </div>
      <img
        src="@/assets/close-color.svg"
        @click="handleClose"
        class="close-button-new"
      />
    </div>
    <div class="body p-4">
      <p class="font-weight-bold mb-0">Contract Address</p>
      <explorer-link :address="contractId" />

      <div class="mt-4 alert alert-info" v-if="!abi">
        Loading contract ABI
        <i class="fas fa-spinner fa-spin"></i>
      </div>

      <div class="contract-transitions mt-4" v-if="abi">
        <p class="font-weight-bold mb-2">Transitions</p>

        <div class="transitions mb-4">
          <button
            class="btn btn-secondary mr-2 mb-2"
            v-for="transition in abi.transitions"
            :key="transition.vname"
            @click="handleTransitionSelect(transition)"
            :class="{ faded: exec && exec.vname !== transition.vname }"
          >
            {{ transition.vname }}
          </button>
        </div>
      </div>

      <div v-if="!exec">
        <p class="mb-0 d-flex align-items-center font-weight-bold">
          Contract State
          <img
            src="@/assets/refresh.svg"
            class="refresh-state-button"
            @click="refreshContractState"
            v-if="contractState"
          />
        </p>
        <div v-if="contractState">
          <vue-json-pretty
            :deep="1"
            :data="contractState"
            v-if="contractStateLength < 50000"
          ></vue-json-pretty>
          <pre style="max-height: 600px; overflow: scroll" v-else>
            {{ contractState }}
          </pre>
        </div>
        <div v-else>
          <i class="fas fa-spinner fa-spin" v-if="refreshingState"></i>
          <button class="btn btn-primary" @click="refreshContractState" v-else>
            Load contract state
          </button>
        </div>
      </div>

      <div class="mt-4" v-if="contractInit && !exec">
        <p class="font-weight-bold mb-0">Contract Init</p>
        <div style="width: 100%; overflow-x: scroll">
          <vue-json-pretty :deep="0" :data="contractInit"></vue-json-pretty>
        </div>
      </div>

      <div class="deploy-form" v-if="abi && exec && !signedTx">
        <div class="row mb-4">
          <div class="col-12">
            <p class="font-weight-bold">Transaction parameters</p>
          </div>
          <div class="col-12 col-md-4">
            <label>Amount (Uint128)</label>
            <input type="text" v-model="amount" class="form-control" />
          </div>
          <div class="col-12 col-md-4">
            <label>Gas Price (Uint128)</label>
            <input type="text" v-model="gasPrice" class="form-control" />
          </div>
          <div class="col-12 col-md-4">
            <label>Gas Limit (Uint128)</label>
            <input type="text" v-model="gasLimit" class="form-control" />
          </div>
        </div>
        <!-- Transition parameters -->
        <div class="row mb-4">
          <div class="col-12">
            <p class="font-weight-bold">
              Transition parameters ({{ exec.vname }})
            </p>
          </div>
          <div
            class="col-12 mb-4"
            v-for="param in exec.params"
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
        <!-- Transition parameters -> needs to be moved to own component -->

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
            <button class="btn btn-primary btn-block" @click="handleCall">
              <i class="fas fa-paper-plane"></i>
              Call transition
            </button>
          </div>
        </div>
      </div>

      <div class="alert alert-info" v-if="loading">
        {{ loading }}
        <i class="fas fa-spin fa-spinner"></i>
      </div>
      <div class="alert alert-danger" v-if="error">{{ error }}</div>

      <div v-if="signedTx">
        <p class="font-weight-bold">Transaction ID</p>
        <explorer-link :txid="signedTx.transId" />
        <p class="font-weight-bold mt-4">Receipt</p>
        <div
          class="alert"
          :class="{
            'alert-success': signedTx.receipt.success === true,
            'alert-danger': signedTx.receipt.success === false,
          }"
          style="overflow-x: scroll"
        >
          <vue-json-pretty
            :data="{
              ...signedTx.receipt,
              event_logs: signedTx.receipt.event_logs
                ? signedTx.receipt.event_logs.length
                : [],
            }"
          ></vue-json-pretty>
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
import ContractInput from "../Inputs/ContractInput";
import AddressDisplay from "../UI/AddressDisplay";
import ExplorerLink from "../UI/ExplorerLink";
import LedgerInterface from "@/utils/ledger-interface";
import VueJsonPretty from "vue-json-pretty";
import TransportWebUsb from "@ledgerhq/hw-transport-webusb";
import TransportU2F from "@ledgerhq/hw-transport-u2f";
import { BN, bytes, Long, units, validation } from "@zilliqa-js/util";
import { Zilliqa } from "@zilliqa-js/zilliqa";
import { mapGetters } from "vuex";
import axios from "axios";
import { validateParams } from "@/utils/validation.js";
import ZilPayMixin from "@/mixins/zilpay";
import { fromBech32Address, toChecksumAddress } from "@zilliqa-js/crypto";

const MAX_TRIES = 120;

export default {
  mixins: [ZilPayMixin],
  data() {
    return {
      VUE_APP_ISOLATED_URL: process.env.VUE_APP_ISOLATED_URL,
      abi: undefined,
      exec: false,
      contractState: undefined,
      contractStateLength: 0,
      contractInit: undefined,
      contractCode: undefined,
      refreshingState: false,
      init: {},
      amount: 0,
      gasPrice: 2000000000,
      gasLimit: 30000,
      startDeploy: false,
      zilliqa: undefined,
      passphrase: undefined,
      validatedParams: [],
      loading: false,
      files: undefined,
      error: false,
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
  components: { VueJsonPretty, ContractInput, ExplorerLink, AddressDisplay },
  props: ["contractId"],
  computed: {
    ...mapGetters("accounts", { account: "selected" }),
    ...mapGetters("contracts", { contract: "selected" }),
    ...mapGetters("networks", { network: "selected" }),
  },
  async mounted() {
    if (this.zilliqa === undefined) {
      this.zilliqa = new Zilliqa(this.network.url);
    }

    // get minimum gas price from network
    const minimumGasPrice = await this.zilliqa.blockchain.getMinimumGasPrice();
    this.gasPrice = minimumGasPrice.result;

    this.contractInit = (
      await this.zilliqa.blockchain.getSmartContractInit(this.contractId)
    ).result;

    //await this.refreshContractState();

    const contractCode = await this.zilliqa.blockchain.getSmartContractCode(
      this.contractId
    );

    this.contractCode = contractCode.result.code;

    this.abi = await this.getContractAbi();
  },
  methods: {
    async resetComponent() {
      this.abi = undefined;
      this.signedTx = undefined;
      this.error = false;
      this.loading = false;
      this.abi = this.getContractAbi();
    },
    handleTransitionSelect(transition) {
      this.exec = transition;
      this.signedTx = undefined;
    },
    handleClose() {
      window.EventBus.$emit("close-right-panel");
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

          if (data.result.TranID !== undefined) {
            this.loading = "Trying to deploy transaction...";
            this.txId = data.result.TranID;
            this.watchTx();
          }

          if (data.error !== undefined) {
            this.actionHappening = false;
            throw new Error(data.error.message);
          }

          if (data.result.error !== undefined) {
            this.actionHappening = false;
            throw new Error(data.result.error.message);
          }

          transport.close();

          this.actionHappening = false;
        }
      } catch (error) {
        this.error = error.message;
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
        this.errror = error.message;
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
        this.errror = error.message;
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
    async watchTx() {
      if (this.txId !== undefined && this.watchTries <= MAX_TRIES) {
        try {
          const txn = await this.zilliqa.blockchain.getTransaction(this.txId);
          if (txn.receipt) {
            if (txn.receipt.success !== false) {
              this.signedTx = {
                receipt: txn.receipt,
                transId: this.txId,
              };
              if (txn.receipt.event_logs && txn.receipt.event_logs.length) {
                await this.$store.dispatch(
                  "events/AddEvents",
                  txn.receipt.event_logs
                );
              }
            }
            this.loading = false;
            window.EventBus.$emit("refresh-balance");
            this.signedTx = { receipt: txn.receipt, transId: this.txId };
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
          } else {
            this.error = error.message;
            throw error;
          }
        }
      }
    },
    async refreshContractState() {
      this.refreshingState = true;
      this.contractState = (
        await this.zilliqa.blockchain.getSmartContractState(this.contractId)
      ).result;
      this.contractStateLength = JSON.stringify(this.contractState).length;
      this.refreshingState = false;
    },
    async handleCancel() {
      this.exec = false;
      this.signedTx = undefined;
      this.loading = false;
      this.error = false;

      await this.refreshContractState();
    },
    getContractAbi() {
      axios
        .post(process.env.VUE_APP_SCILLA_CHECKER_URL, {
          code: this.contractCode,
        })
        .then((response) => {
          if (response.data.result === "success") {
            const { contract_info } = JSON.parse(response.data.message);

            this.abi = contract_info;
          }
        });
    },
    toScillaParams(fields) {
      return Object.keys(fields).map((name) => {
        return {
          vname: name,
          value: this.init[name],
          type: fields[name].type,
        };
      });
    },
    async handleCall() {
      this.error = false;
      const validatedParams = validateParams([...this.exec.params]);

      if (validatedParams.errors) {
        this.exec.params = validatedParams.params;
        this.error = "Please fix the errors in your inputs";
        return false;
      }

      try {
        const chainId = this.network.chainId; // chainId of the developer testnet
        const msgVersion = this.network.msgVersion; // current msgVersion
        const VERSION = bytes.pack(chainId, msgVersion);

        const init = this.exec.params.map((item) => {
          let ret = { vname: item.vname, value: item.value, type: item.type };

          try {
            let val = JSON.parse(item.value);
            if (typeof val == "number") {
              val = item.value;
            }
            ret = { vname: item.vname, value: val, type: item.type };
          } catch (error) {
            ret = { vname: item.vname, value: item.value, type: item.type };
          }

          return {
            vname: ret.vname,
            value: ret.value,
            type: ret.type.startsWith("ByStr")
              ? ret.type.split(" ").shift()
              : ret.type,
          };
        });

        let contractAddress = this.contractId;
        if (validation.isBech32(this.contractId)) {
          contractAddress = fromBech32Address(this.contractId);
        }

        const tx = this.zilliqa.transactions.new(
          {
            version: VERSION,
            toAddr: toChecksumAddress(contractAddress),
            amount: new BN(this.amount),
            gasPrice: new BN(this.gasPrice), // in Qa
            gasLimit: Long.fromNumber(this.gasLimit),
            data: JSON.stringify({
              _tag: this.exec.vname,
              params: init,
            }),
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
  font-size: 0.85rem !important;

  &.faded {
    opacity: 0.5;
  }
}

.refresh-state-button {
  opacity: 0.8;
  height: 16px;
  margin-left: 0.5rem;

  &:hover {
    cursor: pointer;
    opacity: 1;
  }
}
</style>
