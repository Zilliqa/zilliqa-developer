# Zilliqa Web-Socket Service

This page describes the protocol, between the Zilliqa Websocket Server and the sdk client, for querying subscription and message pushing.

## Supported data

The following types of data are the current main focus that we want to consider to be supported by ZWS:

- **New TxBlock**. Which includes TxBlock recently generated and hashes of all the transaction being processed within this block.
- **Event log**. Which includes all the event log generated for interested contract address

## Message encoding

For convention, we still use JSON as our encoding format. For example:

### Subscribe New Block

- query message

```json
{
  "query":"NewBlock",
}
```

- pushed message

```json
{
  "TxBlock":{
    ... // same as the json object by quering jsonrpc for `GetTxBlock`
  }
  "TxHashes":{
    ... // same as the json object by querying jsonrpc for `GetRecentTransactions`
  }
}
```

### Subscribe Event Log

- query message

```json
{
  "query":"EventLog",
  "address":{
    "0x0000000000000000000000000000000000000000",
    "0x0000000000000000000000000000000000000001"
  }
}
```

- pushed message

```json
{
  [
    {
      "address":"0x0000000000000000000000000000000000000000",
      "event_logs":[
        {
          "_eventname":"foo1",
          "params":[
            {
              "vname":"bar1",
              "type":"String",
              "value":"abc"
            },
            {
              "vname":"bar2",
              "type":"ByStr32",
              "value":"0x0000000000000000000000000000000000000001"
            }
          ]
        },
        ...
      ]
    },
    ...
  ]
}
```
