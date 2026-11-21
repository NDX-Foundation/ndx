# NDX Core (experimental)

This crate is the first Rust implementation of the **experimental NDX 0.1 semantic JSON model**.

It currently parses UTF-8 JSON up to 5 MiB, rejects duplicate object keys (including nested keys) and unknown fields, checks model version and block constraints, and enforces unique stable block IDs. It exposes structured error codes.

It does **not** implement a binary/package container, embedded resources, cryptographic signatures, encryption, rendering, or a stable public standard. Do not use it as a security boundary or for untrusted production documents yet.

Run tests from the repository root:

    cargo test --manifest-path core/Cargo.toml

The model remains experimental and must stay aligned with `schemas/document.schema.json` until a versioned compatibility policy is adopted.
