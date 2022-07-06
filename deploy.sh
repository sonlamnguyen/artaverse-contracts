RUSTFLAGS='-C link-arg=-s' cargo wasm

# Create user account

./aurad keys add alice

- name: alice
  type: local
  address: aura1x86wp9ys67hyltcy3wmy4g8wkp3x7u98pkd4pj
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Al1GCGN3kVSUvPTeaVGE/DlIj6UI3E+rQEb//DmQt63n"}'
  mnemonic: ""

  ./aurad keys add bob

- name: bob
  type: local
  address: aura15wx8wuzn2ds54ds9tjgs3c09l85g3v5shm3rpm
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A8zoWJ6ntM7r18lAgpyIlVqDeZh+2HJpyWia2GZRcNP5"}'
  mnemonic: ""


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

process embrace eyebrow master south mom huge day coconut unit steel oval reflect cherry sphere valve keen size have vintage machine excess unusual struggle

**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

junk wire admit found steak pottery math swap fade search fame switch fine essay noble submit sword twice bulb dash square gym vapor raven

# Factory Contract -----
./aurad keys add factory-contract

- name: factory-contract
  type: local
  address: aura13dekh86gy2qdk5yhj5678s6nax8n3lat63upgu
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A6REhjNGyJ0EGLGLF28+1n4Q1eQy3vfqP/FOl1LXR3Q1"}'
  mnemonic: ""

**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

dove goat chimney grant label fury whale extend clip ship swamp solid accuse divide avocado decorate ozone youth actress sick expand man climb bunker
# deploy contract
export RPC="https://rpc.serenity.aura.network:443" 
export CHAIN_ID=serenity-testnet-001
export NODE=(--node $RPC)
export TXFLAG=(${NODE} --chain-id ${CHAIN_ID} --gas-prices 0.025uaura --gas auto --gas-adjustment 1.3)
CODE_ID=$(curl "https://rpc.serenity.aura.network/tx?hash=0xA87585316469361DBA1BF93267F84AABC40D7472FC26648E50F24A33722C4845"| jq -r ".result.tx_result.log"|jq -r ".[0].events[-1].attributes[0].value")

# instantiate contract
INIT='{"minter_code_id":1,"cw721_code_id":2}'
./aurad tx wasm instantiate $CODE_ID "$INIT" \
    --from factory-contract --admin "aura13dekh86gy2qdk5yhj5678s6nax8n3lat63upgu" --label "factory-contract" $TXFLAG -y



# ================================================================
# Minter Contract -----
./aurad keys add minter-contract

- name: minter-contract
  type: local
  address: aura12wggxdnmteu0h22jfwn4kny4npzrwjrqks0uhe
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AnktN8mtUhUQThEVan4tE5axEhidyZMav606DmgQ0bAK"}'
  mnemonic: ""

**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

stable song dose shiver battle bid barrel bridge machine ordinary asset aunt asthma mammal gain stadium session general enter kangaroo october sadness chicken swing

#deploy contract
RES=$(./aurad tx wasm store  ./target/wasm32-unknown-unknown/release/minter.wasm --from minter-contract $TXFLAG --output json)

CODE_ID=$(curl "https://rpc.serenity.aura.network/tx?hash=0x7361E1CC290298A49375EBB9B396FDB0754D57CFE03166CF3780FD5CD5B6CE57"| jq -r ".result.tx_result.log"|jq -r ".[0].events[-1].attributes[0].value")

# instantiate contract
INIT='{"base_token_uri":"ipfs://Sdjbfsdkjfgbdkfjgbdsfgbkiufbguydfguybfsdfjkdnsk","num_tokens":100,"max_tokens_per_batch_mint":10,"max_tokens_per_batch_transfer":10,"cw721_code_id": 2,"minter_code_id":1,"name":"INIT_NFT_MINTER","symbol":"INM","royalty_percentage":10,"royalty_payment_address":"creator_address"}'
./aurad tx wasm instantiate $CODE_ID "$INIT" \
    --from minter-contract --admin aura12wggxdnmteu0h22jfwn4kny4npzrwjrqks0uhe --label "minter-contract" $TXFLAG -y
# ----
txhash: 4D163E7F14935D30D8023018F2EBB98E4C507DDA99CE5BEC47EB0DB9249465D7
# ----
# Mint NFT
CONTRACT=$(./aurad query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')

MINT=$(jq -n '{"mint":{"token_id": 1}}')
./aurad tx wasm execute $CONTRACT "$MINT" --from alice $TXFLAG -y

MINT='{"mint":{"token_id": 3, "token_url":"https://serenity.aurascan.io/assets/images/logo/aura-explorer-logo.png"}}'

MINT='{"mint":{"token_id": 5, "token_url":"https://serenity.aurascan.io/assets/images/logo/aura-explorer-logo.png", "token_uri": "ipfs://Sdjbfsdkjfgbdkfjgbdsfgbkiufbguydfguybfsdfjkdnsk", "owner": "aura1x86wp9ys67hyltcy3wmy4g8wkp3x7u98pkd4pj", "extension": { "image": "None", "image_data": "None", "external_url": "None", "description": "None", "name": "None", "attributes": "None", "background_color": "None", "animation_url": "None", "youtube_url": "None", "royalty_percentage": 10, "royalty_payment_address": "None"}}}'


# Mint NFT for alice
MINT_TO='{"mint_to":{"token_id": 4, "token_url":"https://serenity.aurascan.io/assets/images/logo/aura-explorer-logo.png", "recipient":"aura1x86wp9ys67hyltcy3wmy4g8wkp3x7u98pkd4pj"}}'
./aurad tx wasm execute $CONTRACT "$MINT_TO" --from alice $TXFLAG -y

# Transfer NFT for bob
TRANSFER_NFT='{"transfer_nft":{"recipient": "aura15wx8wuzn2ds54ds9tjgs3c09l85g3v5shm3rpm", "token_id": 5}}'
./aurad tx wasm execute $CONTRACT "$TRANSFER_NFT" --from alice $TXFLAG -y

# Query NFT of bob
OWNER_OF='{"owner_of":{"token_id": "10","include_expired": "true"}}'

OWNER_OF='{"owner_of":{"token_id": "10"}}'
./aurad query wasm contract-state smart $CONTRACT "$NUM_TOKENS"  $NODE --output json

OWNER_OF='{"owner_of":{"token_id": "1"}}'

NFT_INFO='{"nft_info":{"token_id": "1"}}'
NUM_TOKENS='{"num_tokens":{}}'
CONTRACT_INFO='{"contract_info":{}}'
ContractInfo

NumTokens

GETCONFIG='{"get_config":{}}'

./aurad query mint

# query
QUERY='{"get_flower":{"id":"f1"}}'
./aurad query wasm contract-state smart $CONTRACT "$QUERY"  $NODE --output json
# {"data":{"flower":{"id":"f1","name":"rose","amount":150,"price":100}}}

 "image": "None", "image_data": "None", "external_url": "None", "description": "None", "name": "None", "attributes": "None", "background_color": "None", "animation_url": "None", "youtube_url": "None", "royalty_percentage": "None", "royalty_payment_address": "None"
