#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const CARGO_TOML_PATH = path.join(__dirname, '..', 'Cargo.toml');

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

function checkGitStatus() {
  try {
    const output = execSync('git status --porcelain', { encoding: 'utf8' });
    if (output.trim()) {
      console.error('Git working directory is not clean. Please commit or stash your changes first.');
      console.log('Uncommitted changes:');
      console.log(output);
      process.exit(1);
    }
    console.log('✓ Git working directory is clean');
  } catch (error) {
    console.error('Error checking git status:', error.message);
    process.exit(1);
  }
}

function syncVersions() {
  try {
    console.log('Synchronizing versions across package files...');
    execSync('node scripts/sync-versions.js sync', { stdio: 'inherit' });
    console.log('✓ Version synchronization completed');
  } catch (error) {
    console.error('Error synchronizing versions:', error.message);
    process.exit(1);
  }
}

function updateChangelog(version) {
  try {
    console.log(`Updating CHANGELOG.md for version ${version}...`);
    execSync(`git-cliff --tag ${version}`, { stdio: 'inherit' });
    console.log('✓ CHANGELOG.md updated successfully');
  } catch (error) {
    console.error('Error updating CHANGELOG:', error.message);
    console.error('Make sure git-cliff is installed: cargo install git-cliff');
    process.exit(1);
  }
}

function showNextSteps(version) {
  console.log('\n' + '='.repeat(60));
  console.log('RELEASE PREPARATION COMPLETED');
  console.log('='.repeat(60));
  console.log(`Version: ${version}`);
  console.log('\nNext steps:');
  console.log('\n1. Pull latest changes and create branch:');
  console.log('   git pull origin main');
  console.log(`   git checkout -b chore/changelog-update-v${version}`);
  
  console.log('\n2. Commit and push changes:');
  console.log('   git add CHANGELOG.md');
  console.log(`   git commit -m "chore(changelog): Update changelog for v${version}"`);
  console.log(`   git push origin chore/changelog-update-v${version}`);
  
  console.log('\n3. Create Pull Request:');
  console.log(`   gh pr create --title "Update changelog for v${version}" --body "Automated changelog update for version ${version}"`);
  
  console.log('\n4. After PR creation, verify:');
  console.log('   - CI/CD workflows pass successfully');
  console.log('   - No existing CHANGELOG update PRs conflict');
  console.log('   - Review and merge when ready');
  console.log('\n' + '='.repeat(60));
}

function main() {
  console.log('Starting release preparation...\n');

  checkGitStatus();

  const version = extractCargoVersion();
  console.log(`Current version: ${version}\n`);

  syncVersions();
  updateChangelog(version);
  showNextSteps(version);
}

main();
