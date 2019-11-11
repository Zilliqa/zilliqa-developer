# Zilliqa Web-Socket Service

This page describes the protocol, between the Zilliqa Websocket Server and the sdk client, for querying subscription and message pushing.

## Feature workflow

Client can subscribe their interested topics or unsubscribe certain topic by sending query, if the query failed they will normally be informed immediately with related error message. For every Tx block(epoch), the subscribed content will be sent from server to each client in one message where an array contains all their subscribed topic if updated, which we name **notification**.

## Supported query

The following types of data are the current main focus that we want to consider to be supported by ZWS:

- **New TxBlock**. Which includes TxBlock recently generated and hashes of all the transaction being processed within this block.
- **Event log**. Which includes all the event log generated for interested contract address
- **Unsubscribe**. Which tells the server to unsubscribe certain topic for the client

## Exception handling

Usually an **error message** will be responded to the client if the query failed, it may looks like

```json
{
  "type":"Error",
  "error":"..."
}
```

The following error messages will be applied to all kinds of query if being invalid:
- **invalid query field**. Which tells the client if the query is invalid, it could be not found, empty, malformed, or not available

## Message encoding

For convention, we still use JSON as our encoding format.

The epoch message will be presented in this way:

```json
{
  "type":"notification",
  "data":[
    {
      "query":"...",
      "...":"..."
    },
    {
      "query":"...",
      "...":"..."
    }
  ]
}
```

The followings are case by case for each subscription:

### Subscribe New Block

#### query message

```json
{
  "query":"NewBlock",
}
```

#### response message

Once succsfully subscribed, server will echo the query message to the client, 
otherwise will return error message.

Special error message: 
- **NA**`

#### expected field in notification

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

#### query message

```json
{
  "query":"EventLog",
  "addresses":[
    "0x0000000000000000000000000000000000000000",
    "0x1111111111111111111111111111111111111111"
  ]
}
```

#### response message

Once succesfully subscribed, server will echo the query message to the client,
otherwise will return error message.

Special error message:
- **invalid addresses field**, which tells the client the addresses field is invalid, it could either be not found, malformed or empty
- **no contract found in list**, which tells the client the addresses provided are all of non contract


#### expected field in notification

```json
{
  "query":"EventLog",
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
    }
  ]
}
```
Notice that for address `0x1111111111111111111111111111111111111111` is not presented in the message since it doesn't have any event log released in this epoch.

### Unsubscribe 

#### query message
```json
{
  "query":"Unsubscribe",
  "type":"EventLog"
}
```

#### response message
Once succesfully ubsubscribed, server will echo the query message to the client,
otherwise will return error message.

Special error message:
- **invalid type field**, which tells the client the type field is invalid, if could either be not found, malformed or not available.

#### expected field in notification

```json
{
  "query":"Unsubscribe",
  "quries":["NewBlock", "EventLog"]
}
```
