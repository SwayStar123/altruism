import {
  Amman,
  ConfirmedTransactionDetails,
  LOCALHOST,
  PayerTransactionHandler,
} from '@metaplex-foundation/amman-client';
import { Connection, Transaction, Keypair } from '@solana/web3.js';
import { bignum } from '@metaplex-foundation/beet';
import {
  createCreateTokenAccountInstruction,
  createDepositInstruction,
  createInitializeInstruction,
  CreateTokenAccountInstructionAccounts,
  DepositInstructionAccounts,
  findBeneficiaryPda,
  GLOBAL_SOL_VAULT_PUBKEY,
  InitializeInstructionAccounts,
  M_STATE_PUBKEY,
  MARINADE_FINANCE_PUBKEY,
  MSOL_LEG_AUTHORITY_PUBKEY,
  MSOL_LEG_PUBKEY,
  MSOL_MINT_AUTHORITY_PUBKEY,
  MSOL_MINT_PUBKEY,
  MSOL_VAULT_PUBKEY,
  PROGRAM_BENEFICIARY_PUBKEY,
  RESERVE_PDA_PUBKEY,
  SOL_LEG_PDA_PUBKEY,
} from '@altruism-finance/js';
class AltruismFinance {
  public initialized = false;
  public amman: Amman;
  public connection: Connection;
  public altSolMint?: Keypair;
  public msolTokenAccount?: Keypair;
  public authority?: Keypair;
  public altState?: Keypair;
  public user?: Keypair;
  public searcher?: Keypair;
  public userTokenAccount?: Keypair;
  public authHandler?: PayerTransactionHandler;
  public userHandler?: PayerTransactionHandler;
  public searcherHandler?: PayerTransactionHandler;

  constructor() {
    this.amman = Amman.instance();
    this.connection = new Connection(LOCALHOST, 'confirmed');
  }

  private assertInit(): asserts this is {
    amman: Amman;
    connection: Connection;
    altSolMint: Keypair;
    msolTokenAccount: Keypair;
    altState: Keypair;
    userTokenAccount: Keypair;
    authority: Keypair;
    user: Keypair;
    searcher: Keypair;
    authHandler: PayerTransactionHandler;
    userHandler: PayerTransactionHandler;
    searcherHandler: PayerTransactionHandler;
  } {
    if (!this.initialized) {
      throw new Error('Not initialized! Call .init() first!');
    }
  }

  async init() {
    const amman = this.amman;
    const connection = this.connection;

    const newLabels: Record<string, string> = {
      [MSOL_VAULT_PUBKEY.toString()]: 'msol_vault',
      [GLOBAL_SOL_VAULT_PUBKEY.toString()]: 'global_sol_vault',
      [PROGRAM_BENEFICIARY_PUBKEY.toString()]: 'beneficiary',
      [MSOL_LEG_PUBKEY.toString()]: 'msol_leg',
      [MSOL_LEG_AUTHORITY_PUBKEY.toString()]: 'msol_leg_authority',
      [MSOL_MINT_AUTHORITY_PUBKEY.toString()]: 'msol_leg_authority',
    };
    await amman.ammanClient.addAddressLabels(newLabels);

    this.authority = (await amman.genLabeledKeypair('authority'))[1];
    this.user = (await amman.genLabeledKeypair('user'))[1];
    this.userTokenAccount = (await amman.genLabeledKeypair('userTokenAccount'))[1];
    this.searcher = (await amman.genLabeledKeypair('searcher'))[1];
    this.altSolMint = (await amman.genLabeledKeypair('altSolMint'))[1];
    this.altState = (await amman.genLabeledKeypair('altState'))[1];
    this.msolTokenAccount = (await this.amman.genLabeledKeypair('mSolTokenAccount'))[1];
    this.authHandler = this.amman.payerTransactionHandler(connection, this.authority);
    this.userHandler = this.amman.payerTransactionHandler(connection, this.user);
    this.searcherHandler = this.amman.payerTransactionHandler(connection, this.searcher);
    await this.amman.airdrop(connection, this.authority.publicKey, 10);
    await this.amman.airdrop(connection, this.user.publicKey, 10);
    await this.amman.airdrop(connection, this.searcher.publicKey, 10);
    this.initialized = true;
    this.assertInit();
  }

