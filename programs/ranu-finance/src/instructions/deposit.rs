use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{config::RanuConfig, pool::VaultPool};

pub fn deposit(ctx: Context<Deposit>, sol_amount: u64) -> Result<()> {
    let pool_info = ctx.accounts.pool.to_account_info();
    let pool = &mut ctx.accounts.pool;

    pool.deposit(
        sol_amount,
        &ctx.accounts.user,
        &pool_info,
        &mut ctx.accounts.pool_sol_vault,
        &ctx.accounts.token_mint,
        &ctx.accounts.user_token_account,
        &ctx.accounts.system_program,
        &ctx.accounts.token_program,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
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
        seeds = [VaultPool::SOL_VAULT_SEED.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    pub pool_sol_vault: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
