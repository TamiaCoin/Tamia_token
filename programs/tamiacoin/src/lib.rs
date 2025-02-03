use anchor_lang::prelude::*;

declare_id!("vFfDiHaa9toLdP2WUHsjsv6PxM5Wi1sem5gDqgAnr4e");

#[program]
pub mod tamiacoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
