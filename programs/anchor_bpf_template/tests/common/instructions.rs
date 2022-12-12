use std::sync::Arc;

use anchor_lang::solana_program::example_mocks::solana_sdk::transaction::Transaction;
use solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};

use super::setup::Env;

pub async fn initialize<'a>(env: Env<'a>, payer: Arc<Keypair>) -> Transaction {
    let instruction = Instruction {
        program_id: anchor_bpf_template::id(),
        accounts: vec![],
        data: vec![],
    };

    Transaction::new_signed_with_payer(
        std::slice::from_ref(&instruction),
        Some(&payer.pubkey()),
        &[payer.as_ref()],
        env.client.get_latest_blockhash().await.unwrap(),
    )
}
