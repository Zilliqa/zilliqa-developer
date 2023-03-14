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
    selected: null,
    events: []
};

const getters = {
    selected: state => state.selected,
    list: state => state.events.reverse()
};

const actions = {
    AddEvents({ commit }, eventsList) {
        eventsList.map(ev => {
            commit('add', ev);
        });
    },
    RemoveEvent({ commit }, { index }) {
        commit('remove', { index });
    },
    ClearEvents({ commit }) {
        commit('clear');
    }
};


const mutations = {
    select(state, payload) {
        state.selected = payload;
    },
    add(state, payload) {
        state.events.push(payload);
    },
    remove(state, { index }) {
        state.events.splice(index, 1);
    },
    clear(state) {
        state.events = [];
    }
};


export default {
    namespaced: true,
    state,
    getters,
    actions,
    mutations
}