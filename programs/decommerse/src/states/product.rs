use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Product {
    pub product_id: u64,
    #[max_len(50)] // 4 + 100
    pub name: String, // 4 + 200
    #[max_len(642)] // 4 + 100
    pub description: String, // 4 + 200
    #[max_len(240)] // 4 + 100
    pub metadata: String, // 4 + 200
    pub price: u64,     // 8
    pub stock: u64,     // 4
    pub seller: Pubkey, // 32
    pub timestamp:u64,
}