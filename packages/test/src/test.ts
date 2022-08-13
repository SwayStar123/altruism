import { Amman, LOCALHOST } from '@metaplex-foundation/amman-client';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import {
  createInitializeInstruction,
  InitializeInstructionAccounts,
  PROGRAM_ID,
} from '@altruism-finance/js';

async function main() {
  const amman = Amman.instance();
  const labels = await amman.ammanClient.fetchAddressLabels();
  const connection = new Connection(LOCALHOST, 'confirmed');

  const [authority, authorityPair] = await amman.genLabeledKeypair('authority');
  const [altSolMint, altSolMintKeypair] = await amman.genLabeledKeypair('altSolMint');
  const [state, stateKeypair] = await amman.genLabeledKeypair('altState');
  const [msolTokenAccount, msolTokenAccountKeypair] = await amman.genLabeledKeypair(
    'mSolTokenAccount',
  );

  await amman.airdrop(connection, authority, 5);
  const handler = amman.payerTransactionHandler(connection, authorityPair);

  const BENEFICIARY_PREFIX = 'beneficiary';
  const [BENEFICIARY_PUBKEY] = PublicKey.findProgramAddressSync(
    [new Buffer(BENEFICIARY_PREFIX)],
    PROGRAM_ID,
  );

  const GLOBAL_SOL_VAULT_PREFIX = 'global_sol_vault';
  const [GLOBAL_SOL_VAULT_PUBKEY] = PublicKey.findProgramAddressSync(
    [new Buffer(GLOBAL_SOL_VAULT_PREFIX)],
    PROGRAM_ID,
  );

  const MSOL_VAULT_PREFIX = 'msol_vault';
  const [MSOL_VAULT_PUBKEY] = PublicKey.findProgramAddressSync(
    [new Buffer(MSOL_VAULT_PREFIX)],
    PROGRAM_ID,
  );

  const newLabels: Record<string, string> = {
    [BENEFICIARY_PUBKEY.toString()]: 'beneficiary',
    [GLOBAL_SOL_VAULT_PUBKEY.toString()]: 'global_sol_vault',
    [MSOL_VAULT_PUBKEY.toString()]: 'msol_vault',
  };
  await amman.ammanClient.addAddressLabels(newLabels);

  const MSOL_MINT_PUBKEY = new PublicKey('mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So');

  const MSTATE_PUBKEY = new PublicKey('8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC');

  // initialize
  let initializeAccounts: InitializeInstructionAccounts = {
    globalSolVault: GLOBAL_SOL_VAULT_PUBKEY,
    mState: MSTATE_PUBKEY,
    msolTokenAccount,
    state,
    vault: MSOL_VAULT_PUBKEY,
    mint: altSolMint,
    payer: authority,
    beneficiary: BENEFICIARY_PUBKEY,
    msolMint: MSOL_MINT_PUBKEY,
  };
  let ix = createInitializeInstruction(initializeAccounts);
  const init_tx = new Transaction().add(ix);
  const result = await handler
    .sendAndConfirmTransaction(
      init_tx,
      [msolTokenAccountKeypair, altSolMintKeypair, stateKeypair],
      { skipPreflight: true },
      'initIx',
    )
    .assertNone();
}

main()
  .then(() => process.exit(0))
  .catch((err: any) => {
    console.error(err);
    process.exit(1);
  });
