use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Gf5shq1VZ67fXkfw28r4TE422Uc9JTuJqXng1mxVHwuf");

#[program]
pub mod flash_loan {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, fee_rate: u64) -> Result<()> {
        Ok(())
    }

    pub fn deposit_liquidity(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn flash_loan(ctx: Context<FlashLoan>, amount: u64, callback_data: Vec<u8>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Pool::LEN,
        seeds = [b"pool", token_mint.key().as_ref],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = pool,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DepositLiquidity<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", pool.token_mint.as_ref()],
        bump
    )]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = user_token_account.mint == pool.token_mint
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FlashLoan<'info> {
    #[account(
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    pub borrower: Signer<'info>,

    #[account(
        mut,
        constraint = borrower_token_account.mint == pool.token_mint
    )]
    pub borrower_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", pool.token_mint.as_ref()],
        bump
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = authority_token_account.mint == pool.token_mint
    )]
    pub authority_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", pool.token_mint.as_ref()],
        bump
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub fee_rate: u64,
    pub total_liquidity: u64,
    pub bump: u8,
}

impl Pool {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 1;
}

#[error_code]
pub enum FlashLoanError {
    #[msg("Insufficient liquidity in the pool")]
    InsufficientLiquidity,

    #[msg("Flash loan was not repaid")]
    LoanNotRepaid,

    #[msg("Unauthorized access")]
    Unauthorized,
}

