import { PublicKey } from '@solana/web3.js';
import { findBeneficiaryPda, findGlobalSolVaultPda, findMsolVaultPda } from './pda-helpers';

export const M_STATE_PUBKEY: PublicKey = new PublicKey(
  '8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC',
);

export const MSOL_VAULT_PUBKEY: PublicKey = findMsolVaultPda();

export const GLOBAL_SOL_VAULT_PUBKEY: PublicKey = findGlobalSolVaultPda();

export const PROGRAM_BENEFICIARY_PUBKEY: PublicKey = findBeneficiaryPda();

export const RESERVE_PDA_PUBKEY: PublicKey = new PublicKey(
  'Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN',
);

export const MARINADE_FINANCE_PUBKEY: PublicKey = new PublicKey(
  'MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD',
);

export const MSOL_MINT_PUBKEY: PublicKey = new PublicKey(
  'mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So',
);

export const SOL_LEG_PDA_PUBKEY: PublicKey = new PublicKey(
  'UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q',
);

export const MSOL_LEG_PUBKEY: PublicKey = new PublicKey(
  '7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE',
);

export const MSOL_LEG_AUTHORITY_PUBKEY: PublicKey = new PublicKey(
  'EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL',
);

export const MSOL_MINT_AUTHORITY_PUBKEY: PublicKey = new PublicKey(
  '3JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM',
);
