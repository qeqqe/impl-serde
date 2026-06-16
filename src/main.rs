#![allow(dead_code)]

use impl_serde::Serialize;

#[derive(Serialize)]
pub struct Points {
    x: i64,
    y: i64,
    z: String,
}

fn main() {
    println!("Hello, world!");
}
