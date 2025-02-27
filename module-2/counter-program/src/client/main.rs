use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_instruction,
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

// Counter instruction enum (same as in program)
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    Initialize,
    Increment { amount: u64 },
    Decrement { amount: u64 },
    Reset,
}

fn main() {
    // Connect to the cluster
    let rpc_url = "http://localhost:8899".to_string();
    let client = RpcClient::new(rpc_url);

    // Create a new keypair for the counter account
    let payer = Keypair::new();
    let counter_keypair = Keypair::new();
    
    // Hard-code the program ID (replace with your deployed program ID)
    let program_id = Pubkey::from_str("CounterProgramID111111111111111111111111111111").unwrap();

    // Airdrop some SOL to the payer
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let signature = client
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .unwrap();
    client.confirm_transaction_with_spinner(
        &signature,
        &recent_blockhash,
        solana_client::rpc_config::RpcConfirmTransactionConfig::default(),
    ).unwrap();

    println!("Creating counter account...");

    // Calculate the size of the counter account
    let account_span = std::mem::size_of::<u64>() + std::mem::size_of::<Pubkey>();
    
    // Get minimum rent
    let rent = client
        .get_minimum_balance_for_rent_exemption(account_span)
        .unwrap();

    // Create account transaction
    let mut transaction = Transaction::new_with_payer(
        &[system_instruction::create_account(
            &payer.pubkey(),
            &counter_keypair.pubkey(),
            rent,
            account_span as u64,
            &program_id,
        )],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &counter_keypair], recent_blockhash);

    // Submit transaction
    client.send_and_confirm_transaction(&transaction).unwrap();
    
    println!("Counter account created: {}", counter_keypair.pubkey());

    // Initialize counter
    println!("Initializing counter...");
    let instruction_data = CounterInstruction::Initialize.try_to_vec().unwrap();
    
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(counter_keypair.pubkey(), false),
                AccountMeta::new(payer.pubkey(), true),
            ],
        )],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[&payer], recent_blockhash);

    client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Counter initialized!");
    
    // Increment counter
    println!("Incrementing counter by 5...");
    let instruction_data = CounterInstruction::Increment { amount: 5 }.try_to_vec().unwrap();
    
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(counter_keypair.pubkey(), false),
                AccountMeta::new(payer.pubkey(), true),
            ],
        )],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[&payer], recent_blockhash);

    client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Counter incremented by 5!");
    
    println!("Client interaction complete!");
}
