---
title: Uninstallation
description: How to uninstall mocks from your system
---

## Uninstallation Methods

mocks can be uninstalled using the same package manager that was used for installation.

### Homebrew (macOS)

If you installed mocks using Homebrew, you can uninstall it with:

```bash
brew uninstall mocks
```

You can also remove the tap if you no longer need it:

```bash
brew untap mocks-rs/tap
```

### Cargo (Rust)

If you installed mocks using Cargo, you can uninstall it with:

```bash
cargo uninstall mocks
```

### Binary Installation

If you installed mocks by downloading a binary directly:

1. Remove the mocks binary from your system
2. The binary is typically located in one of these directories:
   - `/usr/local/bin/mocks`
   - `~/.local/bin/mocks`
   - Or wherever you placed it during installation

```bash
# Find the binary location
which mocks

# Remove the binary (adjust path as needed)
rm /usr/local/bin/mocks
```

## Verification

After uninstallation, you can verify that mocks has been removed by running:

```bash
mocks --version
```

This command should return an error indicating that the command is not found, confirming successful uninstallation.

You can also check that the binary is no longer in your PATH:

```bash
which mocks
```

This should return no output if mocks has been successfully removed.

## Cleanup

### Configuration Files

mocks does not create persistent configuration files, so no additional cleanup is typically required.

### Data Files

If you created JSON storage files for your mock APIs, these are not automatically removed during uninstallation. You may want to manually remove them if they are no longer needed:

```bash
# Remove your storage files (adjust paths as needed)
rm storage.json
rm *.debug.json
```

## Getting Help

If you encounter issues during uninstallation:

- Check the [Troubleshooting](/troubleshooting/) guide
- Create an issue on [GitHub](https://github.com/mocks-rs/mocks/issues)
- Include your operating system and installation method when reporting issues