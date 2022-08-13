const path = require('path');
const programDir = path.join(__dirname, 'program');
const idlDir = path.join(__dirname, 'packages', 'api', 'idl');
const sdkDir = path.join(__dirname, 'packages', 'api', 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'altruism_finance',
  programId: 'A1tru1e86JujDZ1jP2yaoCHegVj8SvMzSP2yhRFDQgTR',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
