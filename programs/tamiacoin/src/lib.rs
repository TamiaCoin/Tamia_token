use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

declare_id!("");  // program ID

#[program]
pub mod tamia_coin {
    use super::*;

    // Function to initialize the token and distribute supply
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.decimals = 9;
        mint.supply = 100_000_000_000_000; // 100 Trillions

        let cpi_accounts = MinTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.owner_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::mint_to(CpiContext::new(cpi_program, cpi_accounts), mint.supply)?;

        Ok(())  
    }

    // Function to "mint" (create) tokens and send them to a user
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::mint_to(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        Ok(())
    }
}

// Accounts needed to initiate the token
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, mint::decimals = 9, mint::authority = user)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub owner_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

// Accounts needed to "mint" (create) tokens
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
