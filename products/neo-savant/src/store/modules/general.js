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
  mainPanel: null,
  appVersion: import.meta.env.PACKAGE_VERSION || "0",
  editor: {
    fontSize: 14,
  },
};

const getters = {
  mainPanel: (state) => state.mainPanel,
  editor: (state) => state.editor,
  appVersion: (state) => {
    return state.appVersion;
  },
};

const actions = {
  ChangeFontSize({ commit }, { fontSize }) {
    commit("updateFontSize", fontSize);
  },
};

const mutations = {
  updateFontSize(state, payload) {
    state.editor.fontSize = payload;
  },
};

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations,
};
