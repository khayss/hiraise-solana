use anchor_lang::prelude::*;

use crate::states::campaign::campaign::Campaign;



#[derive(Accounts)]
pub struct Donate<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub donor: Signer<'info>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
}
