# Artaverse Minter Contract

Minter Contract Customize CW721 according to the logic of the Artaverse project. It manages the config related to NFT
episode in 1 collection: Base URI, NFT count, minted quantity, etc. and additional information for Royalty.

## Implementation

* `InstantiateMsg` - Initialize config information for mint process management. Details are like comments in source
  code:

```rust
pub struct InstantiateMsg {
  /// base_token_uri of NFTs
  pub base_token_uri: String,
  /// number token of NFTs
  pub num_tokens: u32,
  /// max number token of NFTs can be minted a batch
  pub max_tokens_per_batch_mint: u32,
  /// max number token of NFTs can be transferred a batch
  pub max_tokens_per_batch_transfer: u32,
  /// code id of cw721 was deploy before
  pub cw721_code_id: u64,
  /// name of NFTs
  pub name: String,
  /// symbol of NFTs
  pub symbol: String,
  /// royalty percentage can be received
  pub royalty_percentage: Option<u64>,
  /// royalty addresses
  pub royalty_payment_address: Option<String>,
}
```

* `ExecuteMsg`

```rust
pub enum ExecuteMsg {
  /// Mint a new NFT
  Mint {
    token_id: u32,
  },
  /// Mint a batch of new NFT
  BatchMint {
    token_ids: Vec<u32>,
  },
  /// Mint a new NFT for recipient specified
  MintTo {
    token_id: u32,
    recipient: String,
  },
  /// Transfer is a base message to move a token to another account without triggering actions
  TransferNft {
    recipient: String,
    token_id: u32,
  },

  /// Transfer is a base message to move a batch token to another account without triggering actions
  BatchTransferNft {
    recipient: String,
    token_ids: Vec<u32>,
  },
}
```

* `QueryMsg::Config{}` - Query the parameters are set in Instantiate
* `QueryMsg::...` - other `QueryMsg` inherited from cw721, nft information can be queried based on cw721's [QueryMsg](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md)

If provided, it is expected that the _token_uri_ points to a JSON file following
the [ERC721 Metadata JSON Schema](https://eips.ethereum.org/EIPS/eip-721).

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests:

`cargo test`

Compile:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
```
