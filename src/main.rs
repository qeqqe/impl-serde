#![allow(dead_code)]

use impl_serde::Serialize;

trait Serializer {
    fn to_str(&self) -> String;
}

#[derive(Serialize)]
pub struct Points {
    x: i64,
    y: i64,
    z: i64,
    dim: String,
}

fn main() {
    let np = Points {
        x: 1,
        y: 2,
        z: 5,
        dim: "3d".into(),
    };

    let mut st = String::new();
    st.push_str(&format!("{}", np.x));
    println!("{}", np.to_str());
}
