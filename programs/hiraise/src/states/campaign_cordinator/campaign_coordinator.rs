use anchor_lang::prelude::*;

use crate::errors::CampaignCoordinatorError;

// use super::Campaign;

#[account]
#[derive(InitSpace)]
pub struct CampaignCoordinator {
    pub total_campaigns: u32,
    pub closed_campaigns: u32,
    pub is_initialized: bool,
    pub initializer: Pubkey,
    // #[max_len(10)]
    // campaigns: Vec<Campaign>,
}

impl CampaignCoordinator {
    pub fn init(&mut self, initializer: Pubkey) -> Result<()> {
        self.check_not_initialized()?;

        self.is_initialized = true;
        self.total_campaigns = 0;
        self.closed_campaigns = 0;
        self.initializer = initializer;

        Ok(())
    }

    pub fn create_campaign(&mut self, campaign_id: u32) -> Result<()> {
        self.check_is_valid_id(campaign_id)?;
        self.check_is_initialized()?;

        self.total_campaigns += 1;

        msg!("Campaign Created: {:#?}", self.total_campaigns);
        Ok(())
    }

    pub fn close_campaign(&mut self) -> Result<()> {
        self.check_is_initialized()?;
        self.closed_campaigns += 1;
        Ok(())
    }

    fn check_is_initialized(&self) -> Result<()> {
        require!(
            self.is_initialized,
            CampaignCoordinatorError::NotInitialized
        );
        Ok(())
    }

    fn check_not_initialized(&self) -> Result<()> {
        require!(
            !self.is_initialized,
            CampaignCoordinatorError::AlreadyInitialized
        );
        Ok(())
    }

    fn check_is_valid_id(&self, campaign_id: u32) -> Result<()> {
        require_eq!(
            self.total_campaigns,
            campaign_id - 1,
            CampaignCoordinatorError::InvalidCampaignId
        );
        Ok(())
    }
}
