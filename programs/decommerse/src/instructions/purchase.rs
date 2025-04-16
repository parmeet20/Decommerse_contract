use anchor_lang::prelude::*;

use crate::{
    errors::EcommerceError,
    states::{product::Product, program_state::ProgramState, transaction::Transaction},
};

pub fn purchase_product(ctx: Context<PurchaseProduct>, pid: u64, quantity: u64) -> Result<()> {
    require!(quantity > 0, EcommerceError::InvalidQuantity);

    let product = &mut ctx.accounts.product;
    let transaction = &mut ctx.accounts.transaction;

    if pid != product.product_id {
        return Err(EcommerceError::ProductNotFound.into());
    }

    // Check sufficient stock
    require!(product.stock >= quantity, EcommerceError::InsufficientStock);

    // Calculate total price
    let total_price = product.price * quantity;

    let tx_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.buyer.key(),
        &ctx.accounts.seller.key(),
        total_price,
    );

    let result = anchor_lang::solana_program::program::invoke(
        &tx_instruction,
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.seller.to_account_info(),
        ],
    );
    if let Err(e) = result {
        msg!("Product purchase failed: {:?}", e);
        return Err(e.into());
    }

    // Update stock
    product.stock -= quantity;

    transaction.pid = pid;
    transaction.seller = ctx.accounts.seller.key();
    transaction.buyer = ctx.accounts.buyer.key();
    transaction.amount = total_price;
    transaction.quantity = quantity;
    transaction.timestamp = Clock::get()?.unix_timestamp as u64;

    Ok(())
}
#[derive(Accounts)]
pub struct PurchaseProduct<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"product", product.seller.as_ref(),program_state.product_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        init,
        space = 8 + Transaction::INIT_SPACE,
        payer = buyer,
        seeds = [b"transaction", buyer.key().as_ref(), product.seller.as_ref(), (program_state.transaction_count + 1).to_le_bytes().as_ref()],
        bump,
    )]
    pub transaction: Account<'info, Transaction>,
    /// CHECK:
    #[account(mut, address = product.seller)]
    pub seller: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
