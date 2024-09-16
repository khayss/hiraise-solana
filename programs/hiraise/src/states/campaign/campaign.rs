use std::fmt::Display;

use anchor_lang::prelude::*;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::errors::CampaignError;

#[account]
#[derive(InitSpace, Debug)]
pub struct Campaign {
    pub id: u32,                // 4
    pub creator: Pubkey,        // 32
    pub goal: u64,              // 8
    pub duration: u64,          // 8
    pub start_time: i64,        // 8
    pub end_time: i64,          // 8
    pub amount_raised: u64,     // 8
    pub status: CampaignStatus, // 1 + 1
    pub bump: u8,               // 1
}

#[derive(
    Debug,
    Clone,
    AnchorDeserialize,
    AnchorSerialize,
    PartialEq,
    Eq,
    FromPrimitive,
    ToPrimitive,
    InitSpace,
)]
pub enum CampaignStatus {
    Pending,
    Active,
    Close,
}

impl Display for CampaignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CampaignStatus::Pending => write!(f, "Pending"),
            CampaignStatus::Active => write!(f, "Active"),
            CampaignStatus::Close => write!(f, "Close"),
            // _ => write!(f, "Unknown"),
        }
    }
}

impl PartialEq<&CampaignStatus> for CampaignStatus {
    fn eq(&self, other: &&CampaignStatus) -> bool {
        match (self, other) {
            (CampaignStatus::Pending, CampaignStatus::Pending) => true,
            (CampaignStatus::Active, CampaignStatus::Active) => true,
            (CampaignStatus::Close, CampaignStatus::Close) => true,
            _ => false,
        }
    }
}

impl PartialEq<CampaignStatus> for &CampaignStatus {
    fn eq(&self, other: &CampaignStatus) -> bool {
        match (self, other) {
            (CampaignStatus::Pending, CampaignStatus::Pending) => true,
            (CampaignStatus::Active, CampaignStatus::Active) => true,
            (CampaignStatus::Close, CampaignStatus::Close) => true,
            _ => false,
        }
    }
}

impl Campaign {
    pub fn init(
        &mut self,
        id: u32,
        bump: u8,
        creator: Pubkey,
        goal: u64,
        duration: u64,
    ) -> Result<()> {
        self.check_is_pending()?;

        require!(goal > 0, CampaignError::InvalidGoal);
        require!(duration > 0, CampaignError::InvalidDuration);

        let now = Clock::get()?.unix_timestamp;

        self.creator = creator;
        self.goal = goal;
        self.duration = duration;
        self.id = id;
        self.amount_raised = 0;
        self.status = CampaignStatus::Active;
        self.start_time = now;
        self.end_time = now + duration as i64;
        self.bump = bump;
        Ok(())
    }

    pub fn end_campaign(&mut self) -> Result<()> {
        self.check_is_active()?;

        let now = Clock::get()?.unix_timestamp;
        require!(now > self.end_time, CampaignError::CampaignNotEndedYet);

        self.status = CampaignStatus::Close;

        Ok(())
    }

    pub fn donate(&mut self, amount: u64) -> Result<()> {
        self.check_is_active()?;

        let now = Clock::get()?.unix_timestamp;
        require!(now < self.end_time, CampaignError::CampaignIsEnded);

        self.amount_raised += amount;
        Ok(())
    }

    fn check_is_active(&self) -> Result<()> {
        require_eq!(
            &self.status,
            CampaignStatus::Active,
            CampaignError::CampaignNotActive
        );
        Ok(())
    }

    fn check_is_pending(&self) -> Result<()> {
        require_eq!(
            &self.status,
            CampaignStatus::Pending,
            CampaignError::CampaignAlreadyInitialized
        );
        Ok(())
    }
}
