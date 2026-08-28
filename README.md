# HashCheck

**Calculate and verify SHA-256/SHA-512 checksums, plus deterministic SHA-256 release manifests.**

HashCheck is a small, security-focused command-line tool for calculating and verifying checksums. The current development line also integrates deterministic multi-file release-manifest creation and verification, consolidating the useful core direction previously explored by ReleaseSeal.

It is Linux-first, uses streaming file I/O, and has no telemetry, backend, account system, or network functionality.

## Status

**HashCheck v0.1.0 is the current public release.**

A prebuilt Linux x86_64 archive and SHA-256 checksum are available on the [GitHub Releases page](https://github.com/BLCCoreStudio/HashCheck/releases/tag/v0.1.0).

Release-manifest commands described below are part of the current development line and should not be treated as included in the existing v0.1.0 binary until a later release is published.

## Features

Released in v0.1.0:

- SHA-256 by default
- SHA-512 with `--sha512`
- verify a file with `--expect <HASH>`
- case-insensitive expected hexadecimal checksums
- streaming hashing: files are processed in fixed-size chunks instead of being loaded fully into RAM
- clear exit codes for scripts and automation
- no file contents are printed

Current development line additionally includes:

- deterministic SHA-256 manifests for multiple release artifacts
- stable path ordering in generated manifests
- relative manifest entries constrained to the manifest directory
- traversal/absolute-path rejection during manifest verification
- atomic manifest replacement through a temporary file and rename

## Single-file usage

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

## Release manifests — development line

Create a deterministic SHA-256 manifest:

```bash
hashcheck manifest create dist/SHA256SUMS \
  dist/app-linux.tar.gz \
  dist/app-linux.tar.gz.asc
```

Verify it later:

```bash
hashcheck manifest verify dist/SHA256SUMS
```

Manifest entries must remain inside the manifest directory. Absolute paths and parent-directory traversal are rejected during parsing/verification.

This is checksum integrity, not artifact authenticity: a manifest is only as trustworthy as the channel from which you obtained it.

## Relationship to ReleaseSeal

ReleaseSeal began as a focused experiment for deterministic SHA-256 release manifests. That useful core behavior is now being integrated into HashCheck so checksum and checksum-manifest workflows do not need to remain separate products.

The ReleaseSeal repository remains public as a focused development history/reference rather than being deleted or republished. Broader signing, provenance, or SBOM work is **not** claimed by HashCheck unless it is separately implemented and tested.

## Install on Linux x86_64

Download these files from the current [v0.1.0 release](https://github.com/BLCCoreStudio/HashCheck/releases/tag/v0.1.0):

- `hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz`
- `hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256`

Verify and extract:

```bash
sha256sum -c hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
tar -xzf hashcheck-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
./hashcheck --version
```

You can optionally place `hashcheck` somewhere on your `PATH`, such as `~/.local/bin`.

## Exit codes

| Code | Meaning |
| ---: | --- |
| `0` | Hash calculated successfully, expected hash matched, or manifest verified |
| `1` | Expected hash mismatched, or at least one manifest entry mismatched/failed |
| `2` | Usage, manifest-format, or I/O error |

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

A checksum only proves that the bytes you hashed match the expected checksum. It does not prove that the expected checksum or manifest itself is trustworthy. Obtain expected hashes/manifests from a trusted source or channel when authenticity matters.

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

Built by **BLC Core Studio**.
