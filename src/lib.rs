use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, PanicOnDefault,
};
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    count: i32,
}

#[near_bindgen]
impl Contract {
    /// Initializes deploying contract state.
    ///
    /// Operator key is unset.
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { count: 0 }
    }

    pub fn increase_may_panic(&mut self, is_panic: bool) {
        self.count += 1;
        if is_panic {
            env::panic_str("contract panic when param is_panic is true");
        }
        env::log_str(&format!(
            "contract value increase 1, current val is:{}",
            self.count
        ));
    }

    pub fn get_counter(&self) -> i32 {
        self.count
    }
}
