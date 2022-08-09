mod common;
use common::{fixtures::setup_empty_market_with_dependencies, runner::token, setup::kp};
use solana_program_test::tokio;

#[tokio::test]
async fn test_basic() {
    let mut ctx = setup_empty_market_with_dependencies(&[]).await;
    let _owner = &ctx.initial_market_owner.clone();

    let token_a_mint = kp();
    token::create_mint(&mut ctx, &token_a_mint).await;

    let token_b_mint = kp();
    token::create_mint(&mut ctx, &token_b_mint).await;
}
