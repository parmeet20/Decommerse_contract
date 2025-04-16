use anchor_lang::prelude::*;

use crate::errors::EcommerceError::*;
use crate::states::product::Product;
use crate::states::program_state::ProgramState;

pub fn create_product(
    ctx: Context<CreateProduct>,
    name: String,
    description: String,
    metadata: String,
    price: u64,
    initial_stock: u64,
) -> Result<()> {

    if name.len() > 200 {
        return Err(ProductNameTooLong.into());
    }

    if price <= 0 {
        return Err(InvalidPrice.into());
    }

    if initial_stock <= 0 {
        return Err(InvalidStock.into());
    }

    let product = &mut ctx.accounts.product;
    let state = &mut ctx.accounts.program_state;

    state.product_count += 1;

    product.product_id = state.product_count;
    product.name = name;
    product.timestamp = Clock::get()?.unix_timestamp as u64;
    product.description = description;
    product.metadata = metadata;
    product.price = price;
    product.stock = initial_stock;
    product.seller = ctx.accounts.seller.key();

    Ok(())
}

#[derive(Accounts)]
pub struct CreateProduct<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + Product::INIT_SPACE,
        seeds = [b"product", seller.key().as_ref(),(program_state.product_count+1).to_le_bytes().as_ref()],
        bump
    )]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}
