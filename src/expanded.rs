#![feature(prelude_import)]
#![allow(dead_code)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use impl_serde::Serialize;
trait Serializer {
    fn to_str(&self) -> String;
}
pub struct Points {
    x: i64,
    y: i64,
    z: i64,
    dim: String,
}
impl Serializer for Points {
    fn to_str(&self) -> String {
        let mut json = String::new();
        json.push_str("{");
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", "", "x", self.x))
            }),
        );
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", ",", "y", self.y))
            }),
        );
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}\"{1}\": {2}", ",", "z", self.z))
            }),
        );
        json.push_str(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}\"{1}\": \"{2}\"", ",", "dim", self.dim),
                )
            }),
        );
        json.push_str("}");
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
    let mut st = String::new();
    st.push_str(
        &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", np.x)) }),
    );
    {
        ::std::io::_print(format_args!("{0}\n", np.to_str()));
    };
}
