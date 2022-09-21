extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_number(num: i32) -> i32 {
    num
}

#[wasm_bindgen]
pub fn lookup(input: &JsValue, _limit: usize) -> String {
    // Input is vector of vector of vector of numbers - how strokes and their points are represented in JS
    let input: Vec<Vec<Vec<f32>>> = input.into_serde().unwrap();
    // Convert to typed form: vector of strokes
    let strokes: Vec<Stroke> = Vec::with_capacity(input.len());
    serde_json::to_string(&strokes).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stroke {
    pub points: Vec<Point>,
}
