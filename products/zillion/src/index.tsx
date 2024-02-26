import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './app';
import * as serviceWorker from './serviceWorker';
import 'bootstrap';
import 'bootstrap/dist/css/bootstrap.css';
import './app.css';
import './media-queries.css';
import { BrowserRouter } from 'react-router-dom';
import { Provider } from 'react-redux';
import store from './store/store'
import { CONFIG_LOADED, UPDATE_API_MAX_ATTEMPT, UPDATE_BLOCKCHAIN_EXPLORER, UPDATE_CHAIN_INFO, UPDATE_REFRESH_RATE } from './store/blockchainSlice';
import { getApiMaxRetry, getBlockchainExplorer, getNetworkConfigByEnv, getRefreshRate } from './util/config-json-helper';


const network_config = getNetworkConfigByEnv()
// store the config file info to redux
// to allow other components to read the contract
store.dispatch(UPDATE_CHAIN_INFO({
  proxy: network_config.proxy,
  impl: network_config.impl,
  blockchain: network_config.blockchain,
  staking_viewer: network_config.node_status,
  api_list: network_config.api_list,
}));

store.dispatch(UPDATE_REFRESH_RATE({ refresh_rate: getRefreshRate() }));
store.dispatch(UPDATE_API_MAX_ATTEMPT({ api_max_attempt: getApiMaxRetry() }));
store.dispatch(UPDATE_BLOCKCHAIN_EXPLORER({ blockchain_explorer: getBlockchainExplorer() }));
store.dispatch(CONFIG_LOADED()); // informs saga to start polling data

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <Provider store={store}>
        <App/>
      </Provider>
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
