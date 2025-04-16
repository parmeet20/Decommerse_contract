use anchor_lang::prelude::*;

use crate::states::{product::Product, program_state::ProgramState};

pub fn update_product_stock(ctx: Context<UpdateProductStock>, new_stock: u64) -> Result<()> {
    let product = &mut ctx.accounts.product;
    product.stock = new_stock;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateProductStock<'info> {
    #[account(
        mut,
        seeds = [b"product", seller.key().as_ref(), program_state.product_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program:Program<'info,System>,
}
