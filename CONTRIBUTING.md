# Contributing

Thanks for helping improve HashCheck.

## Development

HashCheck targets stable Rust and intentionally keeps its dependency surface small.

1. Fork the repository and create a focused branch.
2. Make the smallest change that solves the problem.
3. Add or update tests for behavior changes.
4. Run the required checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

5. Open a pull request describing what changed and why.

## Dependencies

New runtime dependencies should be avoided unless they provide clear value that cannot reasonably be implemented with the standard library or existing dependencies. Explain any new dependency in the pull request.

## Security-sensitive changes

Do not include secrets, private data, or exploit details in public issues or pull requests. Follow `SECURITY.md` for vulnerability reports.

## Scope

HashCheck is a checksum calculation and verification CLI. Please keep contributions focused on that purpose and avoid telemetry, background services, network calls, or account systems.
