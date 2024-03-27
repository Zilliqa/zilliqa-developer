import { validation } from '@zilliqa-js/util';
import { fromBech32Address } from "@zilliqa-js/crypto";


const state = {
    network: {
        name: 'Mainnet',
        url: 'https://api.zilliqa.com',
        chainId: 1,
        msgVersion: 1
    },
    networks: [
        {
            name: 'Mainnet',
            url: 'https://api.zilliqa.com',
            chainId: 1,
            msgVersion: 1
        },
        {
            name: 'Testnet',
            url: 'https://dev-api.zilliqa.com',
            chainId: 333,
            msgVersion: 1
        },
        {
            name: 'Isolated Server',
            url: 'http://localhost:5555',
            chainId: 1,
            msgVersion: 1
        }
    ],
    keystore: null,
    address: null,
    login_type: null,
    tokens: [{
        address: null,
        name: 'Zilliqa',
        symbol: 'ZIL',
        decimals: 12
    }],
    contract_version: "2.0"
};

const getters = {
    isLogged: state => {
        return (state.address !== null && state.login_type !== null);
    },
    networks: state => state.networks,
    wallet: state => state.wallet,
    selectedNetwork: state => state.network,
    personalAddress: state => state.address,
    walletType: state => state.login_type,
    getKeystore: state => state.keystore,
    contractVersion: state => state.contract_version,
};

const actions = {
    login({ commit }, { login_type, keystore, address }) {
        let fixedAddress = (validation.isBech32(address)) ? fromBech32Address(address) : address;

        commit('setLoginType', login_type);
        commit('setAddress', fixedAddress);
        commit('setKeystore', keystore);
    },
    logout({ commit }) {
        commit('setWallet', null);
        commit('setAddress', null);
        commit('setLoginType', null);
        commit('setNetwork', {
            name: 'Mainnet',
            url: 'https://api.zilliqa.com',
            chainId: 1,
            msgVersion: 1
        });
    },
    changeNetwork({ commit, state }, url) {
        const network = state.networks.find(function (item) {
            return item.url === url
        });

        commit('setNetwork', network);
    }
};


const mutations = {
    addToken(state, payload) {
        const {address, name, symbol, decimals, methods, wallet} = payload;
        const found = state.tokens.find((t) => t.symbol === symbol);
        const foundIndex = state.tokens.findIndex((t)=> t.symbol === symbol);
        if (!found) {
            state.tokens.push({address, name, symbol, decimals, methods, wallet:[wallet]});
        } else if(found && !found.wallet.includes(wallet)){
            state.tokens[foundIndex].wallet.push(wallet);
        }else{
            throw new Error("Token already added.");
        }
    },
    setNetwork(state, payload) {
        state.network = payload;
    },
    setAddress(state, payload) {
        state.address = payload;
    },
    setKeystore(state, payload) {
        state.keystore = payload
    },
    setLoginType(state, payload) {
        state.login_type = payload;
    },
};


export default {
    namespaced: true,
    state,
    getters,
    actions,
    mutations
}