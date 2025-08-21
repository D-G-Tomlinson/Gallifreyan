mod tree;
mod conversion;
mod shape;

use conversion::get_image;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_gallifreyan(text: &str) -> String {
    return get_image(text);
}
