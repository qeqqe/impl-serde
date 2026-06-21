#[macro_export]
macro_rules! serialize_trait {
    () => {
        trait Serializer {
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
        enum JsonValue {
            Null,
            Bool(bool),
            Number(f64),
            Str(String),
            Array(Vec<JsonValue>),
            Object(HashMap<String, JsonValue>),
        }

        trait Visitor: Sized {
            type Output;
            fn visit_str(self, v: &str) -> Self::Output;
            fn visit_number(self, v: f64) -> Self::Output;
            fn visit_seq(self, seq: &[JsonValue]) -> Self::Output;
            fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output;
        }

        trait Deserialize: Sized {
            fn deserialize(v: &JsonValue) -> Self;
        }

        // primitives
        impl Deserialize for String {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Str(s) => s.clone(),
                    _ => panic!("expected string"),
                }
            }
        }
        impl Deserialize for char {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Str(s) => s.chars().next().expect("empty string for char"),
                    _ => panic!("expected string for char"),
                }
            }
        }
        impl Deserialize for bool {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Bool(b) => *b,
                    _ => panic!("expected bool"),
                }
            }
        }
        // numero
        impl Deserialize for f64 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for f32 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as f32,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for i8 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i8,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for i16 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i16,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for i32 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i32,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for i64 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i64,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for i128 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as i128,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for isize {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as isize,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for u8 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u8,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for u16 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u16,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for u32 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u32,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for u64 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u64,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for u128 {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as u128,
                    _ => panic!("expected number"),
                }
            }
        }
        impl Deserialize for usize {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Number(n) => *n as usize,
                    _ => panic!("expected number"),
                }
            }
        }

        impl<T: Deserialize> Deserialize for Vec<T> {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Array(arr) => arr.iter().map(|v| T::deserialize(v)).collect(),
                    _ => panic!("expected array"),
                }
            }
        }

        impl<T: Deserialize + Eq + std::hash::Hash> Deserialize for std::collections::HashSet<T> {
            fn deserialize(v: &JsonValue) -> Self {
                match v {
                    JsonValue::Array(arr) => arr.iter().map(|v| T::deserialize(v)).collect(),
                    _ => panic!("expected array for HashSet"),
                }
            }
        }

        impl<V: Deserialize> Deserialize for std::collections::HashMap<String, V> {
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
    };
}
