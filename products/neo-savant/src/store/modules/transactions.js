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

import uuidv4 from "uuid";

const state = {
  current: undefined,
  transactions: [],
};

const getters = {
  current: (state) => state.current,
  list: (state) => state.transactions,
};

const actions = {
  SelectFile({ commit, state }, payload) {
    const id = payload.id;

    const file = state.files.find(function (item) {
      return item.id === id;
    });

    commit("contracts/unselect", null, { root: true });
    commit("select", file);
  },
  CreateFile({ commit }) {
    const id = uuidv4();

    commit("add", { id, name: "untitled", code: "" });

    return id;
  },
  RenameFile({ commit, state }, { id, name }) {
    const file = state.files.findIndex((item) => item.id === id);

    if (file === undefined) {
      throw Error("File not found.");
    }

    commit("rename", { index: file, name: name });
  },
  UpdateCode({ commit, state }, { id, code }) {
    const index = state.files.findIndex((item) => item.id === id);

    if (index === undefined) {
      throw Error("File not found.");
    }

    commit("updateCode", { index: index, code: code });
  },
  RemoveFile({ commit, state }, { id }) {
    const file = state.files.findIndex((item) => item.id === id);

    if (file === undefined) {
      throw Error("File not found.");
    }

    if (id === state.selected.id) {
      commit("unselect");
    }

    commit("remove", { index: file });
  },
};

const mutations = {
  select(state, payload) {
    state.selected = payload;
  },
  unselect(state) {
    state.selected = undefined;
  },
  add(state, payload) {
    state.files.push(payload);
  },
  rename(state, payload) {
    state.files[payload.index].name = payload.name;
  },
  updateCode(state, payload) {
    state.files[payload.index].code = payload.code;
  },
  remove(state, payload) {
    state.files.splice(payload.index, 1);
  },
};

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations,
};
