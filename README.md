# HashCheck

**Calculate and verify SHA-256 and SHA-512 checksums.**

HashCheck is a small, security-focused command-line tool for calculating and verifying SHA-256 and SHA-512 checksums.

It is Linux-first, uses portable Rust standard-library file I/O, and has no telemetry, backend, account system, or network functionality.

## Status

**HashCheck v0.1.0 is available as the first public release.**

A prebuilt Linux x86_64 archive and SHA-256 checksum are available on the [GitHub Releases page](https://github.com/BLCCoreStudio/HashCheck/releases/tag/v0.1.0).

## Features

- SHA-256 by default
- SHA-512 with `--sha512`
- Verify a file with `--expect <HASH>`
- Case-insensitive expected hexadecimal checksums
- Streaming hashing: files are processed in fixed-size chunks instead of being loaded fully into RAM
- Clear exit codes for scripts and automation
- No file contents are printed

## Install on Linux x86_64

Download these files from the [v0.1.0 release](https://github.com/BLCCoreStudio/HashCheck/releases/tag/v0.1.0):

- `hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz`
- `hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256`

Verify and extract:

```bash
sha256sum -c hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
tar -xzf hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
./hashcheck --version
```

You can optionally place `hashcheck` somewhere on your `PATH`, such as `~/.local/bin`.

## Usage

Calculate SHA-256:

```bash
hashcheck file.iso
```

Calculate SHA-512:

```bash
hashcheck --sha512 file.iso
```

Verify SHA-256:

```bash
hashcheck file.iso --expect <HASH>
```

Verify SHA-512:

```bash
hashcheck --sha512 file.iso --expect <HASH>
```

Expected hashes may use uppercase or lowercase hexadecimal characters.

### Exit codes

| Code | Meaning |
| ---: | --- |
| `0` | Hash calculated successfully, or expected hash matched |
| `1` | Expected hash did not match |
| `2` | Usage or I/O error |

## Build from source

Requires Rust 1.74 or newer.

```bash
git clone https://github.com/BLCCoreStudio/HashCheck.git
cd HashCheck
cargo build --release --locked
./target/release/hashcheck --version
```

The binary will be at `target/release/hashcheck` on Linux/macOS and `target\release\hashcheck.exe` on Windows.

## Development checks

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
```

GitHub Actions runs formatting, Clippy with warnings denied, tests on Linux, and tests on Windows/macOS.

## Security note

A checksum only proves that the bytes you hashed match the expected checksum. It does not prove that the expected checksum itself is trustworthy. Obtain expected hashes from a trusted source or channel when authenticity matters.

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

Built by **BLC Core Studio**.