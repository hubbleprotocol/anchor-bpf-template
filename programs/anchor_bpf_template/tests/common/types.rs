use anchor_lang::prelude::{thiserror, Rent};
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use std::sync::Arc;
use thiserror::Error;

pub struct TestContext {
    pub initial_market_owner: Arc<Keypair>,
    pub context: ProgramTestContext,
    pub rent: Rent,
}

#[derive(PartialEq, Eq, Error, Debug)]
pub enum TestError {
    #[error("Insufficient collateral to cover debt")]
    CannotDeserialize,
    #[error("Wrong discriminator")]
    BadDiscriminator,
    #[error("Account not found")]
    AccountNotFound,
    #[error("Unknown Error")]
    UnknownError,
}
