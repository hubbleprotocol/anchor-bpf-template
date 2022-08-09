use anchor_lang::prelude::*;
mod handlers;
use crate::handlers::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_bpf_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handlers::handler_initialize::process(ctx)
    }
}
