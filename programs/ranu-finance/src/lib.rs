use anchor_lang::prelude::*;

declare_id!("FJou9p7xKpTwGdH6aCjrGFexE9SHDJpuKntKjN7JL1U2");

pub mod instructions;
pub mod state;

use crate::instructions::*;

#[program]
pub mod ranu_finance {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
        instructions::initialize(ctx, fee)
    }

    pub fn create_pool(ctx: Context<Pool>, max_cap: u64) -> Result<()> {
        instructions::create_pool(ctx, max_cap)
    }

    pub fn deposit(ctx: Context<Deposit>, sol_amount: u64) -> Result<()> {
        instructions::deposit(ctx, sol_amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, sol_amount: u64) -> Result<()> {
        instructions::withdraw(ctx, sol_amount)
    }
}
