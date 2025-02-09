mod distribution; // distribution module (import file distribtuion.rs)
use anchor_lang::solana_program;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use crate::distribution::*;

declare_id!("11111111111111111111111111111111");  // program ID

#[program]
pub mod tamia_coin {
    use super::*;

    // Function to initialize the token and distribute supply
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mint = &ctx.accounts.mint;

        // Initial distribution of tokens based on the predefined percentages in distribution.rs
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.owner_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Total supply is defined in `distribution.rs`
        // Let's distribute the tokens to different categories.
        token::mint_to(CpiContext::new(cpi_program.clone(), cpi_accounts), OWNER_SUPPLY)?;

        // Additional distributions (for liquidity, P2E, etc.)
        let cpi_accounts_liquidity = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.liquidity_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(CpiContext::new(cpi_program.clone(), cpi_accounts_liquidity), LIQUIDITY_SUPPLY)?;

        let cpi_accounts_p2e = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.p2e_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(CpiContext::new(cpi_program.clone(), cpi_accounts_p2e), P2E_SUPPLY)?;

        let cpi_accounts_marketing = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.marketing_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(CpiContext::new(cpi_program.clone(), cpi_accounts_marketing), MARKETING_SUPPLY)?;

        let cpi_accounts_team = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.team_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(CpiContext::new(cpi_program.clone(), cpi_accounts_team), TEAM_SUPPLY)?;

        let cpi_accounts_burn = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.burn_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(CpiContext::new(cpi_program.clone(), cpi_accounts_burn), BURN_SUPPLY)?;

        // Vesting tokens (if applicable)
        // Handle vesting logic as needed for OWNER_VESTING_PERIOD, etc.

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
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub liquidity_account: Account<'info, TokenAccount>, // Added liquidity account
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub p2e_account: Account<'info, TokenAccount>, // Added P2E account
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub marketing_account: Account<'info, TokenAccount>, // Added marketing account
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub team_account: Account<'info, TokenAccount>, // Added team account
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub burn_account: Account<'info, TokenAccount>, // Added burn account
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
