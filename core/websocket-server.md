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

Once succsfully subscribed, server will echo the query message to the client

- pushed message

```json
{
  "type":"NewBlock",
  "TxBlock":{
    // same as the json object by quering jsonrpc for `GetTxBlock`
  },
  "TxHashes":[
    // same as the json object by querying jsonrpc for `GetTransactionsForTxBlock`
  ]
}
```

### Subscribe Event Log

- query message

```json
{
  "query":"EventLog",
  "addresses":[
    "0x0000000000000000000000000000000000000000",
    "0x0000000000000000000000000000000000000001"
  ]
}
```

Once succesfully subscribed, server will echo the query message to the client

- pushed message

```json
{
  "type":"EventLog",
  "value":
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
      ]
    },
    {
      "address":"0x0000000000000000000000000000000000000001",
      "event_logs":[]
    } // maybe don't need this if there is no event for this address
  ]
}
```

### Unsubscribe 
- query message
```json
{
  "query":"Unsubscribe",
  "type":["EventLog", "NewBlock"]
}
```

If the client subscribed NewBlock only, the server will return
```json
{
  "type":"Unsubscribe",
  "result":["NewBlock"]
}
```

