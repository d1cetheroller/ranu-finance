use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[account]
pub struct VaultPool {
    pub max_cap: u64,
    pub total_deposits: u64,
    pub token_mint: Pubkey,
    pub pool_bump: u8,
    pub sol_vault_bump: u8,
    pub is_closed: bool,
}

impl VaultPool {
    pub const POOL_SEED: &'static str = "VaultPool";

    pub const SOL_VAULT_SEED: &'static str = "SolVaultPool";

    pub const ACCOUNT_SIZE: usize = 8 + 8 + 8 + 32 + 1 + 1 + 1;

    pub fn new(max_cap: u64, pool_bump: u8, sol_vault_bump: u8, token_mint: Pubkey) -> Self {
        Self {
            max_cap,
            pool_bump,
            sol_vault_bump,
            total_deposits: 0,
            token_mint,
            is_closed: false,
        }
    }

    pub fn deposit<'info>(
        &mut self,
        amount: u64,
        from: &Signer<'info>,
        pool: &AccountInfo<'info>,
        sol_vault: &mut AccountInfo<'info>,
        token_mint: &Account<'info, Mint>,
        user_token_account: &Account<'info, TokenAccount>,
        system_program: &Program<'info, System>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        if self.is_closed == true {
            return Err(ErrorCode::DepositClosed.into());
        }

        if self.total_deposits.checked_add(amount).unwrap() > self.max_cap {
            return Err(ErrorCode::DepositExceedsMaxCap.into());
        }

        self.total_deposits = self.total_deposits.checked_add(amount).unwrap();

        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: sol_vault.to_account_info(),
                },
            ),
            amount,
        )?;

        let seeds = &[
            VaultPool::POOL_SEED.as_bytes(),
            &self.token_mint.to_bytes(),
            &[self.pool_bump],
        ];
        let signer = &[&seeds[..]];

        token::mint_to(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                token::MintTo {
                    mint: token_mint.to_account_info(),
                    to: user_token_account.to_account_info(),
                    authority: pool.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw<'info>(
        &mut self,
        amount: u64,
        sol_vault: &mut AccountInfo<'info>,
        to: &AccountInfo<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        let seeds = &[
            VaultPool::SOL_VAULT_SEED.as_bytes(),
            &self.token_mint.to_bytes(),
            &[self.sol_vault_bump],
        ];
        let signer = &[&seeds[..]];

        system_program::transfer(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: sol_vault.to_account_info(),
                    to: to.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        self.is_closed = true;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Deposit amount exceeds the maximum cap")]
    DepositExceedsMaxCap,
    #[msg("Deposit closed")]
    DepositClosed,
}
