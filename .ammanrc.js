// @ts-check
'use strict';
const path = require('path');
const {accountProviders} = require('./packages/sdk/');

const localDeployDir = path.join(__dirname, 'program', 'target', 'deploy');
const ALTRUISM_PROGRAM_ID = require("./packages/sdk/idl/altruism_finance.json").metadata.address;

function localDeployPath(programName) {
    return path.join(localDeployDir, `${programName}.so`);
}

const programs = [
    {
        label: 'altruism_finance',
        programId: ALTRUISM_PROGRAM_ID,
        deployPath: localDeployPath('altruism_finance')
    },
];

const accounts = [
    {
        label: 'Marinade Program',
        accountId:'MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD',
        executable: true,
    },
    {
        label: 'm_state',
        accountId:'8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC',
    },
    {
        label: 'stake withdraw auth',
        accountId:'9eG63CdHjsfhHmobHgLtESGC8GabbmRcaSpHAZrtmhco',
    },
    {
        label: 'msol mint',
        accountId:'mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So',
    },
    {
        label: 'reserve_pda',
        accountId:'Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN',
    },
    {
        label: 'liq_pool_sol_leg_pda',
        accountId:'UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q',
    },
    {
        label: 'liq_pool_msol_leg',
        accountId:'GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE',
    },
    {
        label: 'liq_pool_msol_leg_authority',
        accountId:'EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL',
    },
    {
        label: 'msol_mint_authority',
        accountId:'JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM',
    }
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