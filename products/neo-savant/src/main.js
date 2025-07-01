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

import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import "bootstrap/dist/css/bootstrap.min.css";
import "floating-vue/dist/style.css";
import "animate.css";
import Notifications from "@kyvg/vue3-notification";
import "@imengyu/vue3-context-menu/lib/vue3-context-menu.css";
import ContextMenu from "@imengyu/vue3-context-menu";
import FloatingVue from "floating-vue";
import eventBus from "./utils/event-bus";

window.EventBus = eventBus;
const app = createApp(App)
  .use(router)
  .use(store)
  .use(Notifications)
  .use(FloatingVue)
  .use(ContextMenu)
  .mount("#app");
