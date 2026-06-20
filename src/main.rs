#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
mod macros;

use ser_macros::Serialize;

serialize_primitive!();

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
pub struct Unnamed(i32, u32, f64, f32);
#[derive(Debug, Serialize)]
pub struct SomeOtherStuff(i32);

#[derive(Debug, Serialize)]
pub struct SomeMoreStuff(u32);

#[derive(Serialize)]
pub struct GenTest<T, U>
where
    T: Serializer,
    U: Serializer,
{
    val: T,
    val_vec: Vec<U>,
    norm: i32,
}

#[derive(Serialize)]
pub struct D<T, U>
where
    T: Serializer,
    U: Serializer,
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
        g,
    };

    println!("{}", d.to_str());
}
