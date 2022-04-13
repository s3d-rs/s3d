//! This module provides structures to parse and make sense of
//! a smithy model which is loaded in JSON AST format.
//!
//! Smithy specs:
//! - https://awslabs.github.io/smithy/1.0/spec/index.html
//! - https://awslabs.github.io/smithy/1.0/spec/core/json-ast.html

use proc_macro2::Ident;
use quote::format_ident;
use serde_json::{Map, Value};
use std::{collections::HashMap, fs::File, path::Path};

/// SmithyModel is a wrapper around the smithy JSON AST model
/// which provides a convenient interface to read the model
#[derive(Debug, Clone)]
pub struct SmithyModel {
    pub shapes: SmithyShapeMap,
}

#[derive(Debug, Clone)]
pub struct SmithyShape {
    pub key: String,
    pub name: String,
    pub typ: SmithyType,
    pub traits: Value,
    pub members: SmithyMemberMap,
}

#[derive(Debug, Clone)]
pub struct SmithyMember {
    pub name: String,
    pub snake: String,
    pub traits: Value,
    pub target: String,
}

pub type SmithyShapeMap = HashMap<String, SmithyShape>;
pub type SmithyMemberMap = HashMap<String, SmithyMember>;
pub type JsonObject = Map<String, Value>;

impl SmithyModel {
    pub fn get_shape_of<'a>(&'a self, member: &'a SmithyMember) -> &'a SmithyShape {
        &self.shapes[&member.target]
    }
    pub fn _get_shape_if<'a>(&'a self, member: Option<&'a SmithyMember>) -> Option<&'a SmithyShape> {
        member.map(|m| self.get_shape_of(m))
    }
    pub fn _get_shape_by_key<'a>(&'a self, k: &str) -> &'a SmithyShape {
        &self.shapes[k]
    }
    pub fn iter_shapes_by_type<'a>(
        &'a self,
        t: SmithyType,
    ) -> impl Iterator<Item = &'a SmithyShape> + 'a {
        self.shapes.values().filter(move |s| s.typ == t)
    }
    pub fn _iter_shapes_with_trait<'a>(
        &'a self,
        t: &'a str,
    ) -> impl Iterator<Item = &'a SmithyShape> + 'a {
        self.shapes.values().filter(|s| s.has_trait(t))
    }
    pub fn iter_ops<'a>(&'a self) -> impl Iterator<Item = &'a SmithyShape> + 'a {
        self.iter_shapes_by_type(SmithyType::Operation)
            .filter(|op| op.name != "SelectObjectContent")
    }
}

impl SmithyShape {
    pub fn new(json: &Value, key: &str) -> Self {
        let typ = SmithyType::from(json["type"].as_str().unwrap());
        let traits = json["traits"].to_owned();
        let mut members = SmithyMemberMap::from_json(&json["members"]);
        for k in ["input", "output", "member", "key", "value"].iter() {
            if json[k].is_object() {
                members.insert(k.to_string(), SmithyMember::new(k, &json[k]));
            }
        }
        // TODO json["errors"].as_array()
        // TODO json["operations"].as_array()
        Self {
            key: key.to_string(),
            name: camel(&unprefix(key)),
            typ,
            traits,
            members,
        }
    }
    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }
    pub fn _get_type(&self) -> &str {
        self.typ.as_ref()
    }
}

impl Default for SmithyShape {
    fn default() -> Self {
        SmithyShape {
            key: "".to_string(),
            name: "".to_string(),
            typ: SmithyType::String,
            traits: Value::Null,
            members: SmithyMemberMap::new(),
        }
    }
}

impl SmithyMember {
    pub fn new(key: &str, json: &Value) -> Self {
        let traits = json["traits"].to_owned();
        let target = json["target"].as_str().unwrap_or("").to_string();
        Self {
            name: key.to_string(),
            snake: snake(key),
            traits,
            target,
        }
    }
    pub fn ident(&self) -> Ident {
        if syn::parse_str::<Ident>(&self.snake).is_err() {
            format_ident!("r#{}", self.snake)
        } else {
            format_ident!("{}", self.snake)
        }
    }
    pub fn set_ident(&self) -> Ident {
        format_ident!("set_{}", self.snake)
    }
    pub fn _get_ident(&self) -> Ident {
        format_ident!("get_{}", self.snake)
    }
}

