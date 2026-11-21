# NDX Format Overview

**Working draft:** v0.1  
**Status:** Experimental and non-normative  
**Last updated:** 2026-11-21

## 1. Purpose

NDX explores a document representation in which semantic meaning and relationships are stored explicitly rather than inferred only from visual layout or extracted text.

## 2. Design principles

1. **Semantic first:** headings, paragraphs, lists, tables, figures, citations, and other structures have explicit types.
2. **Stable references:** elements have identifiers so annotations and evidence can point to precise content.
3. **Human-readable source:** the initial model uses UTF-8 JSON for inspectability and tooling.
4. **Accessible by design:** reading order and meaningful alternatives for non-text content must be representable.
5. **Evidence and provenance:** machine-generated claims should be traceable to source elements; generated content must be distinguishable from authored content.
6. **Secure by default:** documents are data, never executable instructions. External resources must not load automatically.
7. **Honest conversion:** importers must report unsupported or lossy content.

## 3. Initial document model

A v0.1 document contains:
- a format identifier;
- a model version;
- basic metadata;
- an ordered sequence of typed content blocks;
- stable identifiers for blocks.

The JSON Schema in [schemas/document.schema.json](../schemas/document.schema.json) defines the current validation boundary.

## 4. Initial block types

The first draft supports:
- `heading`
- `paragraph`
- `list`
- `figure`

Tables, charts, footnotes, references, annotations, formulas, richer media, and layout constraints are future design work. Their absence must not be interpreted as support.

## 5. Semantic model versus package format

The current example is a JSON representation of the semantic model. The `.ndx` extension is a project goal, not yet a finalized container specification.

ZIP packaging, canonical serialization, resource naming, duplicate-entry handling, decompression limits, signatures, encryption, and media-type rules require separate threat modeling and interoperability tests before standardization.

## 6. Security requirements under investigation

- No script execution or active content by default.
- No automatic network access when opening a document.
- Strict validation of paths and resource references in any future package.
- Explicit resource and decompression limits.
- Untrusted document text must never be treated as trusted AI/system instructions.
- Cryptographic integrity, signer identity, and trust decisions must remain distinct.

## 7. Versioning and compatibility

The `modelVersion` field identifies the semantic model version. Readers should reject unsupported major versions and preserve unknown optional fields only when doing so is safe. Exact compatibility and extension rules remain to be specified.

## 8. Rendering

NDX has a long-term goal of supporting both responsive/reflowable and page-oriented presentation. This draft defines neither rendering mode nor pixel-identical output guarantees.

## 9. Non-goals for v0.1

This draft does not promise:
- lossless conversion from PDF or office formats;
- pixel-identical rendering;
- a finalized ZIP container;
- digital signatures or encryption;
- a production-ready reader, writer, or converter.

## 10. Change process

Substantive proposals should be discussed in GitHub issues or pull requests. Changes affecting meaning, security, or interoperability require examples and tests before acceptance.