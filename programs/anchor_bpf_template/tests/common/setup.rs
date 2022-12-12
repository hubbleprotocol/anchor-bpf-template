use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use solana_program_test::{BanksClient, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::{Account, AccountSharedData},
    signature::Keypair,
    signer::Signer,
    system_program,
};

pub struct Env<'a> {
    pub program_id: &'a Pubkey,
    pub client: &'a mut BanksClient,
}

pub type KP = Arc<Keypair>;
pub fn kp() -> KP {
    Arc::new(Keypair::new())
}

pub fn fund_kp(test: &mut ProgramTest, min_balance_lamports: u64, user: Arc<Keypair>) -> KP {
    test.add_account(
        user.pubkey(),
        Account {
            lamports: min_balance_lamports,
            ..Account::default()
        },
    );
    user
}

pub fn funded_kp(test: &mut ProgramTest, min_balance_lamports: u64) -> KP {
    fund_kp(test, min_balance_lamports, kp())
}

pub fn funded_new_kp(test: &mut ProgramTestContext, min_balance_lamports: u64) -> KP {
    fund_new_kp(test, min_balance_lamports, kp())
}

pub fn fund_new_kp(
    test: &mut ProgramTestContext,
    min_balance_lamports: u64,
    user: Arc<Keypair>,
) -> KP {
    let account = AccountSharedData::new(min_balance_lamports, 0, &system_program::ID);
    test.set_account(&user.pubkey(), &account);
    user
}

pub fn funded_new_kps<const NUM: usize>(
    test: &mut ProgramTestContext,
    min_balance_lamports: u64,
) -> [KP; NUM] {
    (0..NUM)
        .map(|_| funded_new_kp(test, min_balance_lamports))
        .collect::<Vec<KP>>()
        .try_into()
        .unwrap()
}

pub fn funded_kps<const NUM: usize>(
    test: &mut ProgramTest,
    min_balance_lamports: u64,
) -> [KP; NUM] {
    (0..NUM)
        .map(|_| funded_kp(test, min_balance_lamports))
        .collect::<Vec<KP>>()
        .try_into()
        .unwrap()
}
