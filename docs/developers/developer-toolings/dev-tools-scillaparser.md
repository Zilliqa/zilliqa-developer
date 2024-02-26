---
id: dev-tools-scillaparser
title: Scilla-Parser
keywords:
  - rust
  - scilla-parser
  - scilla
description: Rust parser for scilla
---

---

## rs-scilla-parser

[rs-scilla-parser](https://github.com/Zilliqa/rs-scilla-parser) is a Rust-based library to parse scilla contracts.

The current version of the library parses a given contract and extract:

- The contract name
- Initial parameters needed to deploy the contract
- The contract's fields
- The contract's transitions

### Source Code

The Github repository can be found at
[https://github.com/Zilliqa/rs-scilla-parser](https://github.com/Zilliqa/rs-scilla-parser)

### Documentation

The official documentation can be found at [docs.rs](https://docs.rs/scilla-parser/latest/scilla_parser/)

### Installation

Run the following command to add rs-scilla-parser to your Rust based project:

```bash
cargo add rs-scilla-parser
```

### Examples

To parse [HelloWorld.scilla](https://github.com/Zilliqa/rs-scilla-parser/blob/main/tests/contracts/HelloWorld.scilla) contract:

```rust
    use scilla_parser::{Contract, Field, FieldList, Transition, TransitionList, Type};

    let contract_path = PathBuf::from("tests/contracts/HelloWorld.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "HelloWorld".to_string(),
            init_params: FieldList(vec![Field::new("owner", Type::ByStr(20))]),
            fields: FieldList(vec![Field::new("welcome_msg", Type::String)]),
            transitions: TransitionList(vec![
                Transition::new("setHello", FieldList(vec![Field::new("msg", Type::String)])),
                Transition::new_without_param("getHello")
            ])
        }
    );
```

For more examples, take a look at the [project's tests](https://github.com/Zilliqa/rs-scilla-parser/blob/main/tests/full_contract_tests.rs).
