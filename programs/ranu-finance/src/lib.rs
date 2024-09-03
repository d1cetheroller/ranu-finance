use anchor_lang::prelude::*;

declare_id!("FJou9p7xKpTwGdH6aCjrGFexE9SHDJpuKntKjN7JL1U2");

#[program]
pub mod ranu_finance {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
