use anchor_lang::prelude::*;

use crate::states::campaign_cordinator::campaign_coordinator::CampaignCoordinator;

#[derive(Accounts)]
pub struct InitializeCoordinator<'info> {
    #[account(init, payer = initializer, space = 8 + CampaignCoordinator::INIT_SPACE)]
    pub campaign_cordinator: Account<'info, CampaignCoordinator>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// impl<'info> Initialize<'info> {
//     pub fn derive_campaign_pda(program_id: &Pubkey, creator: &Pubkey) -> (Pubkey, u8) {
//         let (pda, bump) =
//             Pubkey::find_program_address(&[b"campaign", creator.as_ref()], program_id);
//         (pda, bump)
//     }
// }
