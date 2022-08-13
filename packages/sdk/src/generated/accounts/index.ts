export * from './Beneficiary';
export * from './State';

import { State } from './State';
import { Beneficiary } from './Beneficiary';

export const accountProviders = { State, Beneficiary };
