use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("EHuoc9s4Pyvq5jGenXLbcmRspBEN6hUkD4FuYWPHX7w5");

#[program]
pub mod anchor_vault_program_practice {
    // use std::task::Context;

    use super::*;

    pub fn make(ctx:Context<Make>, seed:u64, deposit:u64, receive:u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
        

}

#[derive(Accounts)]
pub struct Initialize {}
