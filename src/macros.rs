#[macro_export]
macro_rules! serialize_primitive {
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
