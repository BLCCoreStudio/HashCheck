# Security Policy

HashCheck is intentionally small and has no telemetry, backend, account system, or network functionality.

## Supported versions

Security fixes are applied to the latest released version.

## Reporting a vulnerability

Please do not publish sensitive vulnerability details in a public issue.

If GitHub private vulnerability reporting is available for this repository, use it. Otherwise, open a minimal public issue asking the maintainers for a private reporting channel without including exploit details, secrets, or private data.

When reporting, include the affected version, operating system, reproduction steps, and the security impact when safe to share privately.

## Security model

HashCheck verifies that a file matches a checksum you provide. A matching checksum does **not** prove that the checksum itself came from a trusted source. Obtain expected hashes over a trusted channel whenever authenticity matters.
