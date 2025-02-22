use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let program_id = Pubkey::from_str("8AFi1JPrEBocFLMDbXaDN8fp4DqiC2uFf5nFQsakVGxi").unwrap();

    let rpc_url = String::from("http://10.190.41.37:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();

    let airdrop_amount = 1_000_000_000;
    let signature = client
        .request_airdrop(&payer.pubkey(), airdrop_amount)
        .expect("Airdrop request failed");

    loop {
        if client.confirm_transaction(&signature).unwrap() {
            break;
        }
    }


    let instruction = Instruction::new_with_borsh(
        program_id,
        &(),    
        vec![], 
    );

   
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[&payer], recent_blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(tx_signature) => println!("Transaction Signature: {}", tx_signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}
