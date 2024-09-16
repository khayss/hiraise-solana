use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    pub user: Pubkey,
    #[max_len(10)]
    pub campaigns: Vec<Pubkey>,
}
