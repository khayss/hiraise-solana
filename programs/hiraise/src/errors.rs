use anchor_lang::error_code;

#[error_code]
pub enum CampaignError {
    #[msg("campaign already initialized")]
    CampaignAlreadyInitialized,

    #[msg("campaign is not initialized")]
    CampaignIsEnded,

    #[msg("wrong creator account")]
    WrongCreatorAccount,

    #[msg("campaign is not active")]
    CampaignNotActive,

    #[msg("campaign is not ended yet")]
    CampaignNotEndedYet,

    #[msg("campaign is already ended")]
    InvalidGoal,

    #[msg("invalid duration")]
    InvalidDuration,
}

#[error_code]
pub enum CampaignCoordinatorError {
    #[msg("campaign coordinator already initialized")]
    AlreadyInitialized,
    #[msg("campaign coordinator is not initialized")]
    NotInitialized,
    #[msg("invalid campaign id")]
    InvalidCampaignId,
}
