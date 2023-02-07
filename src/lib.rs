use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, PanicOnDefault,
};
use std::ops::Sub;

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

impl Decimal {
    /// 数量乘价格
    /// @self: quantity对应的USD价格, USD funding fee
    /// @quantity: 已经乘上精度的数量
    /// @decimals：USDC token的精度
    pub fn cal_cost(&self, quantity: i128, decimals: u8) -> i128 {
        let multiplier = quantity
            .checked_mul(self.multiplier)
            .unwrap_or_else(|| env::panic_str("multiply overflow"));
        if decimals >= self.decimals {
            return multiplier
                .checked_mul(10_i128.pow((decimals - self.decimals) as u32))
                .unwrap_or_else(|| env::panic_str("multiply overflow"));
        }
        multiplier / 10_i128.pow((self.decimals - decimals) as u32)
    }
}

/// funding fee 相减
/// cost_position → cost_position + position_qty * ( sum_unitary_fundings - last_sum_unitary_fundings)
impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.decimals == rhs.decimals {
            Decimal {
                multiplier: self.multiplier - rhs.multiplier,
                decimals: self.decimals,
            }
        } else if self.decimals > rhs.decimals {
            Decimal {
                multiplier: self.multiplier
                    - rhs.multiplier * (self.decimals - rhs.decimals) as i128,
                decimals: self.decimals,
            }
        } else {
            Decimal {
                multiplier: self.multiplier * (rhs.decimals - self.decimals) as i128
                    - rhs.multiplier,
                decimals: rhs.decimals,
            }
        }
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
}

#[cfg(test)]
mod tests {
    use super::Decimal;
    #[test]
    fn test_cal_cost() {
        // 1.56 (10^18)
        let position_qty: i128 = 1_560_000_000_000_000_000;
        let usdc_decimals: u8 = 6;
        // ETH price @1000.6555 USD/ETH, ETH decimals 18
        let price = Decimal {
            multiplier: 10006555,
            decimals: 22,
        };
        let cost = price.cal_cost(position_qty, usdc_decimals);
        // 1561022580 usdc -> 1561.022580 USDC
        let expected_cost = 1561022580;
        assert_eq!(cost, expected_cost);

        // funding fee ETH @1.2222 USD/ETH, ETH decimals 18
        let sum_unitary_long_fundings = Decimal {
            multiplier: 12222,
            decimals: 22,
        };
        let funding_fee = sum_unitary_long_fundings.cal_cost(position_qty, usdc_decimals);
        // 1906632 usdc -> 1.906632 USD
        let expected_funding = 1906632;
        assert_eq!(funding_fee, expected_funding);

        let last_sum_unitary_long_funding = sum_unitary_long_fundings;
        // funding fee ETH @2.2222 USD/ETH, ETH decimals 18
        let sum_unitary_long_funding = Decimal {
            multiplier: 22222,
            decimals: 22,
        };

        let funding_fee_changed = (sum_unitary_long_funding - last_sum_unitary_long_funding)
            .cal_cost(position_qty, usdc_decimals);
        // 1560000 usdc -> 1.560000 USD
        let expect_funding_fee_changed = 1560000;
        assert_eq!(funding_fee_changed, expect_funding_fee_changed);
    }

    #[test]
    fn test_deserialize_json() {
        let raw_price = r#"{
            "multiplier": "10006555",
            "decimals": 22
        }"#;
        let price: Decimal = near_sdk::serde_json::from_str(raw_price).unwrap();
        assert_eq!(price.multiplier, 10006555);
        assert_eq!(price.decimals, 22);
    }
}
