# NDX Core (experimental)

This crate is the first Rust implementation of the **experimental NDX 0.1 semantic JSON model**.

It currently:
- Parses UTF-8 JSON documents up to 5 MiB.
- Rejects duplicate JSON object keys, including nested keys.
- Rejects unknown model fields and explicit null values for optional string fields.
- Checks the model version, required metadata, RFC 3339 creation timestamps, block constraints, and unique stable block IDs.
- Exposes structured parser error codes.

## Shared conformance fixtures

The language-neutral fixtures live in `tests/fixtures/`. Both the Rust core tests and Python validator tests consume the same valid and invalid documents. This is an initial parity check, not yet a complete generated conformance suite.

Run tests from the repository root:

    cargo test --manifest-path core/Cargo.toml
    python -m unittest discover -s tests -v

## Scope and limitations

It does **not** implement a binary/package container, embedded resources, cryptographic signatures, encryption, rendering, or a stable public standard. Do not use it as a security boundary or for untrusted production documents yet.

The model remains experimental and must stay aligned with `schemas/document.schema.json` until a versioned compatibility policy is adopted.
