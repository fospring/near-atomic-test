use near_sdk::env::panic_str;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseOrValue, ONE_NEAR,
};

pub const ON_TOKEN_TRANSFER_FAILED_COST: Gas = Gas(2 * Gas::ONE_TERA.0);
pub const ON_TOKEN_TRANSFER_COMPLETE_COST: Gas =
    Gas(10 * Gas::ONE_TERA.0 + ON_TOKEN_TRANSFER_FAILED_COST.0);
const SUB_ACC_NAME: &str = "bob";
const TEN_PERCENT_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

#[derive(Debug, BorshStorageKey, BorshSerialize, PartialEq, Eq)]
pub enum StorageKey {
    NumInforms,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldContract {
    count: i32,
    slots: [i32; 10],
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    count: i32,
    slots: [i32; 10],
    num_infos_storage: UnorderedMap<i32, String>,
}

#[near_bindgen]
impl Contract {
    /// Initializes deploying contract state.
    ///
    /// Operator key is unset.
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            count: 0,
            slots: [10; 10],
            num_infos_storage: UnorderedMap::new(StorageKey::NumInforms),
        }
    }

    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        Self {
            count: old_state.count,
            slots: old_state.slots,
            num_infos_storage: UnorderedMap::new(StorageKey::NumInforms),
        }
    }

    pub fn get_slots(&self) -> [i32; 10] {
        self.slots.clone()
    }

    pub fn slot_bit_set_0(&mut self) {
        for slot in &mut self.slots {
            *slot = 0;
        }
    }

    pub fn slot_bit_set_10(&mut self) {
        for slot in &mut self.slots {
            *slot = 10;
        }
    }

    pub fn slot_bit_set_10_panic_idx5(&mut self) {
        for (idx, slot) in self.slots.iter_mut().enumerate() {
            if idx == 5 {
                panic_str("panic at index 6");
            }
            *slot = 10;
            env::log_str(&format!("update idx:{},set to 10", idx));
        }
    }

    pub fn set_num_info(&mut self, num: i32, info: String) {
        self.num_infos_storage.insert(&num, &info);
    }

    pub fn get_num_info_lens(&self) -> u64 {
        self.num_infos_storage.len()
    }

    pub fn increase_may_panic(&mut self, is_panic: bool) {
        self.increase_and_emit_change();
        if is_panic {
            env::panic_str("contract panic when param is_panic is true");
        }
    }

    pub fn get_counter(&self) -> i32 {
        self.count
    }

    #[payable]
    pub fn send_native_with_transfer_state(
        &mut self,
        user: AccountId,
        amount: U128,
        is_success: bool,
    ) -> PromiseOrValue<()> {
        self.increase_and_emit_change();
        let send_native = Promise::new(user.clone()).transfer(amount.0);
        send_native
            .then(
                Promise::new(env::current_account_id()).function_call(
                    "on_token_transfer_complete".into(),
                    json!({ "is_transfer_success": is_success })
                        .to_string()
                        .into_bytes(),
                    0,
                    ON_TOKEN_TRANSFER_COMPLETE_COST,
                ),
            )
            .into()
    }

    // #[private]
    pub fn on_token_transfer_complete(&mut self, is_transfer_success: bool) -> PromiseOrValue<()> {
        self.increase_and_emit_change();
        if is_transfer_success {
            PromiseOrValue::Value(())
        } else {
            Promise::new(env::current_account_id())
                .function_call(
                    "on_token_transfer_failed".into(),
                    json!({}).to_string().into_bytes(),
                    0,
                    ON_TOKEN_TRANSFER_FAILED_COST,
                )
                .into()
        }
    }

    #[private]
    pub fn on_token_transfer_failed() {
        env::panic_str("token transfer has failed");
    }

    pub fn promise_action_create_sub_acc(&mut self) -> PromiseOrValue<()> {
        self.increase_and_emit_change();
        Promise::new(Contract::get_sub_acc())
            .create_account()
            .transfer(ONE_NEAR)
            .add_full_access_key(env::signer_account_pk())
            .into()
    }

    pub fn promise_action_create_acc(&mut self, account: AccountId) -> PromiseOrValue<()> {
        let signer_pk = env::signer_account_pk();
        env::log_str(&format!("signer_pk={:?}, acc:{}", signer_pk, account));
        Promise::new(account)
            .create_account()
            .transfer(TEN_PERCENT_NEAR)
            .add_full_access_key(signer_pk)
            .into()
    }

    pub fn promise_delete_account(&mut self, account: AccountId) -> PromiseOrValue<()> {
        Promise::new(account)
            .delete_account(env::current_account_id())
            .into()
    }

    pub fn promise_actions_with_transfer_insufficient(&mut self) -> PromiseOrValue<()> {
        self.increase_and_emit_change();
        Promise::new(Contract::get_sub_acc())
            .create_account()
            .transfer(ONE_NEAR * 1000_000)
            .add_full_access_key(env::signer_account_pk())
            .into()
    }

    pub fn promise_delete_sub_account(&mut self) -> PromiseOrValue<()> {
        Promise::new(Contract::get_sub_acc())
            .delete_account(env::current_account_id())
            .into()
    }

    pub fn increase_and_transfer(&mut self, amount: U128) -> PromiseOrValue<()> {
        self.increase_and_emit_change();
        Promise::new(Contract::get_sub_acc())
            .transfer(amount.0)
            .into()
    }

    fn get_sub_acc() -> AccountId {
        let parent_acc = env::current_account_id().to_string();
        let created_acc = SUB_ACC_NAME.to_string() + "." + &parent_acc;
        AccountId::new_unchecked(created_acc)
    }

    fn increase_and_emit_change(&mut self) {
        self.count += 1;
        env::log_str(&format!(
            "contract value increase 1, current val is:{}",
            self.count
        ));
    }
}
