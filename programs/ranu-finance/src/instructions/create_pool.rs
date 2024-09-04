use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
};

use crate::state::{config::RanuConfig, pool::VaultPool};

pub fn create_pool(ctx: Context<Pool>, max_cap: u64) -> Result<()> {
    ctx.accounts.pool.max_cap = max_cap;
    ctx.accounts.pool.token_mint = ctx.accounts.token_mint.key();
    ctx.accounts.pool.pool_bump = ctx.bumps.pool;
    ctx.accounts.pool.sol_vault_bump = ctx.bumps.pool_sol_vault;

    Ok(())
}

#[derive(Accounts)]
pub struct Pool<'info> {
    #[account(
        seeds = [RanuConfig::SEED.as_bytes()],
        bump
    )]
    pub ranu_config: Box<Account<'info, RanuConfig>>,

    #[account(
        init,
        space = VaultPool::ACCOUNT_SIZE,
        seeds = [VaultPool::POOL_SEED.as_bytes(), token_mint.key().as_ref()],
        payer = user,
        bump
    )]
    pub pool: Box<Account<'info, VaultPool>>,

    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = pool,
    )]
    pub token_mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
        mut,
        seeds = [VaultPool::SOL_POOL_SEED.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    pub pool_sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
