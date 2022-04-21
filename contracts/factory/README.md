# Artaverse Factory Contract

Currently, Factory contract only has the simple task of managing the code_id of the related contracts deployed on the
network. We can instantiate a minter contract via factory contract.

## Implementation

* `InstantiateMsg` - Initialize config information for minter management.

```rust
pub struct InstantiateMsg {
  /// code id of minter contract was stored
  pub minter_code_id: u64,
  /// code id of cw721 contract was stored
  pub cw721_code_id: u64,
}
```

* `ExecuteMsg`

```rust
pub enum ExecuteMsg {
  /// update code id of minter contract was stored
  UpdateMinterCodeId {
    minter_code_id: u64,
  },
  /// update code id of cw721 contract was stored
  UpdateCw721CodeId {
    cw721_code_id: u64,
  },
  /// call minter contract to instantiate a minter contract
  CreateMinter {
    minter_instantiate_msg: CreateMinterInstantiateMsg,
  },
}
```

* `QueryMsg::GetState{}` - Query the parameters are set.

```rust
  pub enum QueryMsg {
  /// GetState returns the current state as a json-encoded number
  GetState {},
}
```
## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests:

`cargo test`

Compile:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
```
