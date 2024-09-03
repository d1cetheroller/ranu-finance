use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use state::{config::RanuConfig, pool::VaultPool};

declare_id!("FJou9p7xKpTwGdH6aCjrGFexE9SHDJpuKntKjN7JL1U2");

pub mod state;

#[program]
pub mod ranu_finance {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
        ctx.accounts.ranu_config.fee = fee;
        Ok(())
    }

    pub fn create_pool(ctx: Context<Pool>, max_cap: u64) -> Result<()> {
        ctx.accounts.pool.max_cap = max_cap;
        ctx.accounts.pool.token_mint = ctx.accounts.token_mint.key();
        ctx.accounts.pool.bump = ctx.bumps.pool;

        Ok(())
    }

    // pub fn deposit() -> Result<()> {
    //     Ok(())
    // }
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

    #[account(mut)]
    pub user: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
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

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = pool
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        seeds = [VaultPool::POOL_SEED.as_bytes()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, VaultPool>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = pool,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    /// CHECK:
    #[account(
        mut,
        seeds = [VaultPool::SOL_POOL_SEED.as_bytes(), token_mint.key().as_ref()],
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

    #[account()]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
