use bitcoin_script::define_pushable;
use wasm_bindgen::prelude::*;

pub mod database;

pub mod spending_requirements;

define_pushable!();

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
