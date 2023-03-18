// Copyright (C) 2020 Zilliqa

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https:www.gnu.org/licenses/>.

const state = {
  selected: undefined,
  networks: [
    {
      name: "Simulated ENV",
      url: process.env.VUE_APP_ISOLATED_URL,
      type: "default",
      chainId: 222,
      msgVersion: 1,
    },
    {
      name: "Testnet",
      url: "https://dev-api.zilliqa.com",
      type: "default",
      chainId: 333,
      msgVersion: 1,
    },
    {
      name: "Mainnet",
      url: "https://api.zilliqa.com",
      type: "default",
      chainId: 1,
      msgVersion: 1,
    },
  ],
};

const getters = {
  selected: (state) => state.selected,
  list: (state) => state.networks,
};

const actions = {
  SelectNetwork({ commit, state }, { url }) {
    const network = state.networks.find(function (item) {
      return item.url === url;
    });

    commit("setNetwork", network);
    commit("accounts/setAccount", undefined, { root: true });
    window.EventBus.$emit("refresh-balance");
  },
  AddNetwork({ commit, state, dispatch }, networkDetails) {
    return new Promise((resolve) => {
      if (
        state.networks.find((item) => item.url === networkDetails.url) !==
        undefined
      ) {
        throw new Error(
          `Network with address ${networkDetails.url} already exists.`
        );
      }

      commit("addNetwork", networkDetails);
      dispatch("SelectNetwork", networkDetails);
      resolve(networkDetails);
    });
  },
  RemoveNetwork({ commit, state }, networkDetails) {
    commit("setNetwork", state.networks[0]);
    const networkIndex = state.networks.findIndex(
      (item) => item.url === networkDetails.url
    );

    if (networkIndex !== -1) {
      commit("removeNetwork", networkIndex);
    }
  },
};

const mutations = {
  setNetwork(state, payload) {
    state.selected = payload;
  },
  addNetwork(state, payload) {
    state.networks.push(payload);
  },
  removeNetwork(state, index) {
    state.networks.splice(index, 1);
  },
};

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations,
};
