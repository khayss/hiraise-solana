use anchor_lang::prelude::*;

use crate::states::{
    campaign::campaign::Campaign, campaign_cordinator::campaign_coordinator::CampaignCoordinator,
};

#[derive(Accounts)]
// #[instruction(campaign_id: u32)]
pub struct EndCampaign<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    /// CHECK: Anyone can end campaign. creator only needs to be writable to receive funds.
    pub creator: Signer<'info>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub campaign_coordinator: Account<'info, CampaignCoordinator>,
}
