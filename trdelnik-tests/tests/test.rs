use fehler::throws;
use program_client::altruism_instruction;
use trdelnik_client::{anyhow::Result, *};
// @todo: do not forget to import your program crate (also in the ../Cargo.toml)
use altruism;
use anchor_spl::token;
use anchor_lang::solana_program::sysvar::rent;


// @todo: create and deploy your fixture
#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let fixture = Fixture::new();
    // @todo: here you can call your <program>::initialize instruction
    // fixture.deploy().await?;
    fixture.client
        .deploy_by_name(&fixture.program, "altruism")
        .await?;

    altruism_instruction::create_mint(
        &fixture.client,
        fixture.mint.pubkey(),
        fixture.client.payer().pubkey(),
        System::id(),
        token::ID,
        rent::ID,
        [fixture.mint.clone()]
    ).await?;

    fixture
}

#[trdelnik_test]
async fn test_happy_path(#[future] init_fixture: Result<Fixture>) {
    // @todo: add your happy path test scenario and the other test cases
    let default_fixture = Fixture::new();
    let fixture = init_fixture.await?;
    assert_eq!(fixture.program, default_fixture.program);

    altruism_instruction::create_token_account(
        &fixture.client, 
        fixture.account.pubkey(), 
        fixture.mint.pubkey(), 
        fixture.account.pubkey(), 
        System::id(), 
        token::ID, 
        rent::ID, 
        [fixture.mint.clone()]
    ).await?;
}

// @todo: design and implement all the logic you need for your fixture(s)
struct Fixture {
    client: Client,
    program: Keypair,
    mint: Keypair,
    account: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            program: program_keypair(1),
            mint: keypair(42),
            account: keypair(43),
        }
    }

    // #[throws]
    // async fn deploy(&mut self) {
    //     self.client
    //         .airdrop(self.client.payer().pubkey(), 5_000_000_000)
    //         .await?;
    // }
}
