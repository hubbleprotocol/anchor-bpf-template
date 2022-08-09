#[macro_export]
macro_rules! readable {
    ($res:expr) => {
        AccountMeta::new_readonly($res, false)
    };
}

#[macro_export]
macro_rules! signer {
    ($res:expr) => {
        AccountMeta::new_readonly($res, true)
    };
}

#[macro_export]
macro_rules! writable {
    ($res:expr) => {
        AccountMeta::new($res, false)
    };
}

#[macro_export]
macro_rules! writable_signer {
    ($res:expr) => {
        AccountMeta::new($res, true)
    };
}

#[macro_export]
macro_rules! send_transaction {
    ($ctx: expr, $transaction: expr) => {
        $ctx.context
            .banks_client
            .process_transaction_with_commitment(
                $transaction,
                solana_sdk::commitment_config::CommitmentLevel::Processed,
            )
            .await
    };
}
