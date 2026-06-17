#![feature(prelude_import)]
#![allow(dead_code)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::collections::{HashMap, HashSet};
use impl_serde::Serialize;
trait Serializer {
    fn to_str(&self) -> String;
}
pub struct UselessStruct {
    field: i64,
}
impl Serializer for UselessStruct {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
}
impl UselessStruct {
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": {2}", indent, "field", self.field),
                )
            }),
        );
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
            }),
        );
        json
    }
}
pub struct Points {
    x: i64,
    y: i64,
    z: i64,
    dim: String,
    us: UselessStruct,
}
impl Serializer for Points {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
}
impl Points {
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": {2}", indent, "x", self.x),
                )
            }),
        );
        json.push_str(",\n");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": {2}", indent, "y", self.y),
                )
            }),
        );
        json.push_str(",\n");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": {2}", indent, "z", self.z),
                )
            }),
        );
        json.push_str(",\n");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": \"{2}\"", indent, "dim", self.dim),
                )
            }),
        );
        json.push_str(",\n");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "us"))
            }),
        );
        json.push_str(&self.us.__to_str_depth(depth + 1));
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
            }),
        );
        json
    }
}
pub struct D {
    points: Points,
    v: Vec<i32>,
    c: char,
    st: String,
    se: HashSet<i32>,
    mp: HashMap<String, i64>,
    un: Unnamed,
}
impl Serializer for D {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
}
impl D {
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "points"))
            }),
        );
        json.push_str(&self.points.__to_str_depth(depth + 1));
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        let v = &self.v;
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": [", indent, "v"))
            }),
        );
        if v.is_empty() {
            json.push_str("]");
        } else {
            json.push_str("\n");
            for (idx, item) in v.iter().enumerate() {
                if idx > 0 {
                    json.push_str(",\n");
                }
                json.push_str(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}{1}", item_indent, item))
                    }),
                );
            }
            json.push_str(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("\n{0}]", indent))
                }),
            );
        }
        json.push_str(",\n");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": \"{2}\"", indent, "c", self.c),
                )
            }),
        );
        json.push_str(",\n");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": \"{2}\"", indent, "st", self.st),
                )
            }),
        );
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        let v = &self.se;
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": [", indent, "se"))
            }),
        );
        if v.is_empty() {
            json.push_str("]");
        } else {
            json.push_str("\n");
            for (idx, item) in v.iter().enumerate() {
                if idx > 0 {
                    json.push_str(",\n");
                }
                json.push_str(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}{1}", item_indent, item))
                    }),
                );
            }
            json.push_str(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("\n{0}]", indent))
                }),
            );
        }
        json.push_str(",\n");
        let item_ident = "  ".repeat(depth + 2);
        let map = &self.mp;
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": {{", indent, "mp"))
            }),
        );
        if map.is_empty() {
            json.push_str("]")
        } else {
            json.push_str("\n");
            for (idx, (k, v)) in map.iter().enumerate() {
                if idx > 0 {
                    json.push_str(",\n");
                }
                json.push_str(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0}\"{1}\": {2}", item_ident, k, v),
                        )
                    }),
                );
            }
            json.push_str(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("\n{0}}}", indent))
                }),
            );
        }
        json.push_str(",\n");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": ", indent, "un"))
            }),
        );
        json.push_str(&self.un.__to_str_depth(depth + 1));
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
            }),
        );
        json
    }
}
pub struct Unnamed(i32, u32, f64, f32);
impl Serializer for Unnamed {
    fn to_str(&self) -> String {
        self.__to_str_depth(0)
    }
}
impl Unnamed {
    fn __to_str_depth(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth + 1);
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}{1}", item_indent, &self.0))
            }),
        );
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}{1}", item_indent, &self.1))
            }),
        );
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}{1}", item_indent, &self.2))
            }),
        );
        json.push_str(",\n");
        let item_indent = "  ".repeat(depth + 2);
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}{1}", item_indent, &self.3))
            }),
        );
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("\n{0}}}", "  ".repeat(depth)))
            }),
        );
        json
    }
}
fn main() {
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
    };
    {
        ::std::io::_print(format_args!("{0}\n", d.to_str()));
    };
}
