# HashCheck

HashCheck is a small, security-focused command-line tool for calculating and verifying SHA-256 and SHA-512 checksums.

It is Linux-first, uses portable Rust standard-library file I/O, and has no telemetry, backend, account system, or network functionality.

## Features

- SHA-256 by default
- SHA-512 with `--sha512`
- Verify a file with `--expect <HASH>`
- Case-insensitive expected hexadecimal checksums
- Streaming hashing: files are processed in fixed-size chunks instead of being loaded fully into RAM
- Clear exit codes for scripts and automation
- No file contents are printed

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

Requires stable Rust.

```bash
git clone https://github.com/BLCCoreStudio/HashCheck.git
cd HashCheck
cargo build --release
```

The binary will be at `target/release/hashcheck` on Linux/macOS and `target\release\hashcheck.exe` on Windows.

## Development checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

GitHub Actions runs formatting, Clippy with warnings denied, tests on Linux, and tests on Windows/macOS.

## Security note

A checksum only proves that the bytes you hashed match the expected checksum. It does not prove that the expected checksum itself is trustworthy. Obtain expected hashes from a trusted source or channel when authenticity matters.

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).
