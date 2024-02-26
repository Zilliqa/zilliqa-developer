---
id: api-introduction
title: Introduction
keywords:
  - api
  - introduction
description: Zilliqa API introduction
---

---

The Zilliqa API is available via
[JSON-RPC](https://en.wikipedia.org/wiki/JSON-RPC), a remote procedure
call protocol encoded in JSON.

All API calls are POST requests.

All requests follow the standard JSON-RPC format and include 4 variables in the
data object:

| Data object | Example             |
| ----------- | :------------------ |
| `id`        | e.g. `"1"`          |
| `jsonrpc`   | e.g. `"2.0"`        |
| `method`    | e.g. `"GetBalance"` |
| `params`    | e.g. `["1"]`        |
