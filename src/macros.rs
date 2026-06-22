#[macro_export]
macro_rules! serialize_trait {
    () => {
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
                format!("\"{}\"", self)
            }
        }
    };
}

#[macro_export]
macro_rules! deserialize_trait {
    () => {
        #[derive(Debug)]
        pub enum JsonValue {
            Null,
            Bool(bool),
            Number(f64),
            Str(String),
            Array(Vec<JsonValue>),
            Object(HashMap<String, JsonValue>),
        }

        trait Visitor: Sized {
            type Output;
            fn visit_seq(self, seq: &[JsonValue]) -> Self::Output;
            fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output;
        }

        pub trait Deserializer: Sized {
            fn deserialize(v: &JsonValue) -> Self;
        }

        // primitives
        impl Deserializer for String {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Str(s) => s.clone(),
                    _ => panic!("expected string"),
                }
            }
        }
        impl Deserializer for char {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Str(s) => s.chars().next().expect("empty string for char"),
                    _ => panic!("expected string for char"),
                }
            }
        }
        impl Deserializer for bool {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Bool(b) => *b,
                    _ => panic!("expected bool"),
                }
            }
        }
        // numero
        impl Deserializer for f64 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for f32 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as f32,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for i8 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i8,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for i16 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i16,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for i32 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i32,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for i64 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i64,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for i128 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i128,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for isize {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as isize,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for u8 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u8,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for u16 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u16,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for u32 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u32,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for u64 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u64,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for u128 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u128,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserializer for usize {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as usize,
                    _ => panic!("expected number"),
                }
            }
        }

        impl<T: Deserializer> Deserializer for Vec<T> {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Array(arr) => arr.iter().map(|v| T::deserialize(v)).collect(),
                    _ => panic!("expected array"),
                }
            }
        }

        impl<T: Deserializer + Eq + std::hash::Hash> Deserializer for std::collections::HashSet<T> {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Array(arr) => arr.iter().map(|v| T::deserialize(v)).collect(),
                    _ => panic!("expected array for HashSet"),
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
                    _ => panic!("expected object for HashMap"),
                }
            }
        }

        // parsing
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
            panic!("unterminated string");
        }

        fn parse_number(s: &str) -> (JsonValue, &str) {
            let end = s
                .find(|c: char| !matches!(c, '0'..='9' | '.' | '-' | '+' | 'e' | 'E'))
                .unwrap_or(s.len());
            let n: f64 = s[..end].parse().expect("invalid number");
            (JsonValue::Number(n), &s[end..])
        }
    };
}
