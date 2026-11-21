//! Experimental NDX 0.1 semantic JSON parser.
//! This crate does not define or read a binary/container format.

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use std::fmt;

pub const MAX_DOCUMENT_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NdxError { pub code: &'static str, pub message: String }
impl NdxError {
    fn new(code: &'static str, message: impl Into<String>) -> Self { Self { code, message: message.into() } }
}
impl fmt::Display for NdxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}: {}", self.code, self.message) }
}
impl std::error::Error for NdxError {}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Document {
    pub format: String,
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    pub metadata: Metadata,
    pub content: Vec<Block>,
}
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub title: String,
    pub language: String,
    pub creator: Option<String>,
    pub created: Option<String>,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Block {
    Heading { id: String, level: u8, text: String },
    Paragraph { id: String, text: String },
    List { id: String, ordered: bool, items: Vec<String> },
    Figure { id: String, alt: String, caption: Option<String>, resource: Option<String> },
}

/// Parse one experimental NDX 0.1 JSON document.
pub fn parse_document(input: &[u8]) -> Result<Document, NdxError> {
    if input.len() > MAX_DOCUMENT_BYTES {
        return Err(NdxError::new("NDX_E_SIZE_LIMIT", "document exceeds 5 MiB"));
    }
    let text = std::str::from_utf8(input)
        .map_err(|_| NdxError::new("NDX_E_UTF8", "document is not valid UTF-8"))?;
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let value = NoDuplicateValue::deserialize(&mut deserializer)
        .map_err(|e| NdxError::new("NDX_E_JSON", e.to_string()))?;
    deserializer.end().map_err(|e| NdxError::new("NDX_E_JSON", e.to_string()))?;
    reject_null_optionals(&value.0)?;
    let document: Document = serde_json::from_value(value.0)
        .map_err(|e| NdxError::new("NDX_E_MODEL", e.to_string()))?;
    validate(&document)?;
    Ok(document)
}

