"""Validate an experimental NDX JSON document against the v0.1 schema."""
from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "document.schema.json"
MAX_DOCUMENT_BYTES = 5 * 1024 * 1024


class DuplicateKeyError(ValueError):
    """Raised when a JSON object contains a duplicate key."""


def _unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise DuplicateKeyError(f"duplicate JSON object key: {key!r}")
        result[key] = value
    return result


def _load_json(path: Path) -> Any:
    raw = path.read_bytes()
    if len(raw) > MAX_DOCUMENT_BYTES:
        raise ValueError(f"document exceeds {MAX_DOCUMENT_BYTES} byte limit")
    text = raw.decode("utf-8")
    return json.loads(text, object_pairs_hook=_unique_object)


def validate_document(document_path: Path) -> list[str]:
    schema = _load_json(SCHEMA_PATH)
    document = _load_json(document_path)

    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    errors = sorted(
        validator.iter_errors(document),
        key=lambda error: (list(map(str, error.absolute_path)), error.message),
    )
    messages = [
        f"{'.'.join(map(str, error.absolute_path)) or '<root>'}: {error.message}"
        for error in errors
    ]

    # Keep date-time acceptance consistent across implementations even when
    # a JSON Schema format checker is configured with reduced optional support.
    if isinstance(document, dict):
        metadata = document.get("metadata")
        if isinstance(metadata, dict) and isinstance(metadata.get("created"), str):
            created = metadata["created"]
            rfc3339_shape = re.fullmatch(
                r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})",
                created,
            )
            try:
                if not rfc3339_shape:
                    raise ValueError("not RFC 3339")
                datetime.fromisoformat(created.replace("Z", "+00:00"))
            except ValueError:
                messages.append("metadata.created: must be an RFC 3339 date-time")

    if isinstance(document, dict):
        blocks = document.get("content", [])
        if isinstance(blocks, list):
            seen: set[str] = set()
            for index, block in enumerate(blocks):
                if not isinstance(block, dict):
                    continue
                block_id = block.get("id")
                if isinstance(block_id, str):
                    if block_id in seen:
                        messages.append(f"content.{index}.id: duplicate block id {block_id!r}")
                    seen.add(block_id)

    return messages


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate an experimental NDX JSON document.")
    parser.add_argument("document", type=Path, help="Path to an NDX JSON document")
    args = parser.parse_args()

    try:
        errors = validate_document(args.document)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, DuplicateKeyError, ValueError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2

    if errors:
        for error in errors:
            print(f"INVALID: {error}")
        return 1

    print(f"VALID (experimental NDX model 0.1): {args.document}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
