use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    println!("The Rust function say() received {}", s);
    let mut r = String::from("");
    let num = s.parse::<i64>().unwrap();
    for x in 0..=num {
        for _ in 0..=x {
            r += &"1"
        }
        r += "\n"
    }
    return r;
}
