use anchor_lang::prelude::*;

use crate::states::program_state::ProgramState;
use crate::errors::EcommerceError::*;

pub fn init(ctx: Context<Init>) -> Result<()> {
    let p = &mut ctx.accounts.p_count;

    if p.initialized{
        return Err(ProgramAlreadyInitialzed.into());
    }

    p.owner = ctx.accounts.user.key();
    p.product_count = 0;
    p.transaction_count = 0;
    p.initialized = true;
    Ok(())
}
#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + ProgramState::INIT_SPACE,
        seeds = [b"program_state"],
        bump,

    )]
    pub p_count:Account<'info,ProgramState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}