
use std::convert::TryFrom;
use crate::tree;
use crate::tree::Word;
use crate::shape::Shapes;
use crate::shape::WORD_RADIUS;

pub struct Svg(String);
impl Svg {
    pub fn svg(self) -> String { self.0 }
}
impl TryFrom<String> for Svg {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.trim().to_lowercase().chars().collect::<Vec<char>>();
        if input.clone().into_iter().any(|c| !c.is_ascii_lowercase()) {
            return Err("only letters for now");
        }
        let input:Vec<char> = input.into_iter().collect();
        let word:Word = Word::try_from(input)?;
        let mut shapes = Shapes::from(word.clone());

        let diff = 1.1 * WORD_RADIUS;
        
        for shape in &mut shapes {
            shape.shove(diff,diff);
        }

        let els:Vec<String> = shapes.into_iter().map(|shape| shape.to_element()).collect();

        let length = 2.4*WORD_RADIUS;

        let mut start =
            format!("<svg
  width=\"100mm\"
  height=\"100mm\"
  viewBox=\"0 0 {} {}\"
  version=\"1.1\"
  xmlns=\"https://github.com/D-G-Tomlinson/Gallifreyan\"> ",length,length);

        for el in els {
            start.push_str(&el);
        }
        start.push_str("</svg>");

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