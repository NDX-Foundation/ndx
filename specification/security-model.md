# NDX Security Model — Working Notes

**Status:** Design notes; not a completed security specification.

## Threat assumptions

A document may be malformed, malicious, oversized, misleading, or crafted to exploit a parser, renderer, converter, or AI-enabled application. A document's author is not automatically trusted.

## Trust boundaries

1. **Input boundary:** bytes and decoded JSON are untrusted.
2. **Parser boundary:** parsers must enforce size, depth, and resource limits.
3. **Rendering boundary:** content must not become executable code or unsafe markup.
4. **Network boundary:** remote resources must not be fetched automatically.
5. **AI boundary:** document text is data, not privileged instructions.
6. **Conversion boundary:** imported content may be incomplete, ambiguous, or lossy.

## Minimum principles for implementations

- Validate before use.
- Fail safely on malformed or unsupported structures.
- Avoid dynamic code execution.
- Escape text when rendering in HTML.
- Restrict resource references to approved package-local identifiers in a future package format.
- Record warnings and unsupported content during conversion.
- Keep secrets and private signing keys outside documents.
- Separate cryptographic verification from identity and trust policy.

## Open questions

- What package container and extraction algorithm should be standardized?
- What are the maximum package, entry, image, and decompressed sizes?
- How should resource paths be normalized and duplicate names rejected?
- Which signature envelope and canonicalization strategy are appropriate?
- How should encryption, key rotation, and recovery be represented?
- How should AI provenance and permissions be expressed without creating unsafe execution semantics?

These notes do not constitute a security audit or guarantee that the current prototype is safe for untrusted production input.