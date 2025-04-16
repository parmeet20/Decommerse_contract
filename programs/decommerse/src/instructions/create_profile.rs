use anchor_lang::prelude::*;

use crate::states::user_profile::UserProfile;
use crate::errors::EcommerceError::*;

pub fn create_user_profile(ctx: Context<CreateUserProfile>, username: String) -> Result<()> {
    if username.len()>50{
        return Err(UsernameTooLong.into());
    }
    let user_profile = &mut ctx.accounts.user_profile;

    if user_profile.is_initialized{
        return Err(ProfileAlreadyInitialized.into());
    }

    user_profile.username = username;
    user_profile.user = ctx.accounts.user.key();
    user_profile.is_initialized = true;
    Ok(())
}
#[derive(Accounts)]
pub struct CreateUserProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserProfile::INIT_SPACE,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}