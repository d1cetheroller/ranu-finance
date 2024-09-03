use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[account]
pub struct VaultPool {
    pub max_cap: u64,
    pub total_deposits: u64,
    pub token_mint: Pubkey,
    pub bump: u8,
}

impl VaultPool {
    pub const POOL_SEED: &'static str = "VaultPool";

    pub const SOL_POOL_SEED: &'static str = "SolVaultPool";

    pub const ACCOUNT_SIZE: usize = 8 + 8 + 8 + 32 + 1;

    pub fn new(max_cap: u64, bump: u8, token_mint: Pubkey) -> Self {
        Self {
            max_cap,
            bump,
            total_deposits: 0,
            token_mint,
        }
    }

    pub fn deposit<'info>(
        &mut self,
        amount: u64,
        from: &Signer<'info>,
        pool_vault: &mut AccountInfo<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        if self.total_deposits.checked_add(amount).unwrap() > self.max_cap {
            return Err(ErrorCode::DepositOverflow.into());
        }

        self.total_deposits = self.total_deposits.checked_add(amount).unwrap();

        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: pool_vault.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Deposit amount exceeds the maximum cap")]
    DepositExceedsMaxCap,
    #[msg("Deposit overflow")]
    DepositOverflow,
}
