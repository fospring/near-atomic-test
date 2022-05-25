# near-atomic-test
## testnet
* create account and init balance
```shell
near create-account near-atomic-test.yongchun.testnet --masterAccount yongchun.testnet --initialBalance 1
near send yongchun.testnet near-atomic-test.yongchun.testnet 10
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