use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::{config::RanuConfig, pool::VaultPool};

pub fn withdraw(ctx: Context<Withdraw>, sol_amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.withdraw(
        sol_amount,
        &mut ctx.accounts.pool_sol_vault,
        &ctx.accounts.user.to_account_info(),
        &ctx.accounts.system_program,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [RanuConfig::SEED.as_bytes()],
        bump
    )]
    pub ranu_config: Box<Account<'info, RanuConfig>>,

    #[account(
        seeds = [VaultPool::POOL_SEED.as_bytes(), token_mint.key().as_ref()],
        bump = pool.pool_bump
    )]
    pub pool: Account<'info, VaultPool>,

    /// CHECK:
    #[account(
        mut,
        seeds = [VaultPool::SOL_POOL_SEED.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    pub pool_sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
