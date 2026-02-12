use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("3eoLkGPXZBvPsziREYSL869Vo71DDYBYCFeDuR5mdRNr");

#[program]
pub mod solchit_fund {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        contribution_amount: u64,
        max_members: u8,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.creator = ctx.accounts.creator.key();
        pool.contribution_amount = contribution_amount;
        pool.max_members = max_members;
        pool.current_members = 0;
        pool.current_round = 0;
        pool.usdc_mint = ctx.accounts.usdc_mint.key();
        Ok(())
    }

    pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        require!(
            pool.current_members < pool.max_members,
            CustomError::PoolFull
        );

        let member = &mut ctx.accounts.member;
        member.wallet = ctx.accounts.user.key();
        member.has_deposited = false;
        member.has_won = false;

        pool.current_members += 1;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let pool = &ctx.accounts.pool;
        let member = &mut ctx.accounts.member;

        require!(!member.has_deposited, CustomError::AlreadyDeposited);

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, pool.contribution_amount)?;

        member.has_deposited = true;

        Ok(())
    }
}

#[account]
pub struct Pool {
    pub creator: Pubkey,
    pub contribution_amount: u64,
    pub max_members: u8,
    pub current_members: u8,
    pub current_round: u8,
    pub usdc_mint: Pubkey,
}

#[account]
pub struct Member {
    pub wallet: Pubkey,
    pub has_deposited: bool,
    pub has_won: bool,
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 8 + 1 + 1 + 1 + 32)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinPool<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer = user, space = 8 + 32 + 1 + 1)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum CustomError {
    #[msg("Pool is full")]
    PoolFull,
    #[msg("Already deposited")]
    AlreadyDeposited,
}
