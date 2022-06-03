alias local_near='near --keyPath=/home/developer/.near/local/validator_key.json --masterAccount test.near --network_id local --nodeUrl http://127.0.0.1:3030'

# will save keys to directory: ~/.near-credentials/local
local_near create-account near-atomic-contract.test.near
local_near state near-atomic-contract.test.near
# deploy contract
local_near deploy near-atomic-contract.test.near --wasmFile ./res/near_atomic_test.wasm --initFunction new --initArgs '{}'