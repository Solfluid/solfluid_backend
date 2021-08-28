use std::borrow::Borrow;

use borsh::BorshDeserialize;
use rocket::{
    get,
    serde::json::{serde_json::json, Json, Value},
};

use crate::{payment_stream::PaymentStreams, solana::get_all_account};

#[get("/")]
pub fn index() -> Json<Value> {
    Json(json!({"code": 200,
    "server": "hello nishant",}))
}

#[get("/getallstream/<public_key>")]
pub fn get_streams(public_key: &str) -> Json<Value> {
    let accounts = get_all_account();
    let mut reciving: Vec<PaymentStreams> = Vec::new();
    let mut sending: Vec<PaymentStreams> = Vec::new();
    for acc in accounts {
        let program_account = acc.1;
        let deserialized_data: PaymentStreams =
            match BorshDeserialize::try_from_slice(program_account.data.borrow()) {
                Ok(p) => p,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

        if deserialized_data.to.to_string().eq(public_key) {
            reciving.push(deserialized_data);
        } else if deserialized_data.from.to_string().eq(public_key) {
            sending.push(deserialized_data)
        }
    }

    Json(json!({"code": 200,"public_key":public_key,"reciving":reciving,"sending":sending}))
}