fn reject_null_optionals(value: &Value) -> Result<(), NdxError> {
    let Some(root) = value.as_object() else { return Ok(()); };
    if let Some(metadata) = root.get("metadata").and_then(Value::as_object) {
        for key in ["creator", "created", "description"] {
            if metadata.get(key).is_some_and(Value::is_null) {
                return Err(NdxError::new("NDX_E_MODEL", format!("metadata.{key} must be omitted or a string, not null")));
            }
        }
    }
    if let Some(blocks) = root.get("content").and_then(Value::as_array) {
        for (index, block) in blocks.iter().enumerate() {
            if let Some(object) = block.as_object() {
                for key in ["caption", "resource"] {
                    if object.get(key).is_some_and(Value::is_null) {
                        return Err(NdxError::new("NDX_E_MODEL", format!("content.{index}.{key} must be omitted or a string, not null")));
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate(doc: &Document) -> Result<(), NdxError> {
    if doc.format != "ndx" { return Err(NdxError::new("NDX_E_FORMAT", "format must be 'ndx'")); }
    if doc.model_version != "0.1" { return Err(NdxError::new("NDX_E_VERSION", "modelVersion must be '0.1'")); }
    if doc.metadata.title.is_empty() || doc.metadata.title.chars().count() > 500 {
        return Err(NdxError::new("NDX_E_METADATA", "metadata.title must contain 1..500 characters"));
    }
    if doc.metadata.creator.as_ref().is_some_and(|s| s.chars().count() > 500)
        || doc.metadata.description.as_ref().is_some_and(|s| s.chars().count() > 5_000)
    {
        return Err(NdxError::new("NDX_E_METADATA", "metadata creator or description exceeds its schema limit"));
    }
    if let Some(created) = &doc.metadata.created {
        if time::OffsetDateTime::parse(created, &time::format_description::well_known::Rfc3339).is_err() {
            return Err(NdxError::new("NDX_E_METADATA", "metadata.created must be an RFC 3339 date-time"));
        }
    }
    if !(2..=35).contains(&doc.metadata.language.chars().count()) {
        return Err(NdxError::new("NDX_E_METADATA", "metadata.language must contain 2..35 characters"));
    }
    if doc.content.is_empty() { return Err(NdxError::new("NDX_E_CONTENT", "content must contain at least one block")); }
    let mut ids = HashSet::new();
    for block in &doc.content {
        let (id, valid) = match block {
            Block::Heading { id, level, text } => (id, (1..=6).contains(level) && !text.is_empty() && text.chars().count() <= 10_000),
            Block::Paragraph { id, text } => (id, text.chars().count() <= 100_000),
            Block::List { id, items, .. } => (id, !items.is_empty() && items.iter().all(|s| s.chars().count() <= 10_000)),
            Block::Figure { id, alt, caption, resource } => (id, !alt.is_empty() && alt.chars().count() <= 2_000 && caption.as_ref().map_or(true, |s| s.chars().count() <= 5_000) && resource.as_ref().map_or(true, |s| s.chars().count() <= 1_024)),
        };
        if !valid { return Err(NdxError::new("NDX_E_BLOCK", format!("block '{id}' violates its field constraints"))); }
        if !valid_id(id) { return Err(NdxError::new("NDX_E_BLOCK_ID", format!("invalid block id '{id}'"))); }
        if !ids.insert(id) { return Err(NdxError::new("NDX_E_DUPLICATE_ID", format!("duplicate block id '{id}'"))); }
    }
    Ok(())
}
fn valid_id(id: &str) -> bool {
    matches!(id.chars().next(), Some(c) if c.is_ascii_alphanumeric())
        && id.len() <= 128
        && id.bytes().all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
}

// Recursively rejects repeated object keys before conversion to serde_json::Value.
#[derive(Debug)]
struct NoDuplicateValue(Value);
impl<'de> Deserialize<'de> for NoDuplicateValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NoDuplicateValue;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("a JSON value without duplicate object keys") }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value,E> { Ok(NoDuplicateValue(Value::Bool(v))) }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value,E> { Ok(NoDuplicateValue(Value::Number(v.into()))) }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value,E> { Ok(NoDuplicateValue(Value::Number(v.into()))) }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value,E> {
                serde_json::Number::from_f64(v).map(|n| NoDuplicateValue(Value::Number(n))).ok_or_else(|| E::custom("invalid number"))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value,E> { Ok(NoDuplicateValue(Value::String(v.to_owned()))) }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value,E> { Ok(NoDuplicateValue(Value::String(v))) }
            fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value,E> { Ok(NoDuplicateValue(Value::Null)) }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value,E> { Ok(NoDuplicateValue(Value::Null)) }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value,A::Error> {
                let mut out = Vec::new();
                while let Some(v) = seq.next_element::<NoDuplicateValue>()? { out.push(v.0); }
                Ok(NoDuplicateValue(Value::Array(out)))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value,A::Error> {
                let mut out = serde_json::Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if out.contains_key(&key) { return Err(serde::de::Error::custom(format!("duplicate object key '{key}'"))); }
                    out.insert(key, map.next_value::<NoDuplicateValue>()?.0);
                }
                Ok(NoDuplicateValue(Value::Object(out)))
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID: &[u8] = br#"{"format":"ndx","modelVersion":"0.1","metadata":{"title":"Hello","language":"en"},"content":[{"id":"p1","type":"paragraph","text":"Hello world"}]}"#;
    #[test] fn parses_valid_document() { assert_eq!(parse_document(VALID).unwrap().metadata.title, "Hello"); }
    #[test] fn rejects_duplicate_keys() {
        let input = br#"{"format":"ndx","format":"ndx","modelVersion":"0.1","metadata":{"title":"x","language":"en"},"content":[{"id":"p","type":"paragraph","text":"x"}]}"#;
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_JSON");
    }
    #[test] fn rejects_duplicate_ids() {
        let input = br#"{"format":"ndx","modelVersion":"0.1","metadata":{"title":"x","language":"en"},"content":[{"id":"p","type":"paragraph","text":"x"},{"id":"p","type":"paragraph","text":"y"}]}"#;
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_DUPLICATE_ID");
    }
    #[test] fn rejects_unknown_fields() {
        let input = br#"{"format":"ndx","modelVersion":"0.1","metadata":{"title":"x","language":"en","surprise":true},"content":[{"id":"p","type":"paragraph","text":"x"}]}"#;
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_MODEL");
    }
    #[test] fn rejects_invalid_utf8() { assert_eq!(parse_document(&[0xff]).unwrap_err().code, "NDX_E_UTF8"); }
    #[test] fn rejects_oversized_input() { assert_eq!(parse_document(&vec![b' '; MAX_DOCUMENT_BYTES + 1]).unwrap_err().code, "NDX_E_SIZE_LIMIT"); }
    #[test] fn shared_valid_fixture_parses() {
        let input = include_bytes!("../../tests/fixtures/valid-minimal.ndx.json");
        assert!(parse_document(input).is_ok());
    }
    #[test] fn shared_duplicate_id_fixture_rejected() {
        let input = include_bytes!("../../tests/fixtures/invalid-duplicate-ids.ndx.json");
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_DUPLICATE_ID");
    }
    #[test] fn shared_invalid_date_fixture_rejected() {
        let input = include_bytes!("../../tests/fixtures/invalid-created-date.ndx.json");
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_METADATA");
    }
    #[test] fn shared_null_optional_fixture_rejected() {
        let input = include_bytes!("../../tests/fixtures/invalid-null-optional.ndx.json");
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_MODEL");
    }
    #[test] fn rejects_invalid_created_timestamp() {
        let input = br#"{"format":"ndx","modelVersion":"0.1","metadata":{"title":"x","language":"en","created":"yesterday"},"content":[{"id":"p","type":"paragraph","text":"x"}]}"#;
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_METADATA");
    }
    #[test] fn rejects_oversized_metadata_description() {
        let description = "x".repeat(5_001);
        let json = format!(r#"{{"format":"ndx","modelVersion":"0.1","metadata":{{"title":"x","language":"en","description":"{}"}},"content":[{{"id":"p","type":"paragraph","text":"x"}}]}}"#, description);
        assert_eq!(parse_document(json.as_bytes()).unwrap_err().code, "NDX_E_METADATA");
    }
    #[test] fn rejects_empty_content() {
        let input = br#"{"format":"ndx","modelVersion":"0.1","metadata":{"title":"x","language":"en"},"content":[]}"#;
        assert_eq!(parse_document(input).unwrap_err().code, "NDX_E_CONTENT");
    }
    #[test] fn shared_all_blocks_fixture_parses() {
        let input = include_bytes!("../../tests/fixtures/valid-all-blocks.ndx.json");
        let document = parse_document(input).unwrap();
        assert_eq!(document.content.len(), 4);
    }
    #[test] fn shared_invalid_field_fixtures_are_rejected() {
        for input in [
            include_bytes!("../../tests/fixtures/invalid-block-id.ndx.json").as_slice(),
            include_bytes!("../../tests/fixtures/invalid-heading-level.ndx.json").as_slice(),
            include_bytes!("../../tests/fixtures/invalid-figure-alt.ndx.json").as_slice(),
            include_bytes!("../../tests/fixtures/invalid-language.ndx.json").as_slice(),
            include_bytes!("../../tests/fixtures/invalid-block-id-trailing-newline.ndx.json").as_slice(),
        ] {
            assert!(parse_document(input).is_err());
        }
    }
}
