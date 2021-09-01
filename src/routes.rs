use crate::{
    payment_stream::{PaymentStreamResponse, PaymentStreams},
    reciver_model::ReciverRewardPercentage,
    solana::{get_all_account, get_rent_exemption},
    withdraw_model::WithdrawAmount,
};
use borsh::BorshSerialize;
use rocket::{
    get, post,
    serde::json::{serde_json::json, Json, Value},
};
use solana_sdk::borsh::try_from_slice_unchecked;

#[get("/")]
pub fn index() -> &'static str {
    "This server is here for data serialization and deserialization with borsh crate, And Beacause I hate javascript!!!
It Also act as a helper to get all active streams a public id is related to by parsing data from all program account : )"
}

#[get("/getallstream/<public_key>")]
pub fn get_streams(public_key: &str) -> Json<Value> {
    let rent_exempt = get_rent_exemption();
    let accounts = get_all_account();
    println!("{}", accounts.len());
    let mut reciving: Vec<PaymentStreamResponse> = Vec::new();
    let mut sending: Vec<PaymentStreamResponse> = Vec::new();
    for acc in accounts {
        let program_account = acc.1;
        let deserialized_data: PaymentStreams =
            match try_from_slice_unchecked(&program_account.data) {
                Ok(p) => p,
                Err(_e) => {
                    print!("{:?}", _e);
                    continue;
                }
            };
        if deserialized_data.to.to_string().eq(public_key) {
            reciving.push(PaymentStreamResponse::new(
                deserialized_data,
                &acc.0,
                (program_account.lamports - rent_exempt) as i64,
            ));
        } else if deserialized_data.from.to_string().eq(public_key) {
            sending.push(PaymentStreamResponse::new(
                deserialized_data,
                &acc.0,
                (program_account.lamports - rent_exempt) as i64,
            ))
        }
    }

    Json(json!({"code": 200,"public_key":public_key,"reciving":reciving,"sending":sending,}))
}

// #[post("/streamde", data = "<stream>")]
// pub fn deserialize_stream(stream: Json<StreamU8>) -> Json<Value> {
//     let temp = stream.0.data;
//     let payment_streams: PaymentStreams = match BorshDeserialize::try_from_slice(temp.borrow()) {
//         Ok(p) => p,
//         Err(_e) => {
//             return Json(json!({"code": 400,"error":"cantParse Invalid"}));
//         }
//     };
//     Json(json!({"code": 200,"result":payment_streams}))
// }

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
