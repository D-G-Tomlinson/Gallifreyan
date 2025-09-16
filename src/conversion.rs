
use std::convert::TryFrom;
use std::f64::consts::TAU;
use crate::tree::{Sentence, Word};
use crate::tree::Number;
use crate::shape::{BShape, SENTENCE_RADIUS};
use crate::shape::{Cart, Circle, Polar, Shapes};
use crate::draw_word::draw_plain_word;
pub struct Svg(String);
impl Svg {
    pub fn svg(self) -> String { self.0 }
}



impl TryFrom<String> for Svg {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.trim().to_lowercase().chars().collect::<Vec<char>>();
        if input.is_empty() {
            return Ok(Svg("<svg id=\"generated_svg\" viewBox=\"0 0 10 10\" version=\"1.1\" xmlns=\"https://github.com/D-G-Tomlinson/Gallifreyan\"></svg>".to_string()));
        }

        let sentence = &Sentence::try_from(input.into_iter().collect::<Vec<char>>())?;
        let sen_rad=SENTENCE_RADIUS;
        let mut sentence:BShape = (sentence,sen_rad).try_into()?;
        let length = sen_rad * 2.0 * 1.1;
        let half_length = Cart::new(length/2.0,length/2.0);
        sentence.shove(half_length);
        let els = sentence.to_element();


        let mut start =
            format!("<svg
  viewBox=\"0 0 {length} {length}\"
  version=\"1.1\"
  xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\"
  id=\"generated_svg\">
    <g id=\"all_gall\">
{els}
    </g>
</svg>");

        let result = Svg(start);
        return Ok(result);
    }
}
pub fn get_image(text: &str) -> String {
    return match Svg::try_from(text.to_string()) {
        Ok(image) => image.svg(),
        Err(error) => format!("<p>{}</p>",error.to_string())
    }
}
