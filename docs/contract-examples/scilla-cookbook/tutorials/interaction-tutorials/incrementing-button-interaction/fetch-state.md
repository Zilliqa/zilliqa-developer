---
sidebar_position: 4
---
# contract State

Review the Javascript that defines [queryState.js](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/queryState.js)

## Retrieve a contracts state

Now we have called a transition and that has set some mutable state on our contract, we can fetch this value and use it for ensuring that the correct value is being returned. The contract state currently says our state for ```total_button_clicks``` is the value ```1```, but we need to pull that value into the backend.

![Docusaurus](/img/tutorials/incrementingbutton/IDE-state.png)

```js
...
// webhook to listen to events emited back to clients
const websocket = "wss://dev-ws.zilliqa.com"
// mutable state field name to monitor
const state_field_to_monitor = "total_count_clicks"
...

// Listen for events from a contract - errors aren't caught
async function ListenForEvents(deployed_contract_base_16) 
{
    const subscriber = zilliqa.subscriptionBuilder.buildEventLogSubscriptions(
      websocket,
      {
        addresses: [
            deployed_contract_base_16
        ],
      },
    );

    console.log("Listener started");
  
    subscriber.emitter.on(MessageType.EVENT_LOG, async (event) => {
        console.log('get new event log: ', JSON.stringify(event)); // this will emit 2/3 times before event emitted

        const current_button_click_count = await zilliqa.blockchain.getSmartContractSubState(
            deployed_contract_base_16,
            state_field_to_monitor
          );
        console.log(current_button_click_count.result);
    });
  
    await subscriber.start();
}
```

We add this method to our implementation

```js
// Application Definition
// DEPLOY a contract, 
// SEND a transaction
// LISTEN to events emitted
// MONITOR mutable state 
async function DeploySendListenMonitor()
{
    deployed_contract_base_16 = await DeployButtonContract();
    bech_32_bystr = toBech32Address(deployed_contract_base_16);
    console.log(`got ${bech_32_bystr} from ${deployed_contract_base_16}`)

    await Promise.all([
        ListenForEvents(deployed_contract_base_16),
        PressTheButton(bech_32_bystr)
    ]);
}
```

```bash
...
Listener started
Pressing the button...
Waiting transaction be confirmed

get new event log:  {"query":"EventLog"}
{ total_count_clicks: '0' }

get new event log:  {"query":"EventLog","value":[{"address":"a69656b54638aa2ffb8c1e59d4b2705fcce177c2","event_logs":[{"_eventname":"PressTheButtonSuccess","params":[{"type":"ByStr20","value":"0x428a2aa43456fe7fd2de66e48c1fbf372ec10eae","vname":"button_presser"}]},{"_eventname":"IncrementCounterSuccess","params":[{"type":"Uint128","value":"0","vname":"pcc"},{"type":"Uint128","value":"1","vname":"ncc"}]},{"_eventname":"NewClickerState","params":[{"type":"ByStr20","value":"0x428a2aa43456fe7fd2de66e48c1fbf372ec10eae","vname":"nc"}]}]}]}
{ total_count_clicks: '1' }
Button pressed by : 0x428A2aA43456FE7fd2De66E48C1fBf372eC10eAE
```
