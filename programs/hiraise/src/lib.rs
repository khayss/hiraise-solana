pub mod errors;
pub mod instructions;
pub mod states;

use anchor_lang::{prelude::*, system_program};

use errors::CampaignError;
use instructions::{
    campaign::{campaign_create::*, campaign_donate::*, campaign_end::*},
    campaign_coordinator::initialize_coordinator::*,
};

declare_id!("B4ZgDcF5jok9LMkPzi3LexmhWfxr63rj5dAeas98h2rT");

#[program]
pub mod hiraise {
    use super::*;

    pub fn initialize_coordinator(ctx: Context<InitializeCoordinator>) -> Result<()> {
        let initializer = &mut ctx.accounts.initializer;
        
        ctx.accounts.campaign_cordinator.init(initializer.key())?;

        msg!("Campaign Coordinator Initialized: {:#?}", ctx.program_id);
        Ok(())
    }

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        campaign_id: u32,
        goal: u64,
        duration: u64,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let coordinator = &mut ctx.accounts.campaign_coordinator;
        let bump = ctx.bumps.campaign;

        coordinator.create_campaign(campaign_id)?;
        campaign.init(
            campaign_id,
            bump,
            ctx.accounts.creator.key(),
            goal,
            duration,
        )?;

        msg!("Campaign Created: {:#?}", campaign.key());
        Ok(())
    }

    pub fn end_campaign(ctx: Context<EndCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let coordinator = &mut ctx.accounts.campaign_coordinator;

        require_keys_eq!(
            ctx.accounts.creator.key(),
            campaign.creator.key(),
            CampaignError::WrongCreatorAccount
        );

        coordinator.close_campaign()?;
        campaign.end_campaign()?;

        if campaign.amount_raised > 0 {
            campaign.sub_lamports(campaign.amount_raised)?;
            **ctx.accounts.creator.try_borrow_mut_lamports()? += campaign.amount_raised;
        }

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let system_program_account = &ctx.accounts.system_program;
        let donor_account = &ctx.accounts.donor;
        let program_account = &ctx.accounts.campaign;

        let donate_cpi_context = CpiContext::new(
            system_program_account.to_account_info(),
            system_program::Transfer {
                from: donor_account.to_account_info(),
                to: program_account.to_account_info(),
            },
        );

        system_program::transfer(donate_cpi_context, amount)?;

        ctx.accounts.campaign.donate(amount)?;
        Ok(())
    }
}