  async initialize(): Promise<ConfirmedTransactionDetails> {
    this.assertInit();
    // initialize
    let initializeAccounts: InitializeInstructionAccounts = {
      globalSolVault: GLOBAL_SOL_VAULT_PUBKEY,
      mState: M_STATE_PUBKEY,
      msolTokenAccount: this.msolTokenAccount.publicKey,
      state: this.altState.publicKey,
      vault: MSOL_VAULT_PUBKEY,
      mint: this.altSolMint.publicKey,
      payer: this.authority.publicKey,
      beneficiary: PROGRAM_BENEFICIARY_PUBKEY,
      msolMint: MSOL_MINT_PUBKEY,
    };
    let ix = createInitializeInstruction(initializeAccounts);
    const initTx = new Transaction().add(ix);
    return await this.authHandler
      .sendAndConfirmTransaction(
        initTx,
        [this.msolTokenAccount, this.altSolMint, this.altState],
        { skipPreflight: true },
        'initializeTxn',
      )
      .assertNone();
  }

  async createTokenAccount(): Promise<ConfirmedTransactionDetails> {
    this.assertInit();
    let beneficiary = findBeneficiaryPda(this.user.publicKey);
    let createTokenAccountAccounts: CreateTokenAccountInstructionAccounts = {
      authority: this.user.publicKey,
      tokenAccount: this.userTokenAccount.publicKey,
      state: this.altState.publicKey,
      mint: this.altSolMint.publicKey,
      beneficiary,
    };
    let ix = createCreateTokenAccountInstruction(createTokenAccountAccounts);
    const createTx = new Transaction().add(ix);
    return await this.userHandler
      .sendAndConfirmTransaction(
        createTx,
        [this.userTokenAccount],
        { skipPreflight: true },
        'createTxn',
      )
      .assertNone();
  }

  async deposit(amount: number): Promise<ConfirmedTransactionDetails> {
    this.assertInit();
    let depositAccounts: DepositInstructionAccounts = {
      liqPoolMsolLeg: MSOL_LEG_PUBKEY,
      liqPoolMsolLegAuthority: MSOL_LEG_AUTHORITY_PUBKEY,
      liqPoolSolLegPda: SOL_LEG_PDA_PUBKEY,
      marinadeFinanceProgram: MARINADE_FINANCE_PUBKEY,
      marinadeState: M_STATE_PUBKEY,
      mintTo: this.msolTokenAccount.publicKey,
      msolMint: MSOL_MINT_PUBKEY,
      msolMintAuthority: MSOL_MINT_AUTHORITY_PUBKEY,
      reservePda: RESERVE_PDA_PUBKEY,
      token: this.userTokenAccount.publicKey,
      vault: MSOL_VAULT_PUBKEY,
      authority: this.user.publicKey,
      state: this.altState.publicKey,
      mint: this.altSolMint.publicKey,
    };
    let ix = createDepositInstruction(depositAccounts, amount as bignum);
    const depositTxn = new Transaction().add(ix);
    return await this.userHandler
      .sendAndConfirmTransaction(depositTxn, [], { skipPreflight: true }, 'depositTxn')
      .assertNone();
  }
}

async function main() {
  let altruism = new AltruismFinance();
  await altruism.init();
  let initResult = await altruism.initialize();
  let createResult = await altruism.createTokenAccount();
  let depositResult = await altruism.deposit(0);
  // let depositResult2 = await altruism.deposit(2e9);
}

main()
  .then(() => process.exit(0))
  .catch((err: any) => {
    console.error(err);
    process.exit(1);
  });
