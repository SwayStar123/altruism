export * from './AltruismState';
export * from './Beneficiary';

import { Beneficiary } from './Beneficiary';
import { AltruismState } from './AltruismState';

export const accountProviders = { Beneficiary, AltruismState };
