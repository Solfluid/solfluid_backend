use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

pub fn get_all_account() -> Vec<(Pubkey, Account)> {
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);
    let pubkey = Pubkey::new("7ZRbUhE2WU57qXaGWZ9hJ6rqf1PvuGJT2LEzRwPvho1x".as_bytes());
    let res = client.get_program_accounts(&pubkey).unwrap();
    res
}
