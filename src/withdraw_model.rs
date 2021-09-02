use borsh::{BorshDeserialize, BorshSchema,BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshSchema,BorshDeserialize, Debug, Clone, Deserialize, Serialize)]
pub struct WithdrawAmount {
    amount: i64,
}
