// #![cfg(feature = "test-bpf")]

use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use hiraise::states::{
    campaign::campaign::Campaign, campaign_cordinator::campaign_coordinator::CampaignCoordinator,
};
use solana_program_test::{tokio, ProgramTestContext};
use solana_sdk::{
    account::ReadableAccount,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod common;

// #[ignore = ""]
#[tokio::test]
async fn test_initialize() {
    let common::SetupTest {
        validator,
        creators: _,
        donors: _,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;

    init(&initializer, &coordinator, &mut context).await;

    let coordinator_account = common::get_account(&mut context, coordinator.pubkey()).await;

    let coordinator_account_data: CampaignCoordinator =
        common::load_and_deserialize(&mut context, coordinator.pubkey()).await;

    assert!(
        !coordinator_account.executable(),
        "asserting coordinator account is not executable"
    );
    assert_eq!(
        coordinator_account.owner,
        hiraise::ID,
        "asserting coordinator account owner is hiraise program id"
    );

    assert!(
        coordinator_account_data.is_initialized,
        "asserting campaign coordinator is initialized"
    );

    assert_eq!(
        coordinator_account_data.initializer,
        initializer.pubkey(),
        "asserting campaign coordinator initializer is correct"
    );
    assert_eq!(
        coordinator_account_data.closed_campaigns, 0,
        "asserting closed campaigns is correctly set"
    );
    assert_eq!(
        coordinator_account_data.total_campaigns, 0,
        "asserting total campaigns is correctly set"
    );
}

// #[ignore = ""]
#[tokio::test]
async fn test_create_campaign() {
    let common::SetupTest {
        validator,
        creators,
        donors: _,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];

    init(&initializer, &coordinator, &mut context).await;

    let (goal, duration, bump, campaign_account) =
        create_campaign(&mut context, &creator, &coordinator).await;

    let campaign_account_data: Campaign =
        common::load_and_deserialize(&mut context, campaign_account).await;

    assert_eq!(
        campaign_account_data.creator,
        creator.pubkey(),
        "asserting campaign creator is correct"
    );
    assert_eq!(
        campaign_account_data.goal, goal,
        "asserting campaign goal is correct"
    );
    assert_eq!(
        campaign_account_data.duration, duration,
        "asserting campaign duration is correct"
    );
    assert_eq!(
        campaign_account_data.amount_raised, 0,
        "asserting campaign amount raised is correct"
    );
    assert_eq!(
        campaign_account_data.status,
        hiraise::states::campaign::campaign::CampaignStatus::Active,
        "asserting campaign status is open"
    );
    assert_eq!(
        campaign_account_data.bump, bump,
        "asserting campaign bump is correct"
    );
}

// #[ignore = ""]
#[tokio::test]
async fn test_donate() {
    let common::SetupTest {
        validator,
        creators,
        donors,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];

    init(&initializer, &coordinator, &mut context).await;

    let (_, _, _, campaign_account) = create_campaign(&mut context, &creator, &coordinator).await;

    let campaign_bal_before = common::get_balance(&mut context, campaign_account).await;

    let donate_amount = donate(&donors[1], campaign_account, &mut context).await;

    let campaign_bal_after = common::get_balance(&mut context, campaign_account).await;

    let campaign_data: Campaign =
        common::load_and_deserialize(&mut context, campaign_account).await;

    assert_eq!(
        campaign_data.amount_raised, donate_amount,
        "asserting amount raised is correctly set"
    );
    assert!(
        campaign_bal_after > campaign_bal_before,
        "asserting campaign balance after is greater than campaign balance before"
    );
    assert_eq!(
        campaign_bal_after - campaign_bal_before,
        donate_amount,
        "asserting campaign balance after minus campaign balance before is equal to donate amount"
    );
}

// #[ignore = ""]
#[tokio::test]
async fn test_multiple_donations() {
    let common::SetupTest {
        validator,
        creators,
        donors,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];

    init(&initializer, &coordinator, &mut context).await;

    let (_, _, _, campaign_account) = create_campaign(&mut context, &creator, &coordinator).await;

    let campaign_bal_before = common::get_balance(&mut context, campaign_account).await;

    let mut total_donation = 0;

    for donor in donors.iter() {
        let donate_amount = donate(&donor, campaign_account, &mut context).await;
        total_donation += donate_amount;
    }

    let campaign_bal_after = common::get_balance(&mut context, campaign_account).await;

    let campaign_data: Campaign =
        common::load_and_deserialize(&mut context, campaign_account).await;

    assert_eq!(campaign_data.amount_raised, total_donation);
    assert!(campaign_bal_after > campaign_bal_before);
    assert_eq!(campaign_bal_after - campaign_bal_before, total_donation);
}

// #[ignore = ""]
#[tokio::test]
async fn test_can_end_campaign() {
    let common::SetupTest {
        validator,
        creators,
        donors,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];

    init(&initializer, &coordinator, &mut context).await;

    let (_, duration, _, campaign_account) =
        create_campaign(&mut context, &creator, &coordinator).await;

    let initial_campaign_bal = common::get_balance(&mut context, campaign_account).await;

    let mut total_donation = 0;

    let donate_amount = donate(&donors[0], campaign_account, &mut context).await;
    total_donation += donate_amount;
    let donate_amount = donate(&donors[1], campaign_account, &mut context).await;
    total_donation += donate_amount;

    common::manipulate_time(&mut context, duration + 1).await;

    let creator_bal_before = common::get_balance(&mut context, creator.pubkey()).await;
    let campaign_bal_before = common::get_balance(&mut context, campaign_account).await;

    end_campaign(&creator, &coordinator, campaign_account, &mut context).await;

    let creator_bal_after = common::get_balance(&mut context, creator.pubkey()).await;
    let campaign_bal_after = common::get_balance(&mut context, campaign_account).await;

    let campaign_data: Campaign =
        common::load_and_deserialize(&mut context, campaign_account).await;

    assert_eq!(
        campaign_data.status,
        hiraise::states::campaign::campaign::CampaignStatus::Close,
        "asserting campaign status is closed"
    );
    assert_eq!(
        campaign_data.amount_raised, total_donation,
        "asserting amount raised is correctly set"
    );
    assert!(
        creator_bal_after > creator_bal_before,
        "asserting creator balance after minus creator balance before is equal to total donation"
    );
    assert_eq!(
        campaign_bal_after, initial_campaign_bal,
        "asserting campaign balance after is equal to initial campaign balance"
    );
    assert_eq!(
        campaign_bal_before - initial_campaign_bal,
        total_donation,
        "asserting campaign balance before is equal to total donation"
    );
}

// #[ignore = ""]
#[should_panic]
#[tokio::test]
async fn test_cannot_donate_after_campaign_end() {
    let common::SetupTest {
        validator,
        creators,
        donors,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];

    init(&initializer, &coordinator, &mut context).await;

    let (_, duration, _, campaign_account) =
        create_campaign(&mut context, &creator, &coordinator).await;

    donate(&donors[0], campaign_account, &mut context).await;
    donate(&donors[1], campaign_account, &mut context).await;

    common::manipulate_time(&mut context, duration + 1).await;

    donate(&donors[2], campaign_account, &mut context).await;
}

#[should_panic]
#[tokio::test]
async fn test_cannot_end_campaign_twice() {
    let common::SetupTest {
        validator,
        creators,
        donors,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];

    init(&initializer, &coordinator, &mut context).await;

    let (_, duration, _, campaign_account) =
        create_campaign(&mut context, &creator, &coordinator).await;

    donate(&donors[0], campaign_account, &mut context).await;
    donate(&donors[1], campaign_account, &mut context).await;

    common::manipulate_time(&mut context, duration + 1).await;

    end_campaign(&creator, &coordinator, campaign_account, &mut context).await;

    end_campaign(&creator, &coordinator, campaign_account, &mut context).await;
}

// #[ignore = ""]
#[should_panic]
#[tokio::test]
async fn test_only_creator_can_receive_funds() {
    let common::SetupTest {
        validator,
        creators,
        donors,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];
    let creator_2 = &creators[1];

    init(&initializer, &coordinator, &mut context).await;

    let (_, duration, _, campaign_account) =
        create_campaign(&mut context, &creator, &coordinator).await;

    donate(&donors[0], campaign_account, &mut context).await;
    donate(&donors[1], campaign_account, &mut context).await;

    common::manipulate_time(&mut context, duration + 1).await;

    end_campaign(&creator_2, &coordinator, campaign_account, &mut context).await;
}

// #[ignore = ""]
#[tokio::test]
async fn test_can_create_multiple_campaigns() {
    let common::SetupTest {
        validator,
        creators,
        donors,
        coordinator,
        initializer,
    } = common::SetupTest::new();
    let mut context = validator.start_with_context().await;
    let creator = &creators[0];
    let creator_2 = &creators[1];

    init(&initializer, &coordinator, &mut context).await;

    let (_, duration, _, campaign_account) =
        create_campaign(&mut context, &creator, &coordinator).await;

    let (_, duration_2, _, campaign_account_2) =
        create_campaign(&mut context, &creator_2, &coordinator).await;

    donate(&donors[0], campaign_account, &mut context).await;
    donate(&donors[1], campaign_account, &mut context).await;
    donate(&donors[0], campaign_account_2, &mut context).await;
    donate(&donors[1], campaign_account_2, &mut context).await;

    common::manipulate_time(&mut context, duration + 1).await;
    common::manipulate_time(&mut context, duration_2 + 1).await;

    end_campaign(&creator, &coordinator, campaign_account, &mut context).await;
    end_campaign(&creator_2, &coordinator, campaign_account_2, &mut context).await;

    let campaign_data_1: Campaign =
        common::load_and_deserialize(&mut context, campaign_account).await;
    let campaign_data_2: Campaign =
        common::load_and_deserialize(&mut context, campaign_account_2).await;
    let coordinator_data: CampaignCoordinator =
        common::load_and_deserialize(&mut context, coordinator.pubkey()).await;

    assert_eq!(
        campaign_data_1.status,
        hiraise::states::campaign::campaign::CampaignStatus::Close,
        "asserting campaign 1 status is closed"
    );
    assert_eq!(
        campaign_data_2.status,
        hiraise::states::campaign::campaign::CampaignStatus::Close,
        "asserting campaign 2 status is closed"
    );
    assert_eq!(
        coordinator_data.total_campaigns, 2,
        "asserting total campaigns is correctly set"
    );
}

// ========================================================================================
// HELPERS:
// ========================================================================================
pub async fn init(initializer: &Keypair, coordinator: &Keypair, context: &mut ProgramTestContext) {
    let init_ix = Instruction {
        program_id: hiraise::ID,
        accounts: hiraise::accounts::InitializeCoordinator {
            campaign_cordinator: coordinator.pubkey(),
            initializer: initializer.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: hiraise::instruction::InitializeCoordinator {}.data(),
    };

    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&initializer.pubkey()),
        &[&initializer, &coordinator],
        context.banks_client.get_latest_blockhash().await.unwrap(),
    );

    context
        .banks_client
        .process_transaction(init_tx)
        .await
        .unwrap();
}

pub async fn create_campaign(
    context: &mut ProgramTestContext,
    creator: &Keypair,
    coordinator: &Keypair,
) -> (u64, u64, u8, Pubkey) {
    let goal = 200_000_000;
    let duration = 1000;
    let coordinator_data: CampaignCoordinator =
        common::load_and_deserialize(context, coordinator.pubkey()).await;
    let campaign_id = coordinator_data.total_campaigns + 1;
    let (campaign_key, bump) = Pubkey::find_program_address(
        &[
            hiraise::instructions::campaign::campaign_create::CAMPAIGN_TAG,
            &creator.pubkey().as_ref(),
            &campaign_id.to_le_bytes(),
        ],
        &hiraise::ID,
    );

    let create_campaign_ix = Instruction {
        program_id: hiraise::ID,
        accounts: hiraise::accounts::CreateCampaign {
            campaign_coordinator: coordinator.pubkey(),
            creator: creator.pubkey(),
            system_program: system_program::ID,
            campaign: campaign_key,
        }
        .to_account_metas(None),
        data: hiraise::instruction::CreateCampaign {
            campaign_id,
            goal,
            duration,
        }
        .data(),
    };

    let create_campaign_tx = Transaction::new_signed_with_payer(
        &[create_campaign_ix],
        Some(&creator.pubkey()),
        &[&creator],
        context.banks_client.get_latest_blockhash().await.unwrap(),
    );

    context
        .banks_client
        .process_transaction(create_campaign_tx)
        .await
        .unwrap();

    (goal, duration, bump, campaign_key)
}

pub async fn donate(
    donor: &Keypair,
    campaign_account: Pubkey,

    context: &mut ProgramTestContext,
) -> u64 {
    let donate_amount = 100_000_000;

    let donate_ix = Instruction {
        program_id: hiraise::ID,
        accounts: hiraise::accounts::Donate {
            donor: donor.pubkey(),
            campaign: campaign_account,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: hiraise::instruction::Donate {
            amount: donate_amount,
        }
        .data(),
    };

    let donate_tx = Transaction::new_signed_with_payer(
        &[donate_ix],
        Some(&donor.pubkey()),
        &[&donor],
        context.banks_client.get_latest_blockhash().await.unwrap(),
    );
    context
        .banks_client
        .process_transaction(donate_tx)
        .await
        .unwrap();

    donate_amount
}

async fn end_campaign(
    creator: &Keypair,
    campaign_coordinator: &Keypair,
    campaign_account: Pubkey,
    context: &mut ProgramTestContext,
) {
    let end_campaign_ix = Instruction {
        program_id: hiraise::ID,
        accounts: hiraise::accounts::EndCampaign {
            campaign_coordinator: campaign_coordinator.pubkey(),
            creator: creator.pubkey(),
            campaign: campaign_account,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: hiraise::instruction::EndCampaign {}.data(),
    };

    let end_campaign_tx = Transaction::new_signed_with_payer(
        &[end_campaign_ix],
        Some(&creator.pubkey()),
        &[&creator],
        context.banks_client.get_latest_blockhash().await.unwrap(),
    );

    context
        .banks_client
        .process_transaction(end_campaign_tx)
        .await
        .unwrap();
}
