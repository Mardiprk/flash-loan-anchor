use anchor_lang::prelude::*;

declare_id!("Gf5shq1VZ67fXkfw28r4TE422Uc9JTuJqXng1mxVHwuf");

#[program]
pub mod flash_loan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
