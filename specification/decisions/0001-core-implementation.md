# ADR 0001: Initial core implementation

- Status: Accepted for the experimental implementation
- Date: 2026-11-21

## Decision

Start the NDX Core implementation in Rust as a library crate under `core/`. Keep the current JSON Schema as the experimental model reference. The existing Python validator remains a bootstrap validation tool; it is not the normative core implementation.

## Rationale

The core will eventually handle hostile document input and needs explicit resource limits, predictable errors, and a small embeddable API. Rust provides memory-safety properties useful for this role. This decision does not establish a final language mandate for every NDX implementation.

## Scope boundary

The first crate parses the current experimental JSON semantic model only. It does not define the NDX package/container format, guarantee complete schema parity, render content, fetch remote resources, or establish a stable standard.

## Follow-up

Before a package format is designed, publish a threat model covering archive bombs, path traversal, duplicate entries, resource limits, external references, signatures, and parser differentials. Add conformance tests shared across independent implementations.
