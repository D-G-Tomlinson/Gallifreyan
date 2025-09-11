
use std::convert::TryFrom;
use std::f64::consts::TAU;
use crate::tree::Word;
use crate::draw_shape::WORD_RADIUS;
use crate::shape::{Cart, Circle, Polar, Shapes};
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
use crate::shape::Thickness::*;

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
    if let Some(cw) = current_word {
        words.push(cw);
    }
    return words;
}

fn get_num_words(sentence:&Vec<WordTypes>) -> u32 {
    let mut num_words = 0;
    for w in sentence {
        if let Punctuation(_) = w {
            ()
        } else {
            num_words += 1;
        }
    }
    return num_words;
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
        let num_words = get_num_words(&words);

        println!("Number of words: {}", num_words);

        let diff = TAU/num_words as f64;
        let max_word_radius = 1.6*WORD_RADIUS;
        let inner_radius = match num_words {
            0|1 => 0f64,
            2 => max_word_radius,
            n => (2.0*max_word_radius*max_word_radius/(1.0-diff.cos())).sqrt()
        };
        let outer_radius = inner_radius+max_word_radius;
        let length = outer_radius * 1.1 * 2.0;

        let mut pos = Polar::new(inner_radius, -TAU/4.0);

        let mut shapes:Shapes = Vec::new();

        let sentence_ring = Circle::new(Cart::origin(), outer_radius,Some(Thick));
        let punctuation_ring = Circle::new(Cart::origin(), outer_radius-2.0*Thick.val(),Some(Thin));

        shapes.push(Box::new(sentence_ring));
        shapes.push(Box::new(punctuation_ring));

        for word in &words {
            if let PlainWord(word) = word {
                let word = Word::try_from(word.clone())?;
                let cart_pos:Cart = Cart::from(pos);
                let mut these_shapes = draw_word(word,cart_pos);
                these_shapes.iter_mut().for_each(|shape| shape.shove(cart_pos));
                shapes.append(&mut these_shapes);
                pos = pos.rotate(diff);
            }
        }

        let half_length = Cart::new(length/2.0,length/2.0);
        shapes.iter_mut().for_each(|s| s.shove(half_length));
        let els:Vec<String> = shapes.into_iter().map(|shape| shape.to_element()).collect();


        let mut start =
            format!("<svg
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