pub trait FromJson: Sized {
    fn from_json(json: &Value) -> Self;
    fn from_json_file(path: &Path) -> Self {
        let v = serde_json::from_reader(File::open(path).unwrap()).unwrap();
        Self::from_json(&v)
    }
}

impl FromJson for SmithyModel {
    fn from_json(json: &Value) -> Self {
        let shapes = SmithyShapeMap::from_json(&json["shapes"]);
        SmithyModel { shapes }
    }
}

impl FromJson for SmithyShapeMap {
    fn from_json(v: &Value) -> Self {
        v.as_object().map_or_else(
            || SmithyShapeMap::new(),
            |m| {
                m.iter()
                    .map(|(k, v)| (k.to_owned(), SmithyShape::new(v, k)))
                    .collect()
            },
        )
    }
}

impl FromJson for SmithyMemberMap {
    fn from_json(v: &Value) -> Self {
        v.as_object().map_or_else(
            || SmithyMemberMap::new(),
            |m| {
                m.iter()
                    .map(|(k, v)| (k.to_owned(), SmithyMember::new(k, v)))
                    .collect()
            },
        )
    }
}

/// SmithyTraits provide an interface to read the traits of a shape or a member
pub trait SmithyTraits {
    fn set_trait(&mut self, t: &str);
    fn has_trait(&self, t: &str) -> bool;
    fn get_trait(&self, t: &str) -> String;
    fn get_trait_value(&self, t: &str) -> Value;
    fn has_http_trait(&self) -> bool {
        self.has_trait(SM_HTTP_LABEL)
            || self.has_trait(SM_HTTP_QUERY)
            || self.has_trait(SM_HTTP_HEADER)
            || self.has_trait(SM_HTTP_PREFIX_HEADERS)
    }
    fn get_doc_summary(&self) -> String {
        let doc = self.get_trait(SM_DOC);
        if doc.is_empty() {
            return String::new();
        }
        let startp = doc.find("<p>").map_or(0, |p| p + 3);
        let endp = doc.find("</p>").unwrap_or(doc.len());
        let summary = doc[startp..endp].to_string();
        summary.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

/// SmithyTraitor provide an interface to get direct access to the complete traits values of a shape or a member
/// It is used to implement SmithyTraits easily for shapes and members loaded from json.
pub trait SmithyTraitor {
    fn traits(&self) -> &Value;
    fn traits_mut(&mut self) -> &mut Value;
}

/// Implement SmithyTraits for any SmithyTraitor
impl<T: SmithyTraitor> SmithyTraits for T {
    fn set_trait(&mut self, t: &str) {
        self.traits_mut()
            .as_object_mut()
            .map(|o| o.insert(t.to_string(), Value::Object(JsonObject::new())));
    }
    fn has_trait(&self, t: &str) -> bool {
        self.traits()
            .as_object()
            .map_or(false, |o| o.contains_key(t))
    }
    fn get_trait(&self, t: &str) -> String {
        self.traits()
            .as_object()
            .and_then(|o| o.get(t))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
    fn get_trait_value(&self, t: &str) -> Value {
        self.traits()
            .as_object()
            .and_then(|o| o.get(t))
            .map_or(Value::Null, |v| v.to_owned())
    }
}

impl SmithyTraitor for SmithyShape {
    fn traits(&self) -> &Value {
        &self.traits
    }
    fn traits_mut(&mut self) -> &mut Value {
        &mut self.traits
    }
}

impl SmithyTraitor for SmithyMember {
    fn traits(&self) -> &Value {
        &self.traits
    }
    fn traits_mut(&mut self) -> &mut Value {
        &mut self.traits
    }
}

/// unprefix returns just the suffix for `prefix#suffix` strings
pub fn unprefix(s: &str) -> String {
    s.split_once('#')
        .map_or_else(|| s.to_string(), |(_prefix, suffix)| suffix.to_string())
}

/// camel changes from MIXOfUPPERCaseAndCamelCase to MixOfUpperCaseAndCamelCase
pub fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut upper_streak = 0;
    for c in s.chars() {
        if c.is_uppercase() || c.is_numeric() {
            if upper_streak == 0 {
                out.push(c);
            } else {
                out.push(c.to_lowercase().next().unwrap());
            }
            upper_streak += 1;
        } else {
            if upper_streak > 1 && out.len() > 1 {
                let c = out.pop().unwrap();
                out.push(c.to_uppercase().next().unwrap());
            }
            out.push(c);
            upper_streak = 0;
        }
    }
    out
}

/// snake changes from CamelCase to snake_case
pub fn snake(s: &str) -> String {
    let mut out = String::new();
    let mut upper_streak = 0;
    for mut c in s.chars() {
        if c.is_uppercase() || c.is_numeric() {
            if upper_streak == 0 && out.len() > 0 && out.chars().last().unwrap() != '_' {
                out.push('_');
            }
            out.push(c.to_lowercase().next().unwrap());
            upper_streak += 1;
        } else {
            if !c.is_alphanumeric() {
                c = '_';
            }
            if upper_streak > 1 && out.len() > 1 && c != '_' {
                let c = out.pop().unwrap();
                out.push('_');
                out.push(c);
            }
            out.push(c);
            upper_streak = 0;
        }
    }
    out
}

/// smithy shape types
/// https://awslabs.github.io/smithy/1.0/spec/core/model.html#
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmithyType {
    // primitive-shapes
    Boolean,
    Byte,
    Short,
    Integer,
    Long,
    Float,
    Double,
    BigInteger,
    BigDecimal,
    // basic-shapes
    Blob,
    String,
    Timestamp,
    Document,
    // aggregate-shapes
    Member,
    List,
    Set,
    Map,
    Structure,
    Union,
    // service-shapes
    Service,
    Operation,
    Resource,
}

impl SmithyType {
    pub fn is_always_required(&self) -> bool {
        match self {
            SmithyType::Blob
            | SmithyType::Boolean
            | SmithyType::Byte
            | SmithyType::Short
            | SmithyType::Integer
            | SmithyType::Long
            | SmithyType::Float
            | SmithyType::Double
            | SmithyType::BigInteger
            | SmithyType::BigDecimal => true,
            _ => false,
        }
    }
}

impl ToString for SmithyType {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}

impl AsRef<str> for SmithyType {
    fn as_ref(&self) -> &str {
        match self {
            // primitive-shapes
            SmithyType::Boolean => SM_TYPE_BOOLEAN,
            SmithyType::Byte => SM_TYPE_BYTE,
            SmithyType::Short => SM_TYPE_SHORT,
            SmithyType::Integer => SM_TYPE_INTEGER,
            SmithyType::Long => SM_TYPE_LONG,
            SmithyType::Float => SM_TYPE_FLOAT,
            SmithyType::Double => SM_TYPE_DOUBLE,
            SmithyType::BigInteger => SM_TYPE_BIGINTEGER,
            SmithyType::BigDecimal => SM_TYPE_BIGDECIMAL,
            // basic-shapes
            SmithyType::Blob => SM_TYPE_BLOB,
            SmithyType::String => SM_TYPE_STRING,
            SmithyType::Timestamp => SM_TYPE_TIMESTAMP,
            SmithyType::Document => SM_TYPE_DOCUMENT,
            // aggregate-shapes
            SmithyType::Member => SM_TYPE_MEMBER,
            SmithyType::List => SM_TYPE_LIST,
            SmithyType::Set => SM_TYPE_SET,
            SmithyType::Map => SM_TYPE_MAP,
            SmithyType::Structure => SM_TYPE_STRUCTURE,
            SmithyType::Union => SM_TYPE_UNION,
            // service-shapes
            SmithyType::Service => SM_TYPE_SERVICE,
            SmithyType::Operation => SM_TYPE_OPERATION,
            SmithyType::Resource => SM_TYPE_RESOURCE,
        }
    }
}

impl From<&str> for SmithyType {
    fn from(s: &str) -> Self {
        match s {
            // primitive-shapes
            SM_TYPE_BOOLEAN => SmithyType::Boolean,
            SM_TYPE_BYTE => SmithyType::Byte,
            SM_TYPE_SHORT => SmithyType::Short,
            SM_TYPE_INTEGER => SmithyType::Integer,
            SM_TYPE_LONG => SmithyType::Long,
            SM_TYPE_FLOAT => SmithyType::Float,
            SM_TYPE_DOUBLE => SmithyType::Double,
            SM_TYPE_BIGINTEGER => SmithyType::BigInteger,
            SM_TYPE_BIGDECIMAL => SmithyType::BigDecimal,
            // basic-shapes
            SM_TYPE_BLOB => SmithyType::Blob,
            SM_TYPE_STRING => SmithyType::String,
            SM_TYPE_TIMESTAMP => SmithyType::Timestamp,
            SM_TYPE_DOCUMENT => SmithyType::Document,
            // aggregate-shapes
            SM_TYPE_MEMBER => SmithyType::Member,
            SM_TYPE_LIST => SmithyType::List,
            SM_TYPE_SET => SmithyType::Set,
            SM_TYPE_MAP => SmithyType::Map,
            SM_TYPE_STRUCTURE => SmithyType::Structure,
            SM_TYPE_UNION => SmithyType::Union,
            // service-shapes
            SM_TYPE_SERVICE => SmithyType::Service,
            SM_TYPE_OPERATION => SmithyType::Operation,
            SM_TYPE_RESOURCE => SmithyType::Resource,
            _ => panic!("unknown SmithyType: {}", s),
        }
    }
}

// primitive-shapes
const SM_TYPE_BOOLEAN: &str = "boolean";
const SM_TYPE_BYTE: &str = "byte";
const SM_TYPE_SHORT: &str = "short";
const SM_TYPE_INTEGER: &str = "integer";
const SM_TYPE_LONG: &str = "long";
const SM_TYPE_FLOAT: &str = "float";
const SM_TYPE_DOUBLE: &str = "double";
const SM_TYPE_BIGINTEGER: &str = "bigInteger";
const SM_TYPE_BIGDECIMAL: &str = "bigDecimal";
// basic-shapes
const SM_TYPE_BLOB: &str = "blob";
const SM_TYPE_STRING: &str = "string";
const SM_TYPE_TIMESTAMP: &str = "timestamp";
const SM_TYPE_DOCUMENT: &str = "document";
// aggregate-shapes
const SM_TYPE_MEMBER: &str = "member";
const SM_TYPE_LIST: &str = "list";
const SM_TYPE_SET: &str = "set";
const SM_TYPE_MAP: &str = "map";
const SM_TYPE_STRUCTURE: &str = "structure";
const SM_TYPE_UNION: &str = "union";
// service-shapes
const SM_TYPE_SERVICE: &str = "service";
const SM_TYPE_OPERATION: &str = "operation";
const SM_TYPE_RESOURCE: &str = "resource";

// smithy traits used in s3.json:
const _SM_PREFIX: &str = "smithy.api#";
pub const SM_ENUM: &str = "smithy.api#enum";
pub const SM_REQUIRED: &str = "smithy.api#required";
const SM_DOC: &str = "smithy.api#documentation";
const _SM_ERROR: &str = "smithy.api#error";
const _SM_HTTP: &str = "smithy.api#http";
#[allow(unused)]
const SM_HTTP_LABEL: &str = "smithy.api#httpLabel";
#[allow(unused)]
const SM_HTTP_QUERY: &str = "smithy.api#httpQuery";
#[allow(unused)]
const SM_HTTP_HEADER: &str = "smithy.api#httpHeader";
const _SM_HTTP_PAYLOAD: &str = "smithy.api#httpPayload";
#[allow(unused)]
const SM_HTTP_PREFIX_HEADERS: &str = "smithy.api#httpPrefixHeaders";
const _SM_HTTP_CHECKSUM_REQUIRED: &str = "smithy.api#httpChecksumRequired";
const _SM_XML_NS: &str = "smithy.api#xmlNamespace";
const _SM_XML_NAME: &str = "smithy.api#xmlName";
const _SM_XML_ATTR: &str = "smithy.api#xmlAttribute";
const _SM_XML_FLATTENED: &str = "smithy.api#xmlFlattened";
const _SM_SENSITIVE: &str = "smithy.api#sensitive";
const _SM_TIMESTAMP_FORMAT: &str = "smithy.api#timestampFormat";
const _SM_EVENT_PAYLOAD: &str = "smithy.api#eventPayload";
const _SM_STREAMING: &str = "smithy.api#streaming";
const _SM_PAGINATED: &str = "smithy.api#paginated";
const _SM_DEPRECATED: &str = "smithy.api#deprecated";
const _SM_TITLE: &str = "smithy.api#title";
const _SM_PATTERN: &str = "smithy.api#pattern";
const _SM_LENGTH: &str = "smithy.api#length";
const _SM_HOST_LABEL: &str = "smithy.api#hostLabel";
const _SM_ENDPOINT: &str = "smithy.api#endpoint";
const _SM_AUTH: &str = "smithy.api#auth";
