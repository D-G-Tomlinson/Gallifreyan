
use std::convert::TryFrom;
use crate::tree::Word;
use crate::draw_shape::WORD_RADIUS;
use crate::shape::Cart;
use crate::draw_shape::draw_word;

pub struct Svg(String);
impl Svg {
    pub fn svg(self) -> String { self.0 }
}

#[derive(Debug,Clone)]
enum WordTypes {
    PlainWord(Vec<char>),
    Punctuation(Vec<char>),
    Number(Vec<char>),
    NumberDec(Vec<char>),
}
use crate::conversion::WordTypes::*;

fn get_words(input:Vec<char>) -> Vec<WordTypes> {
    let mut words:Vec<WordTypes> = Vec::new();
    let mut current_word = None;
    for i in 0..input.len() {
        let c = &input[i];
        match &c {
            'a'..'z' => {
                if let Some(PlainWord(ref mut word)) = current_word {
                    word.push(*c);
                } else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(PlainWord(vec![*c]));
                }
            }
            ' ' => {
                if let Some(PlainWord(ref word)) = current_word {
                    words.push(PlainWord(word.clone()));
                    current_word = None;
                } else if let Some(Number(ref word)) = current_word {
                    words.push(Number(word.clone()));
                    current_word = None;
                } else if let Some(NumberDec(ref word)) = current_word {
                    words.push(NumberDec(word.clone()));
                    current_word = None;
                }
            },
            // other punctuation goes here later
            _ => {
                if let Some(Punctuation(ref mut ps)) = current_word {
                    ps.push(*c);
                } else {
                    words.push(Punctuation(vec![*c]));
                }
            }//good enough for now, will fix later for numbers and punctuation.
        }
    }
    return words;
}

impl TryFrom<String> for Svg {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.trim().to_lowercase().chars().collect::<Vec<char>>();
        if input.is_empty() {
            return Ok(Svg(value));
        }

        let input:Vec<char> = input.into_iter().collect();

        let words = get_words(input);

        let word:Word = Word::try_from(input)?;

        //need pos w.r.t sentance circle, so origin for now
        let centre = Cart::origin();
        let mut shapes =draw_word(word,centre);

        let length = 3.2 * WORD_RADIUS;

        let half_length = Cart::new(length/2.0,length/2.0);
        shapes.iter_mut().for_each(|s| s.shove(half_length));
        let els:Vec<String> = shapes.into_iter().map(|shape| shape.to_element()).collect();


        let mut start =
            format!("<svg
  width=\"500mm\"
  height=\"200mm\"
  viewBox=\"0 0 {} {}\"
  version=\"1.1\"
  xmlns=\"https://github.com/D-G-Tomlinson/Gallifreyan\"> ",length,length);

        for el in &els {
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