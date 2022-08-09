use anchor_lang::prelude::*;
use anchor_lang::Accounts;

pub fn process(_ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize {}
