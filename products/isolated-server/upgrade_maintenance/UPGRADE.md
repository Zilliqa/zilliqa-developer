## Edit the constants.xml
You would need to download the latest constants.xml file that can be found on the mainnet join page
```
curl -O https://mainnet-join.zilliqa.com/seed-configuration.tar.gz
tar -zxvf seed-configuration.tar.gz
vim constants.xml
```

Keep the following tags set to these values
- `<LOOKUP_NODE_MODE>true</LOOKUP_NODE_MODE>`
- `<CHAIN_ID>222</CHAIN_ID>`
- `<NETWORK_ID>3</NETWORK_ID>`
- `<GENESIS_PUBKEY>03B70CF2ABEAE4E86DAEF1A36243E44CD61138B89055099C0D220B58FB86FF588A</GENESIS_PUBKEY>`
- `<ARCHIVAL_LOOKUP>false</ARCHIVAL_LOOKUP>`
- `<ENABLE_SC>true</ENABLE_SC>`
- `<SCILLA_ROOT>/scilla/0/</SCILLA_ROOT>`
- `<INPUT_CODE>input</INPUT_CODE>`
- `<ENABLE_SCILLA_MULTI_VERSION>false</ENABLE_SCILLA_MULTI_VERSION>`
- `<IGNORE_BLOCKCOSIG_CHECK>true</IGNORE_BLOCKCOSIG_CHECK>`
- `<exclusion_list>` is empty

IGNORE the following sections and their subsections
- `<remotestorageDB>`
- `<TXN_PATH/>`
- `<multipliers>`
- `<accounts>`
- `<ds_accounts>`
- `<lookups>`
- `<upper_seed>`
- `<lower_seed>`
- `<ds_guard>`


