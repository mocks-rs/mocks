// This is the main entry point for the @mocks-rs/mocks package
// The actual binary is executed through the bin/mocks.js wrapper

const packageJson = require('./package.json');

module.exports = {
  name: packageJson.name,
  version: packageJson.version,
  description: packageJson.description,
  binaryPath: require('./lib/install.js')
};
