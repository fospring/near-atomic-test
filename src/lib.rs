use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, PanicOnDefault,
};

use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    price: Decimal,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Decimal {
    #[serde(with = "i128_dec_format")]
    pub multiplier: i128,
    pub decimals: u8,
}

pub(crate) mod i128_dec_format {
    use near_sdk::serde::de;
    use near_sdk::serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &i128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i128, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
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
            price: Decimal {
                multiplier: 0,
                decimals: 0,
            },
        }
    }

    pub fn set_price(&mut self, price: Decimal) {
        self.price = price;
    }

    pub fn get_price(&self) -> Decimal {
        self.price
    }

    pub fn get_cost(&self, quantity: i128, decimals: u8) -> i128 {
        let multiplier = quantity
            .checked_mul(self.price.multiplier)
            .unwrap_or_else(|| env::panic_str("multiply overflow"));
        if decimals >= self.price.decimals {
            return multiplier
                .checked_mul(10_i128.pow((decimals - self.price.decimals) as u32))
                .unwrap_or_else(|| env::panic_str("multiply overflow"));
        }
        multiplier / 10_i128.pow((self.price.decimals - decimals) as u32)
    }

    pub fn cal_cost(quantity: i128, price: Decimal, decimals: u8) -> i128 {
        let multiplier = quantity
            .checked_mul(price.multiplier)
            .unwrap_or_else(|| env::panic_str("multiply overflow"));
        if decimals >= price.decimals {
            return multiplier
                .checked_mul(10_i128.pow((decimals - price.decimals) as u32))
                .unwrap_or_else(|| env::panic_str("multiply overflow"));
        }
        multiplier / 10_i128.pow((price.decimals - decimals) as u32)
    }
}
