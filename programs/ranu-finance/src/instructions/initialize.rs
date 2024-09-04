use anchor_lang::prelude::*;

use crate::state::config::RanuConfig;

pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
    ctx.accounts.ranu_config.fee = fee;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = RanuConfig::ACCOUNT_SIZE,
        seeds = [RanuConfig::SEED.as_bytes()],
        payer = user,
        bump
    )]
    pub ranu_config: Box<Account<'info, RanuConfig>>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
}
