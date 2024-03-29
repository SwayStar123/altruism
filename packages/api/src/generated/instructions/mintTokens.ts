/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token';
import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category MintTokens
 * @category generated
 */
export type MintTokensInstructionArgs = {
  amount: beet.bignum;
};
/**
 * @category Instructions
 * @category MintTokens
 * @category generated
 */
export const mintTokensStruct = new beet.BeetArgsStruct<
  MintTokensInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['amount', beet.u64],
  ],
  'MintTokensInstructionArgs',
);
/**
 * Accounts required by the _mintTokens_ instruction
 *
 * @property [_writable_] state
 * @property [_writable_] token
 * @property [_writable_] mint
 * @property [_writable_, **signer**] authority
 * @property [_writable_] vault
 * @property [_writable_] marinadeState
 * @property [_writable_] msolMint
 * @property [_writable_] liqPoolSolLegPda
 * @property [_writable_] liqPoolMsolLeg
 * @property [] liqPoolMsolLegAuthority
 * @property [_writable_] reservePda
 * @property [_writable_] mintTo
 * @property [] msolMintAuthority
 * @property [] marinadeFinanceProgram
 * @category Instructions
 * @category MintTokens
 * @category generated
 */
export type MintTokensInstructionAccounts = {
  state: web3.PublicKey;
  token: web3.PublicKey;
  mint: web3.PublicKey;
  authority: web3.PublicKey;
  tokenProgram?: web3.PublicKey;
  vault: web3.PublicKey;
  marinadeState: web3.PublicKey;
  msolMint: web3.PublicKey;
  liqPoolSolLegPda: web3.PublicKey;
  liqPoolMsolLeg: web3.PublicKey;
  liqPoolMsolLegAuthority: web3.PublicKey;
  reservePda: web3.PublicKey;
  mintTo: web3.PublicKey;
  msolMintAuthority: web3.PublicKey;
  rent?: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  marinadeFinanceProgram: web3.PublicKey;
};

export const mintTokensInstructionDiscriminator = [59, 132, 24, 246, 122, 39, 8, 243];

/**
 * Creates a _MintTokens_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category MintTokens
 * @category generated
 */
export function createMintTokensInstruction(
  accounts: MintTokensInstructionAccounts,
  args: MintTokensInstructionArgs,
  programId = new web3.PublicKey('A1tru1e86JujDZ1jP2yaoCHegVj8SvMzSP2yhRFDQgTR'),
) {
  const [data] = mintTokensStruct.serialize({
    instructionDiscriminator: mintTokensInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.state,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.token,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.mint,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.vault,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.marinadeState,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.msolMint,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.liqPoolSolLegPda,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.liqPoolMsolLeg,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.liqPoolMsolLegAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.reservePda,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.mintTo,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.msolMintAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.marinadeFinanceProgram,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
