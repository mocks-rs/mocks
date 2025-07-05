#!/usr/bin/env node

const { spawn } = require('child_process');
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
      } else {
        throw new Error(`Unsupported architecture: ${arch} on ${platform}`);
      }
      break;
    case 'darwin':
      if (arch === 'x64' || arch === 'arm64') {
        platformName = 'darwin-x64';
        packageName = '@mocks-rs/mocks-darwin-x64';
      } else {
        throw new Error(`Unsupported architecture: ${arch} on ${platform}`);
      }
      break;
    case 'win32':
      if (arch === 'x64') {
        platformName = 'win32-x64';
        packageName = '@mocks-rs/mocks-win32-x64';
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
  
  throw new Error(`Binary not found for platform: ${platformName}. Please ensure the appropriate platform package is installed.`);
}

try {
  const binaryPath = getBinaryPath();
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
} catch (error) {
  console.error('Error:', error.message);
  console.error('');
  console.error('Please install the appropriate platform package:');
  console.error('  npm install @mocks-rs/mocks-linux-x64    # Linux x64');
  console.error('  npm install @mocks-rs/mocks-darwin-x64   # macOS');
  console.error('  npm install @mocks-rs/mocks-win32-x64    # Windows x64');
  process.exit(1);
}