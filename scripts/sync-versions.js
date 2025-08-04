#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const CARGO_TOML_PATH = path.join(__dirname, '..', 'Cargo.toml');
const PACKAGE_JSON_PATHS = [
  path.join(__dirname, '..', 'packages', '@mocks-rs', 'mocks', 'package.json'),
  path.join(__dirname, '..', 'packages', '@mocks-rs', 'mocks-linux-arm64', 'package.json'),
  path.join(__dirname, '..', 'packages', '@mocks-rs', 'mocks-linux-x64', 'package.json'),
  path.join(__dirname, '..', 'packages', '@mocks-rs', 'mocks-darwin-arm64', 'package.json'),
  path.join(__dirname, '..', 'packages', '@mocks-rs', 'mocks-darwin-x64', 'package.json'),
  path.join(__dirname, '..', 'packages', '@mocks-rs', 'mocks-win32-arm64', 'package.json'),
  path.join(__dirname, '..', 'packages', '@mocks-rs', 'mocks-win32-x64', 'package.json')
];

function getRelativePackagePath(packagePath) {
  return path.join(path.basename(path.dirname(packagePath)), path.basename(packagePath));
}

function extractCargoVersion() {
  try {
    const cargoToml = fs.readFileSync(CARGO_TOML_PATH, 'utf8');
    const versionMatch = cargoToml.match(/^version\s*=\s*"([^"]+)"/m);
    
    if (!versionMatch) {
      throw new Error('Could not find version in Cargo.toml');
    }
    
    return versionMatch[1];
  } catch (error) {
    console.error('Error reading Cargo.toml:', error.message);
    process.exit(1);
  }
}

function updatePackageJson(packagePath, version) {
  try {
    const packageJson = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
    const oldVersion = packageJson.version;
    
    packageJson.version = version;
    
    // Update optionalDependencies if they exist
    if (packageJson.optionalDependencies) {
      for (const dep in packageJson.optionalDependencies) {
        if (dep.startsWith('@mocks-rs/mocks-')) {
          packageJson.optionalDependencies[dep] = version;
        }
      }
    }
    
    fs.writeFileSync(packagePath, JSON.stringify(packageJson, null, 2) + '\n');
    const relativePath = getRelativePackagePath(packagePath);
    console.log(`Updated ${relativePath}: ${oldVersion} → ${version}`);
    
    return oldVersion !== version;
  } catch (error) {
    console.error(`Error updating ${packagePath}:`, error.message);
    process.exit(1);
  }
}

function checkVersionConsistency() {
  const cargoVersion = extractCargoVersion();
  console.log(`Cargo.toml version: ${cargoVersion}`);
  
  let hasChanges = false;
  
  for (const packagePath of PACKAGE_JSON_PATHS) {
    if (fs.existsSync(packagePath)) {
      const changed = updatePackageJson(packagePath, cargoVersion);
      hasChanges = hasChanges || changed;
    } else {
      console.warn(`Warning: ${packagePath} does not exist`);
    }
  }
  
  if (hasChanges) {
    console.log('\nVersion synchronization completed.');
    console.log('All package.json files have been updated to match Cargo.toml version.');
  } else {
    console.log('\nAll versions are already synchronized.');
  }
  
  return hasChanges;
}

function verifyVersionConsistency() {
  const cargoVersion = extractCargoVersion();
  console.log(`Checking version consistency. Expected version: ${cargoVersion}`);
  
  let allConsistent = true;
  
  for (const packagePath of PACKAGE_JSON_PATHS) {
    if (fs.existsSync(packagePath)) {
      try {
        const packageJson = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
        const packageVersion = packageJson.version;
        
        const relativePath = getRelativePackagePath(packagePath);
        if (packageVersion !== cargoVersion) {
          console.error(`[ERROR] Version mismatch in ${relativePath}: ${packageVersion} (expected: ${cargoVersion})`);
          allConsistent = false;
        } else {
          console.log(`[OK] ${relativePath}: ${packageVersion}`);
        }
        
        // Check optionalDependencies
        if (packageJson.optionalDependencies) {
          for (const [dep, depVersion] of Object.entries(packageJson.optionalDependencies)) {
            if (dep.startsWith('@mocks-rs/mocks-') && depVersion !== cargoVersion) {
              console.error(`[ERROR] Version mismatch in ${path.basename(packagePath)} optionalDependency ${dep}: ${depVersion} (expected: ${cargoVersion})`);
              allConsistent = false;
            }
          }
        }
      } catch (error) {
        console.error(`Error reading ${packagePath}:`, error.message);
        allConsistent = false;
      }
    } else {
      console.warn(`Warning: ${packagePath} does not exist`);
    }
  }
  
  if (allConsistent) {
    console.log('\n[OK] All versions are consistent!');
    return true;
  } else {
    console.log('\n[ERROR] Version inconsistencies detected!');
    return false;
  }
}

// Export functions for reuse
module.exports = {
  extractCargoVersion,
  checkVersionConsistency,
  verifyVersionConsistency
};

// Main logic (only run if this script is executed directly)
if (require.main === module) {
  const command = process.argv[2];

  switch (command) {
    case 'sync': {
      checkVersionConsistency();
      break;
    }
    case 'check': {
      const isConsistent = verifyVersionConsistency();
      process.exit(isConsistent ? 0 : 1);
      break;
    }
    default: {
      console.log('Usage:');
      console.log('  node sync-versions.js sync   - Synchronize all package.json versions with Cargo.toml');
      console.log('  node sync-versions.js check  - Check version consistency (exits with error if inconsistent)');
      process.exit(1);
    }
  }
}
