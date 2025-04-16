use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub pid:u64,
    pub buyer:Pubkey,
    pub seller:Pubkey,
    pub timestamp:u64,
    pub amount:u64,
    pub quantity:u64,
}