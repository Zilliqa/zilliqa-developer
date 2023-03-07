---
sidebar_position: 8
---

# Events

An event is a piece of data that gets stored on the blockchain and emitted to any consuming client application listening. It is stored in the transaction history on the block the transaction was processed at.

An event is comprised of a number of parameters in the pattern of `vname : value` pairs delimited by `;`. An event must contain the compulsory field \_eventname, and may contain other fields such as the code field in the example above.

```ocaml
e = {_eventname : "EventNameString"; vname : value };
event e
```

:::note
Event calls can be useful for debugging by emitting function parameters and other relevant data
:::

:::tip
It's good convention to emit an event on success or failure with the relevant parameters.
:::

## IncrementingButton

Lets continue to improve IncrementingButton by defining some events to be emitted on successfully pressing the button.

Let's muse on the design of the contract if we defined a singular event within the public transition like the below example.

```ocaml
...

transition PressTheButton()
  IsPreviousClicker _sender;
  SetNewClicker _sender;
  IncrementCounter

  e = {_eventname : "PressedTheButtonSuccess"; button_presser : _sender };
  event e
end
```

Now all calls to the transition `PressTheButton` that do not throw an error by calling the procedures will emit the event `PressedTheButtonSuccess` with the `_senders` address. This event is now able to be listened for and consumed by client applications or audited from.

We might also want to emit events within procedures especially if these are able to be called cross-contract. We identify that the procedure "setters" should have events and implement them as so.

```ocaml
...

procedure IncrementCounter()
  previous_click_count <- total_count_clicks;
  new_click_count = builtin add previous_click_count uint128_one;
  total_count_clicks := new_click_count;

  e = {_eventname : "IncrementCounterSuccess"; pcc:previous_click_count; ncc: new_click_count  };
  event e
end

procedure SetNewClicker(new_clicker: ByStr20)
  current_clicker := new_clicker;

  e = {_eventname : "NewClickerState"; nc:new_clicker};
  event e
end
...
```
