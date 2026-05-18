# Security Policy

## Supported Versions

Currently, only the latest major release of `bitcraft` is actively supported for security updates.

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

Because `bitcraft` deals with low-level memory operations, zero-copy slicing (`byteslice!`), and lock-free concurrency (`atomic_bitstruct!`), security vulnerabilities are taken very seriously.

If you discover a security issue (e.g., a bounds-checking bypass, undefined behavior in macro expansions, or a data race in the atomic primitives), **please do not report it in a public issue**.

Instead, please send an email directly to the maintainers (or create a private security advisory on GitHub).

When reporting, please include:

* The version of `bitcraft` where the issue was found.
* A description of the vulnerability.
* A minimal reproducible example.

We will acknowledge receipt of your vulnerability report as soon as possible and strive to send you regular updates about our progress. If the vulnerability is verified, we will issue a patch and a security advisory as quickly as we can.
