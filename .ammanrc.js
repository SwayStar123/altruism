// @ts-check
'use strict';
const path = require('path');
const { accountProviders } = require('./packages/api/');

const localDeployDir = path.join(__dirname, 'program', 'target', 'deploy');
const ALTRUISM_PROGRAM_ID = require('./packages/api/idl/altruism_finance.json').metadata.address;

function localDeployPath(programName) {
  return path.join(localDeployDir, `${programName}.so`);
}

const programs = [
  {
    label: 'altruism_finance',
    programId: ALTRUISM_PROGRAM_ID,
    deployPath: localDeployPath('altruism_finance'),
  },
];

const accounts = [
  {
    label: 'Marinade Program',
    accountId: 'MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD',
    executable: true,
  },
  {
    label: 'm_state',
    accountId: '8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC',
  },
  {
    label: 'msol_mint',
    accountId: 'mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So',
  },
  {
    label: 'reserve_pda',
    accountId: 'Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN',
  },
  {
    label: 'liq_pool_sol_leg_pda',
    accountId: 'UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q',
  },
];

const validator = {
  programs,
  accounts,
  accountsCluster: 'https://api.metaplex.solana.com',
  verifyFees: false,
  limitLedgerSize: 10000000,
};

module.exports = {
  validator,
  relay: {
    accountProviders,
  },
};
