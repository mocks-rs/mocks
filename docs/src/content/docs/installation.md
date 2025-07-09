---
title: Installation
description: How to install mocks on your system
---

## Installation Methods

mocks can be installed in several ways. Choose the method that best fits your environment.

### Homebrew (macOS)

If you're using Homebrew on macOS, you can install from the dedicated tap:

```bash
brew install mocks-rs/tap/mocks
```

### Cargo (Rust)

If you have a Rust development environment, you can install directly from cargo:

```bash
cargo install mocks
```

### Binary Download

You can download pre-built binaries for your platform from the GitHub Releases page:

1. Visit [GitHub Releases](https://github.com/mocks-rs/mocks/releases)
2. Select the binary for your platform from the latest version assets
3. Extract the downloaded file and place it in an executable location

## Verification

After installation, you can verify the installation with the following commands:

```bash
mocks --version
```

```bash
mocks --help
```

## System Requirements

- **Supported OS**: Linux, macOS, Windows
- **Minimum Rust Version**: 1.78.0+ (when installing via Cargo)

## Next Steps

Once installation is complete, try creating your first mock API with the [Quick Start](/quick-start/) guide.