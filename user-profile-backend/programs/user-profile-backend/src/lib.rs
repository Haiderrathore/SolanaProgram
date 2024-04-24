use anchor_lang::prelude::*;

declare_id!("EEoe57rfT1qw8SrCy1M1AmVQUJjXndFRY4q5KYWwGdj");

#[program]
pub mod user_profile_backend {
    use super::*;

    pub fn init_userprofile(ctx: Context<InitUserProfile>, name: String, username: String, email: String) -> Result<()> 
    {
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.name = name;
        user_profile.username = username;
        user_profile.email = email;
         
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, username: String, email: String)]
pub struct InitUserProfile<'info>
{
    #[account(init, payer = user, seeds=[b"User", user.key().as_ref()], bump, space = 8 + 4 + name.len() + 4 + username.len() + 4 + email.len())]
    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct UserProfile 
{
    pub name: String,
    pub username: String,
    pub email: String
}
