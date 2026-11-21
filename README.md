# NDX — Next-Generation Document eXchange

**A universal document format for humans and intelligent machines.**

NDX is an open-source project exploring a document format in which semantic structure, accessibility, provenance, and machine interpretation are explicit parts of the document model.

> **Development status:** Experimental, early-stage project. The v0.1 model in this repository is a working proposal, not a stable standard. Do not rely on it for long-term archival or production interchange.

## Why NDX?

Many document workflows require software to infer meaning from layout, typography, or extracted text. NDX explores a semantic-first representation so that documents can be more reliably understood by people and software.

Project goals:

- Represent meaning and structure explicitly.
- Preserve stable identifiers for document elements.
- Make accessibility and reading order first-class concerns.
- Support traceable evidence and provenance for machine-assisted interpretation.
- Treat document content as untrusted data and use secure defaults.
- Report conversion limitations honestly rather than claiming perfect fidelity.

## Repository contents

- `specification/` — evolving design and format specification.
- `schemas/` — machine-readable JSON Schemas.
- `examples/` — example semantic documents.
- `tools/` — validation utilities.
- `tests/` — automated tests and fixtures.
- `.github/workflows/` — continuous integration.

## Quick start

Requires Python 3.11 or later.

```powershell
py -m venv .venv
.\.venv\Scripts\Activate.ps1
python -m pip install -r tools/requirements.txt
python tools/validate.py examples/hello.ndx.json
python -m unittest discover -s tests -v
```

## Current scope

The initial implementation validates a UTF-8 JSON semantic document model. A packaged `.ndx` container, rendering model, digital signatures, encryption, and format converters are not yet finalized or implemented.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). For security issues, see [SECURITY.md](SECURITY.md).

## Licensing

The initial code and specification licensing policy is still under review. No license is granted by this repository until an explicit license file and policy are added.