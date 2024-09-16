use anchor_lang::AccountDeserialize;
use solana_program_test::{ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::Account, clock::Clock, pubkey::Pubkey, signature::Keypair, signer::Signer,
};

pub struct SetupTest {
    pub validator: ProgramTest,
    pub creators: [Keypair; 2],
    pub donors: [Keypair; 3],
    pub coordinator: Keypair,
    pub initializer: Keypair,
}

impl SetupTest {
    pub fn new() -> Self {
        let mut validator = ProgramTest::new("hiraise", hiraise::ID, None);
        let creators = [Keypair::new(), Keypair::new()];
        let donors = [Keypair::new(), Keypair::new(), Keypair::new()];
        let initializer = Keypair::new();

        validator.add_account(
            initializer.pubkey(),
            Account {
                lamports: 1_000_000_000,
                ..Account::default()
            },
        );

        for creator in creators.iter() {
            validator.add_account(
                creator.pubkey(),
                Account {
                    lamports: 2_000_000_000,
                    ..Account::default()
                },
            );
        }

        for donor in donors.iter() {
            validator.add_account(
                donor.pubkey(),
                Account {
                    lamports: 1_000_000_000,
                    ..Account::default()
                },
            );
        }

        Self {
            validator,
            creators,
            donors,
            coordinator: Keypair::new(),
            initializer,
        }
    }
}

pub async fn load_and_deserialize<T: AccountDeserialize>(
    ctx: &mut ProgramTestContext,
    address: Pubkey,
) -> T {
    let account = ctx
        .banks_client
        .get_account(address)
        .await
        .unwrap() //unwraps the Result into an Option<Account>
        .unwrap(); //unwraps the Option<Account> into an Account

    T::try_deserialize(&mut account.data.as_slice()).unwrap()
}

pub async fn get_balance(ctx: &mut ProgramTestContext, address: Pubkey) -> u64 {
    ctx.banks_client.get_balance(address).await.unwrap()
}

pub async fn get_account(ctx: &mut ProgramTestContext, address: Pubkey) -> Account {
    ctx.banks_client
        .get_account(address)
        .await
        .unwrap()
        .unwrap()
}

pub async fn manipulate_time(ctx: &mut ProgramTestContext, duration: u64) {
    let mut current_time = ctx.banks_client.get_sysvar::<Clock>().await.unwrap();
    current_time.unix_timestamp = current_time.unix_timestamp + (duration as i64);
    ctx.set_sysvar::<Clock>(&current_time);
}
