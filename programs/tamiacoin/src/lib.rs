mod distribution; // distribution module (import file distribtuion.rs)


use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use crate::distribution::*;


declare_id!("6LuDa36enHe3rgJbTYVQiCpnBDtRakDPkz46Yy7AhM2y"); // program ID

#[program]
pub mod tamia_coin {
    use super::*;

    // Function to initialize the token and distribute supply
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mint = &ctx.accounts.mint.to_account_info();
        let authority = &ctx.accounts.authority.to_account_info();
        let token_program = &ctx.accounts.token_program.to_account_info();
        
        // Initial mint only for owner (others will be added separately)
        mint_to_account(mint, &ctx.accounts.owner_account.to_account_info(), authority, token_program, OWNER_SUPPLY)?;
        
        Ok(())
    }

    // New feature: Add only one distribution account at a time
    pub fn add_single_account(ctx: Context<AddSingleAccount>, amount: u64) -> Result<()> {
        let mint = &ctx.accounts.mint.to_account_info();
        let authority = &ctx.accounts.authority.to_account_info();
        let token_program = &ctx.accounts.token_program.to_account_info();

        mint_to_account(mint, &ctx.accounts.token_account.to_account_info(), authority, token_program, amount)?;

        Ok(())
    }

    // Function to mount tokens to a specific user
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

// Account structure for initialization (creation of mint and owner account)
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

// Account structure to add other distribution accounts
// New structure to initialize ONLY one distribution account at a time
#[derive(Accounts)]
pub struct AddSingleAccount<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

/// Account structure for mounting tokens
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// Utility function to perform a secure mint
fn mint_to_account<'info>(
    mint: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64
) -> Result<()> {
    let cpi_accounts = MintTo {
        mint: mint.clone(),
        to: to.clone(),
        authority: authority.clone(),
    };
    token::mint_to(CpiContext::new(token_program.clone(), cpi_accounts), amount)?;
    Ok(())
}
