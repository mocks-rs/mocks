#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

function getPlatformInfo() {
  const platform = os.platform();
  const arch = os.arch();
  
  let platformName;
  let packageName;
  
  switch (platform) {
    case 'linux':
      if (arch === 'x64') {
        platformName = 'linux-x64';
        packageName = '@mocks-rs/mocks-linux-x64';
      } else if (arch === 'arm64') {
        platformName = 'linux-arm64';
        packageName = '@mocks-rs/mocks-linux-arm64';
      } else {
        throw new Error(`Unsupported architecture: ${arch} on ${platform}`);
      }
      break;
    case 'darwin':
      if (arch === 'x64') {
        platformName = 'darwin-x64';
        packageName = '@mocks-rs/mocks-darwin-x64';
      } else if (arch === 'arm64') {
        platformName = 'darwin-arm64';
        packageName = '@mocks-rs/mocks-darwin-arm64';
      } else {
        throw new Error(`Unsupported architecture: ${arch} on ${platform}`);
      }
      break;
    case 'win32':
      if (arch === 'x64') {
        platformName = 'win32-x64';
        packageName = '@mocks-rs/mocks-win32-x64';
      } else if (arch === 'arm64') {
        platformName = 'win32-arm64';
        packageName = '@mocks-rs/mocks-win32-arm64';
      } else {
        throw new Error(`Unsupported architecture: ${arch} on ${platform}`);
      }
      break;
    default:
      throw new Error(`Unsupported platform: ${platform}`);
  }
  
  return { platformName, packageName };
}

function getBinaryPath() {
  const { platformName, packageName } = getPlatformInfo();
  
  // Try to find the binary in the optional dependency
  const packageDir = path.join(__dirname, '..', 'node_modules', packageName);
  const binaryName = platformName.startsWith('win32') ? 'mocks.exe' : 'mocks';
  const binaryPath = path.join(packageDir, binaryName);
  
  if (fs.existsSync(binaryPath)) {
    return binaryPath;
  }
  
  // Fallback: try to find it in the main package
  const fallbackPath = path.join(__dirname, '..', 'bin', binaryName);
  if (fs.existsSync(fallbackPath)) {
    return fallbackPath;
  }
  
  throw new Error(`Binary not found for platform: ${platformName}`);
}

function createWrapper() {
  const binaryPath = getBinaryPath();
  const wrapperPath = path.join(__dirname, '..', 'bin', 'mocks.js');
  
  // Ensure bin directory exists
  const binDir = path.dirname(wrapperPath);
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }
  
  const wrapperContent = `#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

const binaryPath = ${JSON.stringify(binaryPath)};
const args = process.argv.slice(2);

const child = spawn(binaryPath, args, {
  stdio: 'inherit',
  windowsHide: false
});

child.on('close', (code) => {
  process.exit(code);
});

child.on('error', (err) => {
  console.error('Failed to start mocks:', err.message);
  process.exit(1);
});
`;
  
  fs.writeFileSync(wrapperPath, wrapperContent);
  
  // Make executable on Unix systems
  if (process.platform !== 'win32') {
    fs.chmodSync(wrapperPath, '755');
  }
}

try {
  createWrapper();
  console.log('mocks binary wrapper created successfully');
} catch (error) {
  console.error('Failed to install mocks:', error.message);
  process.exit(1);
}