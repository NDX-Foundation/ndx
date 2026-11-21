# NDX Core (experimental)

This crate is the first Rust implementation of the **experimental NDX 0.1 semantic JSON model**.

It parses UTF-8 JSON documents up to 5 MiB; rejects duplicate JSON keys, unknown fields, and explicit nulls for optional string fields; checks model version, metadata, RFC 3339 timestamps, block constraints, and unique block IDs; and exposes structured parser error codes. It also provides the `ndx-validate` CLI.

## Validate a document

From the repository root:

    cargo run --manifest-path core/Cargo.toml --bin ndx-validate -- examples/hello.ndx.json

Exit codes: `0` accepted, `1` invalid document, `2` usage or file I/O error.

## Shared conformance fixtures

Language-neutral fixtures live in `tests/fixtures/`. Rust core tests and Python validator tests consume the same valid documents (minimal and all supported block types) and invalid documents (duplicate IDs, invalid timestamps, explicit null optionals, invalid IDs, heading levels, figure alt text, and language length). This is an initial parity check, not a complete generated conformance suite.

Run tests:

    cargo test --manifest-path core/Cargo.toml
    python -m unittest discover -s tests -v

## Scope and limitations

It does **not** implement a binary/package container, embedded resources, cryptographic signatures, encryption, rendering, or a stable public standard. Do not use it as a security boundary or for untrusted production documents yet. The model remains experimental and must stay aligned with `schemas/document.schema.json`.
