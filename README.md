# near-atomic-test
## build contract
```shell
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p res
cp ./target/wasm32-unknown-unknown/release/near_atomic_test.wasm ./res/
```
## testnet
* redeploy
  * if redeploy contract, delete the contract first
```shell
near delete near-atomic-test.yongchun.testnet yongchun.testnet
```
* create account and init balance
```shell
near create-account near-atomic-test.yongchun.testnet --masterAccount yongchun.testnet --initialBalance 10
```
* deploy
```shell
near deploy --accountId near-atomic-test.yongchun.testnet  --wasmFile ./res/near_atomic_test.wasm --initFunction new --initArgs '{}'
```

* call
```shell
# no panic
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet increase_may_panic "{\"is_panic\": false}"
```
view the [transaction on explorer](https://explorer.testnet.near.org/transactions/8RJuzqwBANgemNuFwdY72sUkDDbNmXQ4M7BaLu7jmT29)
# query result
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet get_counter "{}"
```
view the [transaction on explorer](https://explorer.testnet.near.org/transactions/9HhYxZd9zeFhwAtBfSMnEQhiZjjFPZcb1Rrv5t9rR23f)
# panic
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet increase_may_panic "{\"is_panic\": true}"
```
view the [transaction on explorer](https://explorer.testnet.near.org/transactions/8J2G4paLFuce5zKkH2owcWioSMJ2wivNbQ8vnbFjrnbp#FWTMUhnRGo2ui1fV59NCaWDhRxf2zzTeECpvkRq6wcL9)
# query result again
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet get_counter "{}"
```
view the [transaction on explorer](https://explorer.testnet.near.org/transactions/3MwRwi5BdzrhQqNr7RDfrPr5VkgnzTvbgHVJffQDxSg9)
## Promise call
### then
#### success case:
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet send_native_with_transfer_state "{\"user\": \"yongchun.testnet\", \"amount\": \"100000000\", \"is_success\": true}"
```
view the [transaction on explorer](https://explorer.testnet.near.org/transactions/AB2YVp5W8vnxMjsPwMDNV5ynEW9GkiquPdS3EhTYF2tz)
* Result: counter increase by two(from 0 to 2): query the current value [transaction on explorer](https://explorer.testnet.near.org/transactions/9bS8sdhZosLMxp1ydcQCXxnHbWBgAXN3ur9bNLUDXd2n)
#### with function_call fail case:
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet send_native_with_transfer_state "{\"user\": \"yongchun.testnet\", \"amount\": \"100000000\", \"is_success\": false}"
```
Receipts are: `Receipts: 9ccMBTydfJXkZiQqP2DNqAMPChcfygNX6htSkxcTifW7, 9TynuGyQdjsnKDhL75pppEFvDZHYxDVDcbgn4TV5pg3U, 8gnDDRecbQ7LKzuXkbpL9PrmTeKk7Zx25R6WcS1W71dM`
* Result: counter increase by two(from 2 to 4): query the current value [transaction on explorer](https://explorer.testnet.near.org/transactions/6KVJaVSNXS734X9sGxnSGnNXU3pqBkm3XbQbMVfDJtED)
### promise
* update state and promise ok
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet promise_action_create_sub_acc "{}"
```
view the [transaction on explorer](https://explorer.testnet.near.org/transactions/9Rafq6tS8tfWowxxCf1XN4qMcCGyHgZHNJK2kXn21KGb)
* update state and promise fail(Exceeded the account balance.)
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet promise_actions_with_transfer_insufficient "{}"
```
view the [transaction on explorer](https://explorer.testnet.near.org/transactions/CVxiwEvRS7uCkhdaWWZKTKxZcGrQdnHcQWB71H3brwdM#Hu4PmuZWzHfeSHP39fuc3A4QDFEueaG1j8BUF1VEcN5L)
* Result: 
  * generate log
  * Promise fail and state did not change.

### Promise action
* create account `wallet.yongchun.testnet` to receive native NEAR
```shell
near create-account wallet.yongchun.testnet --masterAccount yongchun.testnet --initialBalance 1
```
### send it near by contract with then promise success.
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet send_native_with_transfer_state "{\"user\": \"wallet.yongchun.testnet\", \"amount\": \"100000000\", \"is_success\": true}"
```
the account's balance increase `100000000:  [tx detail](https://explorer.testnet.near.org/transactions/6Et4hAMCv8E5u2Agie2Ro5DMLboLKuJy3odtRLznoWo4)
### send it near by contract with then promise fail.
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet send_native_with_transfer_state "{\"user\": \"wallet.yongchun.testnet\", \"amount\": \"100000000\", \"is_success\": false}"
```
* Receipts: 7cmx5Z2BCarCXuk4LKR4QKkFPafBtbp9PcpmpJGHZkMh, 3ZKki25rG9RDU2d3DTqMNwma6WJSVBSWFq9JETx9tDPa, 7hzzSAiNheAZ9E3gZpyt5APbqiWFJVKsDNNC1f662d2M
* Result: it's balance increase and contract's counter also increase(from 12 to 14),query [transaction details on explorer](https://explorer.testnet.near.org/transactions/2aYNTgJ4WPCUT5zN1tTd6WZhmZ57Xwe77fsDJNJX8qfD)
### send action fail before then
```shell
 near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet send_native_with_transfer_state "{\"user\": \"wallet.yongchun.testnet\", \"amount\": \"19999508525168242600000000\", \"is_success\": true}"
```
```text
Receipt: 6Ft9gaSquudmQEYdEVS1fq8JMPxPLXT9PZu3zvsiYVSk
        Log [near-atomic-test.yongchun.testnet]: contract value increase 1, current val is:17
        Failure [near-atomic-test.yongchun.testnet]: Error: {"index":0,"kind":{"ExecutionError":"Exceeded the account balance."}}
```
* Result: The first Promise(Send Near) failed, contract counter didn't change, and didn't call next promise