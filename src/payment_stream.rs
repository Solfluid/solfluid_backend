use borsh::{BorshDeserialize, BorshSerialize};
use rocket::serde::ser::{Serialize, SerializeStruct};
use serde::Deserialize;
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey};

#[derive(Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct PaymentStreams {
    pub end_time: UnixTimestamp,
    pub start_time: UnixTimestamp,
    pub amount_second: i64,
    pub to: Pubkey,
    pub from: Pubkey,
    pub lamports_withdrawn: i64,
    pub is_active: bool,
}

impl Serialize for PaymentStreams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("PaymentStream", 7)?;
        s.serialize_field("end_time", &self.end_time)?;
        s.serialize_field("start_time", &self.start_time)?;
        s.serialize_field("amount_second", &self.amount_second)?;
        s.serialize_field("lamports_withdrawn", &self.lamports_withdrawn)?;
        s.serialize_field("is_active", &self.is_active)?;
        let to_string = &self.to.to_string();
        let from_string = &self.from.to_string();
        s.serialize_field("to", to_string)?;
        s.serialize_field("from", from_string)?;
        s.end()
    }
}
