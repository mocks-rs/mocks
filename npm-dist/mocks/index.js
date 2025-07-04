// This is the main entry point for the @mocks-rs/mocks package
// The actual binary is executed through the bin/mocks.js wrapper

module.exports = {
  name: '@mocks-rs/mocks',
  version: '0.4.0',
  description: 'Get a mock REST APIs with zero coding within seconds.',
  binaryPath: require('./lib/install.js')
};