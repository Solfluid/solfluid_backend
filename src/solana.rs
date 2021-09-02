use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

pub fn get_all_account() -> Vec<(Pubkey, Account)> {
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);

    let pubkey = Pubkey::from_str("GZLs51zidwmTGiftGJt1in3kWaicd4Ctvp9W95hZxgiZ").unwrap();
    let res = client.get_program_accounts(&pubkey).unwrap();
    res
}

pub fn get_rent_exemption() -> u64 {
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);
    client.get_minimum_balance_for_rent_exemption(97).unwrap()
}


pub fn stake(){
    
}