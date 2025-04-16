use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

use crate::instructions::*;
declare_id!("Fyv3yoqLqFmMbhyFy8XcSrx1n6EzXecGQG58g5WKxFz2");

#[program]
pub mod decommerse {

    use super::*;
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn create_user_profile(ctx: Context<CreateUserProfile>, username: String) -> Result<()> {
        instructions::create_user_profile(ctx, username)
    }

    pub fn create_product(
        ctx: Context<CreateProduct>,
        name: String,
        description: String,
        metadata: String,
        price: u64,
        initial_stock: u64,
    ) -> Result<()> {
        instructions::create_product(ctx, name, description, metadata, price, initial_stock)
    }

    pub fn update_product_stock(ctx: Context<UpdateProductStock>, new_stock: u64) -> Result<()> {
        instructions::update_product_stock(ctx, new_stock)
    }

    pub fn purchase_product(ctx: Context<PurchaseProduct>, pid: u64, quantity: u64) -> Result<()> {
        instructions::purchase_product(ctx, pid, quantity)
    }
}
