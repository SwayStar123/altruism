import { PublicKey } from '@solana/web3.js';
import { PROGRAM_ID } from './generated';

export const findMsolVaultPda = (): PublicKey => {
  let seeds = Buffer.from('msol_vault');
  return PublicKey.findProgramAddressSync([seeds], PROGRAM_ID)[0];
};

export const findGlobalSolVaultPda = (): PublicKey => {
  let seeds = Buffer.from('global_sol_vault');
  return PublicKey.findProgramAddressSync([seeds], PROGRAM_ID)[0];
};

export const findBeneficiaryPda = (authority?: PublicKey): PublicKey => {
  let seedsArray = [Buffer.from('beneficiary')];
  if (authority) {
    seedsArray.push(authority.toBuffer());
  }
  return PublicKey.findProgramAddressSync(seedsArray, PROGRAM_ID)[0];
};
