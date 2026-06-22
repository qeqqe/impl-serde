#![feature(prelude_import)]
extern crate std;
use std::collections::{HashMap, HashSet};
#[prelude_import]
use std::prelude::rust_2024::*;
mod macros {}
use deser_macros::Deserialize;
use ser_macros::Serialize;
pub trait Serializer {
    fn to_str(&self) -> String;
    fn __to_str_depth(&self, depth: usize) -> String;
}
impl Serializer for i32 {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, _depth: usize) -> String {
        self.to_string()
    }
}
impl Serializer for i64 {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, _depth: usize) -> String {
        self.to_string()
    }
}
impl Serializer for u32 {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, _depth: usize) -> String {
        self.to_string()
    }
}
impl Serializer for f32 {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, _depth: usize) -> String {
        self.to_string()
    }
}
impl Serializer for f64 {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, _depth: usize) -> String {
        self.to_string()
    }
}
impl Serializer for String {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, _depth: usize) -> String {
        ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("\"{0}\"", self)) })
    }
}
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
#[automatically_derived]
impl ::core::fmt::Debug for JsonValue {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            JsonValue::Null => ::core::fmt::Formatter::write_str(f, "Null"),
            JsonValue::Bool(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Bool", &__self_0)
            }
            JsonValue::Number(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Number", &__self_0)
            }
            JsonValue::Str(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Str", &__self_0)
            }
            JsonValue::Array(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Array", &__self_0)
            }
            JsonValue::Object(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Object", &__self_0)
            }
        }
    }
}
trait Visitor: Sized {
    type Output;
    fn visit_seq(self, seq: &[JsonValue]) -> Self::Output;
    fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output;
}
pub trait Deserializer: Sized {
    fn deserialize(v: &JsonValue) -> Self;
}
impl Deserializer for String {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Str(s) => s.clone(),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected string"));
            }
        }
    }
}
impl Deserializer for char {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Str(s) => s.chars().next().expect("empty string for char"),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected string for char"));
            }
        }
    }
}
impl Deserializer for bool {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Bool(b) => *b,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected bool"));
            }
        }
    }
}
impl Deserializer for f64 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for f32 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as f32,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for i8 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as i8,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for i16 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as i16,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for i32 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as i32,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for i64 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as i64,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for i128 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as i128,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for isize {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as isize,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for u8 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as u8,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for u16 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as u16,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for u32 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as u32,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for u64 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as u64,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for u128 {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as u128,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl Deserializer for usize {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Number(n) => *n as usize,
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected number"));
            }
        }
    }
}
impl<T: Deserializer> Deserializer for Vec<T> {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Array(arr) => arr.iter().map(|v| T::deserialize(v)).collect(),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected array"));
            }
        }
    }
}
impl<T: Deserializer + Eq + std::hash::Hash> Deserializer for std::collections::HashSet<T> {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Array(arr) => arr.iter().map(|v| T::deserialize(v)).collect(),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected array for HashSet"));
            }
        }
    }
}
impl<V: Deserializer> Deserializer for std::collections::HashMap<String, V> {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Object(map) => map
                .iter()
                .map(|(k, v)| (k.clone(), V::deserialize(v)))
                .collect(),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected object for HashMap"));
            }
        }
    }
}
fn parse_json(input: &str) -> JsonValue {
    parse_value(input.trim()).0
}
fn parse_value(s: &str) -> (JsonValue, &str) {
    let s = s.trim_start();
    match s.chars().next().expect("empty input") {
        '{' => parse_object(&s[1..]),
        '[' => parse_array(&s[1..]),
        '"' => {
            let (st, rest) = parse_str_inner(&s[1..]);
            (JsonValue::Str(st), rest)
        }
        't' => (JsonValue::Bool(true), &s[4..]),
        'f' => (JsonValue::Bool(false), &s[5..]),
        'n' => (JsonValue::Null, &s[4..]),
        _ => parse_number(s),
    }
}
fn parse_object(s: &str) -> (JsonValue, &str) {
    let mut map = std::collections::HashMap::new();
    let mut s = s.trim_start();
    if s.starts_with('}') {
        return (JsonValue::Object(map), &s[1..]);
    }
    loop {
        s = s.trim_start();
        let inner = s.strip_prefix('"').expect("expected string key");
        let (key, rest) = parse_str_inner(inner);
        s = rest.trim_start().strip_prefix(':').expect("expected ':'");
        let (val, rest) = parse_value(s);
        map.insert(key, val);
        s = rest.trim_start();
        if let Some(r) = s.strip_prefix('}') {
            return (JsonValue::Object(map), r);
        }
        s = s.strip_prefix(',').expect("expected ','");
    }
}
fn parse_array(s: &str) -> (JsonValue, &str) {
    let mut arr = Vec::new();
    let mut s = s.trim_start();
    if s.starts_with(']') {
        return (JsonValue::Array(arr), &s[1..]);
    }
    loop {
        let (val, rest) = parse_value(s);
        arr.push(val);
        s = rest.trim_start();
        if let Some(r) = s.strip_prefix(']') {
            return (JsonValue::Array(arr), r);
        }
        s = s.strip_prefix(',').expect("expected ','");
    }
}
fn parse_str_inner(s: &str) -> (String, &str) {
    let mut result = String::new();
    let mut chars = s.char_indices();
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => return (result, &s[i + 1..]),
            '\\' => {
                if let Some((_, esc)) = chars.next() {
                    result.push(match esc {
                        '"' => '"',
                        '\\' => '\\',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        c => c,
                    });
                }
            }
            _ => result.push(c),
        }
    }
    {
        ::core::panicking::panic_fmt(format_args!("unterminated string"));
    };
}
fn parse_number(s: &str) -> (JsonValue, &str) {
    let end = s
        .find(|c: char| !#[allow(non_exhaustive_omitted_patterns)]
        match c {
            '0'..='9' | '.' | '-' | '+' | 'e' | 'E' => true,
            _ => false,
        })
        .unwrap_or(s.len());
    let n: f64 = s[..end].parse().expect("invalid number");
    (JsonValue::Number(n), &s[end..])
}
pub struct UselessStruct {
    field: i64,
}
#[automatically_derived]
impl ::core::fmt::Debug for UselessStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UselessStruct",
            "field",
            &&self.field,
        )
    }
}
impl Serializer for UselessStruct {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", indent, "field", self.field))
        }));
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
        }));
        json
    }
}
struct __UselessStructVisitor;
impl Visitor for __UselessStructVisitor {
    type Output = UselessStruct;
    fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
        UselessStruct {
            field: <i64 as Deserializer>::deserialize(map.get("field").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "field"));
            })),
        }
    }
    fn visit_seq(self, _s: &[JsonValue]) -> Self::Output {
        {
            ::core::panicking::panic_fmt(format_args!(
                "expected item for {0}, got array",
                "UselessStruct"
            ));
        }
    }
}
impl Deserializer for UselessStruct {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Object(map) => __UselessStructVisitor.visit_map(map),
            _ => {
                ::core::panicking::panic_fmt(format_args!(
                    "expected item for {0}",
                    "UselessStruct"
                ));
            }
        }
    }
}
pub struct Points {
    x: i64,
    y: i64,
    z: i64,
    dim: String,
    us: UselessStruct,
}
#[automatically_derived]
impl ::core::fmt::Debug for Points {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f, "Points", "x", &self.x, "y", &self.y, "z", &self.z, "dim", &self.dim, "us",
            &&self.us,
        )
    }
}
impl Serializer for Points {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", indent, "x", self.x))
        }));
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", indent, "y", self.y))
        }));
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", indent, "z", self.z))
        }));
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": \"{2}\"", indent, "dim", self.dim))
        }));
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "us"))
        }));
        json.push_str(&self.us.__to_str_depth(depth + 1));
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
        }));
        json
    }
}
struct __PointsVisitor;
impl Visitor for __PointsVisitor {
    type Output = Points;
    fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
        Points {
            x: <i64 as Deserializer>::deserialize(map.get("x").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "x"));
            })),
            y: <i64 as Deserializer>::deserialize(map.get("y").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "y"));
            })),
            z: <i64 as Deserializer>::deserialize(map.get("z").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "z"));
            })),
            dim: <String as Deserializer>::deserialize(map.get("dim").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "dim"));
            })),
            us: <UselessStruct as Deserializer>::deserialize(map.get("us").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "us"));
            })),
        }
    }
    fn visit_seq(self, _s: &[JsonValue]) -> Self::Output {
        {
            ::core::panicking::panic_fmt(format_args!(
                "expected item for {0}, got array",
                "Points"
            ));
        }
    }
}
impl Deserializer for Points {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Object(map) => __PointsVisitor.visit_map(map),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected item for {0}", "Points"));
            }
        }
    }
}
pub struct Unnamed(i32, u32, f64, f32);
#[automatically_derived]
impl ::core::fmt::Debug for Unnamed {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field4_finish(
            f, "Unnamed", &self.0, &self.1, &self.2, &&self.3,
        )
    }
}
impl Serializer for Unnamed {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("[\n");
        json.push_str("");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!(
                "{0}{1}",
                item_indent,
                self.0.__to_str_depth(depth + 2)
            ))
        }));
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!(
                "{0}{1}",
                item_indent,
                self.1.__to_str_depth(depth + 2)
            ))
        }));
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!(
                "{0}{1}",
                item_indent,
                self.2.__to_str_depth(depth + 2)
            ))
        }));
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!(
                "{0}{1}",
                item_indent,
                self.3.__to_str_depth(depth + 2)
            ))
        }));
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("\n{0}]", "  ".repeat(depth)))
        }));
        json
    }
}
struct __UnnamedVisitor;
impl Visitor for __UnnamedVisitor {
    type Output = Unnamed;
    fn visit_seq(self, seq: &[JsonValue]) -> Self::Output {
        Unnamed(
            <i32 as Deserializer>::deserialize(seq.get(0usize).unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing index {0} in tuple", 0usize));
            })),
            <u32 as Deserializer>::deserialize(seq.get(1usize).unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing index {0} in tuple", 1usize));
            })),
            <f64 as Deserializer>::deserialize(seq.get(2usize).unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing index {0} in tuple", 2usize));
            })),
            <f32 as Deserializer>::deserialize(seq.get(3usize).unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing index {0} in tuple", 3usize));
            })),
        )
    }
    fn visit_map(self, _m: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
        {
            ::core::panicking::panic_fmt(format_args!(
                "expected array for {0}, got object",
                "Unnamed"
            ));
        }
    }
}
impl Deserializer for Unnamed {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Array(seq) => __UnnamedVisitor.visit_seq(seq),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected array for {0}", "Unnamed"));
            }
        }
    }
}
pub struct SomeOtherStuff(i32);
#[automatically_derived]
impl ::core::fmt::Debug for SomeOtherStuff {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "SomeOtherStuff", &&self.0)
    }
}
impl Serializer for SomeOtherStuff {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("[\n");
        json.push_str("");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!(
                "{0}{1}",
                item_indent,
                self.0.__to_str_depth(depth + 2)
            ))
        }));
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("\n{0}]", "  ".repeat(depth)))
        }));
        json
    }
}
struct __SomeOtherStuffVisitor;
impl Visitor for __SomeOtherStuffVisitor {
    type Output = SomeOtherStuff;
    fn visit_seq(self, seq: &[JsonValue]) -> Self::Output {
        SomeOtherStuff(<i32 as Deserializer>::deserialize(
            seq.get(0usize).unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing index {0} in tuple", 0usize));
            }),
        ))
    }
    fn visit_map(self, _m: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
        {
            ::core::panicking::panic_fmt(format_args!(
                "expected array for {0}, got object",
                "SomeOtherStuff"
            ));
        }
    }
}
impl Deserializer for SomeOtherStuff {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Array(seq) => __SomeOtherStuffVisitor.visit_seq(seq),
            _ => {
                ::core::panicking::panic_fmt(format_args!(
                    "expected array for {0}",
                    "SomeOtherStuff"
                ));
            }
        }
    }
}
pub struct SomeMoreStuff(u32);
#[automatically_derived]
impl ::core::fmt::Debug for SomeMoreStuff {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "SomeMoreStuff", &&self.0)
    }
}
impl Serializer for SomeMoreStuff {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("[\n");
        json.push_str("");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!(
                "{0}{1}",
                item_indent,
                self.0.__to_str_depth(depth + 2)
            ))
        }));
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("\n{0}]", "  ".repeat(depth)))
        }));
        json
    }
}
struct __SomeMoreStuffVisitor;
impl Visitor for __SomeMoreStuffVisitor {
    type Output = SomeMoreStuff;
    fn visit_seq(self, seq: &[JsonValue]) -> Self::Output {
        SomeMoreStuff(<u32 as Deserializer>::deserialize(
            seq.get(0usize).unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing index {0} in tuple", 0usize));
            }),
        ))
    }
    fn visit_map(self, _m: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
        {
            ::core::panicking::panic_fmt(format_args!(
                "expected array for {0}, got object",
                "SomeMoreStuff"
            ));
        }
    }
}
impl Deserializer for SomeMoreStuff {
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Array(seq) => __SomeMoreStuffVisitor.visit_seq(seq),
            _ => {
                ::core::panicking::panic_fmt(format_args!(
                    "expected array for {0}",
                    "SomeMoreStuff"
                ));
            }
        }
    }
}
pub struct GenTest<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    val: T,
    val_vec: Vec<U>,
    norm: i32,
}
#[automatically_derived]
impl<T: ::core::fmt::Debug, U: ::core::fmt::Debug> ::core::fmt::Debug for GenTest<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "GenTest",
            "val",
            &self.val,
            "val_vec",
            &self.val_vec,
            "norm",
            &&self.norm,
        )
    }
}
impl<T, U> Serializer for GenTest<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "val"))
        }));
        json.push_str(&self.val.__to_str_depth(depth + 1));
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        let v = &self.val_vec;
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": [", indent, "val_vec"))
        }));
        if v.is_empty() {
            json.push_str("]");
        } else {
            json.push_str("\n");
            for (idx, item) in v.iter().enumerate() {
                if idx > 0 {
                    json.push_str(",\n");
                }
                json.push_str(&::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!(
                        "{0}{1}",
                        item_indent,
                        item.__to_str_depth(depth + 2),
                    ))
                }));
            }
            json.push_str(&::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}]", indent))
            }));
        }
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", indent, "norm", self.norm))
        }));
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
        }));
        json
    }
}
struct __GenTestVisitor<T, U>(std::marker::PhantomData<(T, U)>);
impl<T, U> Visitor for __GenTestVisitor<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    type Output = GenTest<T, U>;
    fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
        GenTest {
            val: <T as Deserializer>::deserialize(map.get("val").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "val"));
            })),
            val_vec: <Vec<U> as Deserializer>::deserialize(map.get("val_vec").unwrap_or_else(
                || {
                    ::core::panicking::panic_fmt(format_args!("missing field: {0}", "val_vec"));
                },
            )),
            norm: <i32 as Deserializer>::deserialize(map.get("norm").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "norm"));
            })),
        }
    }
    fn visit_seq(self, _s: &[JsonValue]) -> Self::Output {
        {
            ::core::panicking::panic_fmt(format_args!(
                "expected item for {0}, got array",
                "GenTest"
            ));
        }
    }
}
impl<T, U> Deserializer for GenTest<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Object(map) => __GenTestVisitor(std::marker::PhantomData).visit_map(map),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected item for {0}", "GenTest"));
            }
        }
    }
}
pub struct D<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    points: Points,
    v: Vec<i32>,
    c: char,
    st: String,
    se: HashSet<i32>,
    mp: HashMap<String, i64>,
    un: Unnamed,
    g: GenTest<T, U>,
}
#[automatically_derived]
impl<T: ::core::fmt::Debug, U: ::core::fmt::Debug> ::core::fmt::Debug for D<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &["points", "v", "c", "st", "se", "mp", "un", "g"];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.points,
            &self.v,
            &self.c,
            &self.st,
            &self.se,
            &self.mp,
            &self.un,
            &&self.g,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "D", names, values)
    }
}
impl<T, U> Serializer for D<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "points"))
        }));
        json.push_str(&self.points.__to_str_depth(depth + 1));
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        let v = &self.v;
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": [", indent, "v"))
        }));
        if v.is_empty() {
            json.push_str("]");
        } else {
            json.push_str("\n");
            for (idx, item) in v.iter().enumerate() {
                if idx > 0 {
                    json.push_str(",\n");
                }
                json.push_str(&::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!(
                        "{0}{1}",
                        item_indent,
                        item.__to_str_depth(depth + 2),
                    ))
                }));
            }
            json.push_str(&::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}]", indent))
            }));
        }
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": \"{2}\"", indent, "c", self.c))
        }));
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": \"{2}\"", indent, "st", self.st))
        }));
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        let v = &self.se;
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": [", indent, "se"))
        }));
        if v.is_empty() {
            json.push_str("]");
        } else {
            json.push_str("\n");
            for (idx, item) in v.iter().enumerate() {
                if idx > 0 {
                    json.push_str(",\n");
                }
                json.push_str(&::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!(
                        "{0}{1}",
                        item_indent,
                        item.__to_str_depth(depth + 2),
                    ))
                }));
            }
            json.push_str(&::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}]", indent))
            }));
        }
        json.push_str(",\n");
        let item_ident = "  ".repeat(depth + 2);
        let map = &self.mp;
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": {{", indent, "mp"))
        }));
        if map.is_empty() {
            json.push_str("}")
        } else {
            json.push_str("\n");
            for (idx, (k, v)) in map.iter().enumerate() {
                if idx > 0 {
                    json.push_str(",\n");
                }
                json.push_str(&::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", item_ident, k, v))
                }));
            }
            json.push_str(&::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}}}", indent))
            }));
        }
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "un"))
        }));
        json.push_str(&self.un.__to_str_depth(depth + 1));
        json.push_str(",\n");
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "g"))
        }));
        json.push_str(&self.g.__to_str_depth(depth + 1));
        json.push_str(&::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
        }));
        json
    }
}
struct __DVisitor<T, U>(std::marker::PhantomData<(T, U)>);
impl<T, U> Visitor for __DVisitor<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    type Output = D<T, U>;
    fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
        D {
            points: <Points as Deserializer>::deserialize(map.get("points").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "points"));
            })),
            v: <Vec<i32> as Deserializer>::deserialize(map.get("v").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "v"));
            })),
            c: <char as Deserializer>::deserialize(map.get("c").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "c"));
            })),
            st: <String as Deserializer>::deserialize(map.get("st").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "st"));
            })),
            se: <HashSet<i32> as Deserializer>::deserialize(map.get("se").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "se"));
            })),
            mp: <HashMap<String, i64> as Deserializer>::deserialize(map.get("mp").unwrap_or_else(
                || {
                    ::core::panicking::panic_fmt(format_args!("missing field: {0}", "mp"));
                },
            )),
            un: <Unnamed as Deserializer>::deserialize(map.get("un").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "un"));
            })),
            g: <GenTest<T, U> as Deserializer>::deserialize(map.get("g").unwrap_or_else(|| {
                ::core::panicking::panic_fmt(format_args!("missing field: {0}", "g"));
            })),
        }
    }
    fn visit_seq(self, _s: &[JsonValue]) -> Self::Output {
        {
            ::core::panicking::panic_fmt(format_args!("expected item for {0}, got array", "D"));
        }
    }
}
impl<T, U> Deserializer for D<T, U>
where
    T: Serializer + Deserializer,
    U: Serializer + Deserializer,
{
    fn deserialize(v: &JsonValue) -> Self {
        match v {
            JsonValue::Object(map) => __DVisitor(std::marker::PhantomData).visit_map(map),
            _ => {
                ::core::panicking::panic_fmt(format_args!("expected item for {0}", "D"));
            }
        }
    }
}
fn main() {
    let g = GenTest {
        val: SomeMoreStuff(12),
        val_vec: Vec::<SomeOtherStuff>::new(),
        norm: 2,
    };
    let np = Points {
        x: 1,
        y: 2,
        z: 5,
        dim: "3d".into(),
        us: UselessStruct { field: 4 },
    };
    let un = Unnamed(1, 2, 3.12, 4.23);
    let d = D {
        points: np,
        v: ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [1, 2, 3, 4, 5],
            ),
        ),
        c: 'c',
        st: String::from("Allo"),
        se: HashSet::from([1, 2, 2, 4]),
        mp: HashMap::from([
            ("Apple".to_owned(), 2),
            ("Banama".to_owned(), 7),
            ("Watermelom".to_owned(), 10),
        ]),
        un,
        g,
    };
    let data_str = d.to_str();
    {
        ::std::io::_print(format_args!("{0}\n", data_str));
    };
    let json_value = parse_json(&data_str);
    let deser = D::<SomeMoreStuff, SomeOtherStuff>::deserialize(&json_value);
    {
        ::std::io::_print(format_args!("{0:#?}\n", deser));
    };
}
