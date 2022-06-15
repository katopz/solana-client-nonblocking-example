use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub counter: u32,
}

fn main() {}

pub async fn get_greeting_account() -> GreetingAccount {
    // Create a new RPC client on devnet
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::processed(),
    );

    // Target account
    let account_pubkey = Pubkey::from_str("G98GXE4HjwSgUHShMTJhb3GDDmD9oKV95vTmL2bFbFYv").unwrap();

    // Deserialize the account
    let borsh_serialized = rpc_client.get_account_data(&account_pubkey).await.unwrap();
    let greeting_account = GreetingAccount::try_from_slice(&borsh_serialized).unwrap();

    // Return
    greeting_account
}

#[cfg(test)]
mod tests {
    use crate::get_greeting_account;

    #[tokio::test]
    async fn test_greeting_account() {
        let greeting_account = get_greeting_account().await;
        println!("{:#?}", greeting_account);
        assert_eq!(greeting_account.counter, 1);
    }
}
