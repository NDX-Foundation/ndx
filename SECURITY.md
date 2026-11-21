# Security Policy

## Project status

NDX is experimental and is not currently recommended for processing untrusted documents in production.

## Reporting a vulnerability

Please do not disclose exploitable vulnerability details in a public issue. Use GitHub's private vulnerability reporting feature if enabled. Otherwise, contact the maintainers privately through the NDX Foundation organization.

Include, where possible:
- affected commit, release, or component;
- a clear description and impact assessment;
- reproducible steps or a proof of concept that does not expose real user data;
- suggested mitigations, if known.

## Current security design goals

- Treat every document as untrusted input.
- Do not execute scripts or active content from documents.
- Do not fetch remote resources automatically.
- Apply strict size, nesting, and resource limits in future package readers.
- Prevent path traversal and unsafe archive extraction in any future container format.
- Do not treat document text as trusted AI/system instructions.
- Separate cryptographic integrity, signer identity, and trust decisions.

These are design goals, not a claim that all protections are implemented today.