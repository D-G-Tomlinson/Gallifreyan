mod tree;
mod conversion;
mod shape;
mod draw_word;
mod draw_number;
mod draw_sentence;

use conversion::get_image;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_gallifreyan(text: &str) -> String {
    return get_image(text);
}
