#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use ser_macros::Serialize;

trait Serializer {
    fn to_str(&self) -> String;
}

#[derive(Serialize)]
pub struct UselessStruct {
    field: i64,
}

#[derive(Serialize)]
pub struct Points {
    x: i64,
    y: i64,
    z: i64,
    dim: String,
    us: UselessStruct,
}

#[derive(Serialize)]
pub struct D {
    points: Points,
    v: Vec<i32>,
    c: char,
    st: String,
    se: HashSet<i32>,
    mp: HashMap<String, i64>,
    un: Unnamed,
}
#[derive(Serialize)]
pub struct Unnamed(i32, u32, f64, f32);

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
        v: vec![1, 2, 3, 4, 5],
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

    println!("{}", d.to_str());
}
