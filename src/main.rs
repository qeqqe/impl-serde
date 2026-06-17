#![allow(dead_code)]

use impl_serde::Serialize;

trait Serializer {
    fn to_str(&self) -> String;
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "formated shhit")
    }
}

#[derive(Serialize)]
pub struct Points {
    x: i64,
    y: i64,
    z: i64,
    dim: String,
}

#[derive(Serialize)]
pub struct D {
    points: Points,
    v: Vec<i32>,
    c: char,
    st: String,
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
        v: vec![1, 2, 3, 4, 5],
        c: 'c',
        st: String::from("Allo"),
    };

    println!("{}", d.to_str());
}
