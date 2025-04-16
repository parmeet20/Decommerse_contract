use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    #[max_len(50)]
    pub username: String,
    pub user: Pubkey,
    pub products_count:u64,
    pub is_initialized:bool,
}