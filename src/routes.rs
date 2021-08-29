use std::borrow::Borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use rocket::{
    get, post,
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    payment_stream::PaymentStreams, reciver_model::ReciverRewardPercentage,
    solana::get_all_account, stream_u8::StreamU8, withdraw_model::WithdrawAmount,
};

#[get("/")]
pub fn index() -> &'static str {
    "This server is here for data serialization and deserialization with borsh crate, And Beacause I hate javascript!!!"
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
                    continue;
                }
            };

        if deserialized_data.to.to_string().eq(public_key) {
            reciving.push(deserialized_data);
        } else if deserialized_data.from.to_string().eq(public_key) {
            sending.push(deserialized_data)
        }
    }

    Json(json!({"code": 200,"public_key":public_key,"reciving":reciving,"sending":sending,}))
}

#[post("/streamde", data = "<stream>")]
pub fn deserialize_stream(stream: Json<StreamU8>) -> Json<Value> {
    let temp = stream.0.data;
    let payment_streams: PaymentStreams = match BorshDeserialize::try_from_slice(temp.borrow()) {
        Ok(p) => p,
        Err(_e) => {
            return Json(json!({"code": 400,"error":"cantParse Invalid"}));
        }
    };
    Json(json!({"code": 200,"result":payment_streams}))
}

#[post("/stream", data = "<stream>")]
pub fn serialize_stream(stream: Json<PaymentStreams>) -> Json<Value> {
    let temp = stream.0;
    let res = temp.try_to_vec().unwrap();
    Json(json!({"code": 200,"result":res}))
}

#[post("/reward", data = "<reward>")]
pub fn reciver_reward_serialize(reward: Json<ReciverRewardPercentage>) -> Json<Value> {
    let temp = reward.0;
    let res = temp.try_to_vec().unwrap();
    Json(json!({"code": 200,"result":res}))
}

#[post("/withdraw", data = "<withdraw>")]
pub fn withdraw_serialize(withdraw: Json<WithdrawAmount>) -> Json<Value> {
    let temp = withdraw.0;
    let res = temp.try_to_vec().unwrap();
    Json(json!({"code": 200,"result":res}))
}
