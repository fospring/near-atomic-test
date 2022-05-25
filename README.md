# near-atomic-test
## build contract
```shell
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```
## testnet
* create account and init balance
```shell
near create-account near-atomic-test.yongchun.testnet --masterAccount yongchun.testnet --initialBalance 1
near send yongchun.testnet near-atomic-test.yongchun.testnet 10
```
* deploy
  * if redeploy, delete the contract first
```shell
near delete near-atomic-test.yongchun.testnet yongchun.testnet
```
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
* success:
```shell
near --accountId yongchun.testnet call near-atomic-test.yongchun.testnet send_native_with_transfer_state "{\"user\": \"yongchun.testnet\", \"amount\": \"100000000\", \"is_success\": true}"
```
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