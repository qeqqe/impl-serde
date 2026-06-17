#![feature(prelude_import)]
#![allow(dead_code)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use impl_serde::Serialize;
trait Serializer {
    fn to_str(&self) -> String;
}
impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("formated shhit"))
    }
}
pub struct Points {
    x: i64,
    y: i64,
    z: i64,
    dim: String,
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
        {
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
                            ::alloc::fmt::format(
                                format_args!("{0}{1}", item_indent, item),
                            )
                        }),
                    );
                }
                json.push_str(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("\n{0}]", indent))
                    }),
                );
            }
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
    };
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
    };
    {
        ::std::io::_print(format_args!("{0}\n", d.to_str()));
    };
}
