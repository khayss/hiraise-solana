use anchor_lang::prelude::*;

use crate::states::{
    campaign::campaign::Campaign, campaign_cordinator::campaign_coordinator::CampaignCoordinator,
};

pub const CAMPAIGN_TAG: &[u8] = b"campaign";

#[derive(Accounts)]
#[instruction(campaign_id: u32)]
pub struct CreateCampaign<'info> {
    #[account(init, payer = creator, space = 8 + Campaign::INIT_SPACE, seeds = [CAMPAIGN_TAG, creator.key().as_ref(), &campaign_id.to_le_bytes()], bump)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub campaign_coordinator: Account<'info, CampaignCoordinator>,
    pub system_program: Program<'info, System>,
}
