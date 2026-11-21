import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))

from validate import validate_document


class ValidateDocumentTests(unittest.TestCase):
    def test_shared_conformance_fixtures(self):
        fixtures = ROOT / "tests" / "fixtures"
        valid_fixtures = ("valid-minimal.ndx.json", "valid-all-blocks.ndx.json")
        invalid_fixtures = (
            "invalid-duplicate-ids.ndx.json",
            "invalid-created-date.ndx.json",
            "invalid-null-optional.ndx.json",
            "invalid-block-id.ndx.json",
            "invalid-heading-level.ndx.json",
            "invalid-figure-alt.ndx.json",
            "invalid-language.ndx.json",
            "invalid-block-id-trailing-newline.ndx.json",
        )

        for fixture in valid_fixtures:
            with self.subTest(fixture=fixture):
                self.assertEqual(validate_document(fixtures / fixture), [])

        for fixture in invalid_fixtures:
            with self.subTest(fixture=fixture):
                self.assertTrue(validate_document(fixtures / fixture))

    def test_example_document_is_valid(self):
        self.assertEqual(validate_document(ROOT / "examples" / "hello.ndx.json"), [])

    def test_unknown_block_type_is_rejected(self):
        document = {
            "format": "ndx",
            "modelVersion": "0.1",
            "metadata": {"title": "Bad example", "language": "en"},
            "content": [{"id": "x", "type": "script", "code": "alert(1)"}],
        }
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "invalid.json"
            path.write_text(json.dumps(document), encoding="utf-8")
            self.assertTrue(validate_document(path))

    def test_duplicate_block_ids_are_rejected(self):
        document = {
            "format": "ndx",
            "modelVersion": "0.1",
            "metadata": {"title": "Duplicate IDs", "language": "en"},
            "content": [
                {"id": "same", "type": "paragraph", "text": "one"},
                {"id": "same", "type": "paragraph", "text": "two"},
            ],
        }
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "duplicate.ndx.json"
            path.write_text(json.dumps(document), encoding="utf-8")
            errors = validate_document(path)
        self.assertTrue(any("duplicate block id" in error for error in errors))

    def test_duplicate_json_object_keys_are_rejected(self):
        raw = '{"format":"ndx","format":"other"}'
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "duplicate-key.ndx.json"
            path.write_text(raw, encoding="utf-8")
            with self.assertRaises(ValueError):
                validate_document(path)

    def test_oversized_document_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "large.ndx.json"
            path.write_bytes(b" " * (5 * 1024 * 1024 + 1))
            with self.assertRaises(ValueError):
                validate_document(path)


if __name__ == "__main__":
    unittest.main()
