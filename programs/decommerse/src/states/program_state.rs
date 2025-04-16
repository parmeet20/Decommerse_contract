use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub owner:Pubkey,
    pub transaction_count:u64,
    pub product_count:u64,
    pub initialized:bool,
}