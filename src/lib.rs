mod tree;
mod conversion;
use conversion::get_image;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_gallifreyan(text: &str) -> String {
    return get_image(text);
}
