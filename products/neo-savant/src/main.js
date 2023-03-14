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

import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import "bootstrap/dist/css/bootstrap.min.css";
import "animate.css";
import Notifications from "vue-notification";
import VTooltip from "v-tooltip";

Vue.config.productionTip = false;

window.EventBus = new Vue();

Vue.use(Notifications);
Vue.use(VTooltip);

new Vue({
  router,
  store,
  render: (h) => h(App),
}).$mount("#app");
