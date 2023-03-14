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

import Vue from 'vue'
import Vuex from 'vuex'
import VuexPersist from 'vuex-persist'

import accounts from './modules/accounts'
import networks from './modules/networks'
import files from './modules/files'
import contracts from './modules/contracts'
import events from './modules/events'
import general from './modules/general'
import transactions from './modules/transactions'
// import console from './modules/console'

Vue.use(Vuex)

const vuexPersist = new VuexPersist({
  key: 'savant-ide',
  storage: window.localStorage,
  reducer: state => ({
    accounts: state.accounts,
    contracts: state.contracts,
    events: state.events,
    files: state.files,
    general: {
      editor: state.general.editor
    },
    networks: state.networks,
    transactions: state.transactions
    // getRidOfThisModule: state.getRidOfThisModule (No one likes it.)
  })
})

export default new Vuex.Store({
  modules: {
    accounts,
    networks,
    files,
    contracts,
    events,
    general,
    transactions
    // console
  },
  plugins: [vuexPersist.plugin]
})
