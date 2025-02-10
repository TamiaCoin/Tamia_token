mod distribution; // distribution module (import file distribtuion.rs)


use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use crate::distribution::*;


declare_id!("11111111111111111111111111111111"); // program ID

#[program]
pub mod tamia_coin {
    use super::*;

    // Function to initialize the token and distribute supply
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mint = &ctx.accounts.mint.to_account_info();
        let authority = &ctx.accounts.authority.to_account_info();
        let token_program = &ctx.accounts.token_program.to_account_info();
        
        // Mint initial uniquement pour le propriétaire (les autres seront ajoutés séparément)
        mint_to_account(mint, &ctx.accounts.owner_account.to_account_info(), authority, token_program, OWNER_SUPPLY)?;
        
        Ok(())
    }

    // Function to add other distribution accounts
    pub fn add_distribution_accounts(ctx: Context<AddDistributionAccounts>) -> Result<()> {
        let mint = &ctx.accounts.mint.to_account_info();
        let authority = &ctx.accounts.authority.to_account_info();
        let token_program = &ctx.accounts.token_program.to_account_info();

        // Mint to the various distribution accounts
        mint_to_account(mint, &ctx.accounts.liquidity_account.to_account_info(), authority, token_program, LIQUIDITY_SUPPLY)?;
        mint_to_account(mint, &ctx.accounts.p2e_account.to_account_info(), authority, token_program, P2E_SUPPLY)?;
        mint_to_account(mint, &ctx.accounts.marketing_account.to_account_info(), authority, token_program, MARKETING_SUPPLY)?;
        mint_to_account(mint, &ctx.accounts.team_account.to_account_info(), authority, token_program, TEAM_SUPPLY)?;
        mint_to_account(mint, &ctx.accounts.burn_account.to_account_info(), authority, token_program, BURN_SUPPLY)?;
        
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
#[derive(Accounts)]
pub struct AddDistributionAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub liquidity_account: Account<'info, TokenAccount>,
    
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub p2e_account: Account<'info, TokenAccount>,
    
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub marketing_account: Account<'info, TokenAccount>,

    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub team_account: Account<'info, TokenAccount>,

    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub burn_account: Account<'info, TokenAccount>,

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
