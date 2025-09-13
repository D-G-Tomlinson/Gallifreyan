
use std::convert::TryFrom;
use std::f64::consts::TAU;
use crate::tree::Word;
use crate::tree::Number;
use crate::shape::WORD_RADIUS;
use crate::shape::{Cart, Circle, Polar, Shapes};
use crate::draw_word::draw_plain_word;
pub struct Svg(String);
impl Svg {
    pub fn svg(self) -> String { self.0 }
}

#[derive(Debug,Clone)]
enum WordTypes {
    PlainWord(Vec<char>),
    Punctuation(Vec<PunctuationTypes>),
    Number(Vec<char>),
}
use crate::conversion::PunctuationTypes::*;
#[derive(Debug,Clone)]
enum PunctuationTypes {
    NEnd(char),//not sentence ender
    SEnd(char),//sentence ender
}

use crate::conversion::WordTypes::*;
use crate::shape::Thickness::*;

fn get_words(input:Vec<char>) -> Result<Vec<WordTypes>,String> {
    let mut words:Vec<WordTypes> = Vec::new();
    let mut current_word = None;
    for i in 0..input.len() {
        let c = &input[i];
        match &c {
            '0'..='9' => {
                if let Some(Number(ref mut word)) = current_word {
                    word.push(*c);
                } else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(Number(vec![*c]));
                }
            },
            '.' => {
                if let Some(Number(ref mut word)) = current_word {
                    if let Some(next) = input.get(i+1) {
                        if ('0'..='9').contains(&next)  {
                            word.push(*c);
                        }
                    }
                } else if let Some(Punctuation(ref mut word)) = current_word {
                    word.push(SEnd(c.clone()));
                }  else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(Number(vec![*c]));
                }
            },
            '-' => {
                if let Some(next) = input.get(i+1) && ('0'..='9').contains(&next) {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(Number(vec![*c]));
                }
                 else if let Some(Punctuation(ref mut word)) = current_word {
                    word.push(NEnd(c.clone()));
                }  else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(Punctuation(vec![NEnd(c.clone())]));
                }
            },
            'a'..='z' => {
                if let Some(PlainWord(ref mut word)) = current_word {
                    word.push(*c);
                } else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(PlainWord(vec![*c]));
                }
            },
            '\'' => {
                if let Some(PlainWord(ref mut word)) = current_word {
                    if let Some(next) = input.get(i+1) && ('a'..='z').contains(&next) {
                            word.push(*c);
                    }
                } else if let Some(Punctuation(ref mut word)) = current_word {
                    word.push(NEnd(c.clone()));
                } else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(Punctuation(vec![NEnd(c.clone())]));
                }
            },
            ' ' => {
                if let Some(Punctuation(_)) = current_word {
                    ()
                } else if let Some(ref cw) = current_word {
                    words.push(cw.clone());
                    current_word = None;
                }
            },
            //normal, non ending punctuation
            '"'|','|';'|':' => {
                if let Some(Punctuation(ref mut word)) = current_word {
                    word.push(NEnd(c.clone()));
                } else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(PlainWord(vec![*c]));
                }
            },
            //normal, ending punctuation
            '?'|'!' => {
                if let Some(Punctuation(ref mut word)) = current_word {
                    word.push(SEnd(c.clone()));
                } else {
                    if let Some(cw) = current_word {
                        words.push(cw);
                    }
                    current_word = Some(PlainWord(vec![*c]));
                }
            },
            _ => return Err(format!("{} is not a valid letter", c)),
        }
    }
    if let Some(cw) = current_word {
        words.push(cw);
    }
    return Ok(words);
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

fn get_inner_outer_diff(num_words:u32) -> (f64,f64,f64) {
    let diff = TAU/num_words as f64;
    let max_word_radius = 1.6*WORD_RADIUS;
    let inner_radius = match num_words {
        0|1 => 0f64,
        2 => max_word_radius,
        n => (2.0*max_word_radius*max_word_radius/(1.0-diff.cos())).sqrt()
    };
    let outer_radius = inner_radius+max_word_radius;
    return (inner_radius, outer_radius, diff);
}
fn draw_word(word: &WordTypes, this:&Polar,last:&Polar,diff:f64) -> Result<(Shapes,Polar),String> {
    match word{
        PlainWord(word) => {
            let word = Word::try_from(word.clone())?;
            let cart_pos:Cart = Cart::from(this.clone());
            let mut these_shapes = draw_plain_word(&word, this);
            these_shapes.iter_mut().for_each(|shape| shape.shove(cart_pos));
            Ok((these_shapes,this.rotate(diff)))
        },
        Number(word) => {
            let num = &Number::try_from(word.clone())?;
            let cart_pos:Cart = Cart::from(this.clone());
            let mut these_shapes:Shapes = num.into();
            these_shapes.iter_mut().for_each(|shape| shape.shove(cart_pos));
            Ok((these_shapes,this.rotate(diff)))
        },
        Punctuation(word) => {
            todo!()
        }
    }
}

impl TryFrom<String> for Svg {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.trim().to_lowercase().chars().collect::<Vec<char>>();
        if input.is_empty() {
            return Ok(Svg("<svg viewBox=\"0 0 10 10\" version=\"1.1\" xmlns=\"https://github.com/D-G-Tomlinson/Gallifreyan\"></svg>".to_string()));
        }

        let words = get_words(input.into_iter().collect())?;
        let num_words = get_num_words(&words);

        println!("Number of words: {}", num_words);
        let (inner_radius ,outer_radius,diff) = get_inner_outer_diff(num_words);

        let mut pos = Polar::new(inner_radius, -TAU/4.0);
        let mut last = pos.clone();

        let mut shapes:Shapes = Vec::new();

        shapes.push(Box::new(Circle::new(Cart::origin(), outer_radius,Some(Thick))));
        shapes.push(Box::new(Circle::new(Cart::origin(), outer_radius-2.0*Thick.val(),Some(Thin))));

        for word in &words {
            let (mut new_shapes,new_next) = draw_word(word,&pos,&last,diff)?;
            shapes.append(&mut new_shapes);
            last = pos;
            pos = new_next;
        }
        let length = outer_radius * 1.1 * 2.0;
        let half_length = Cart::new(length/2.0,length/2.0);
        shapes.iter_mut().for_each(|s| s.shove(half_length));
        let els:Vec<String> = shapes.into_iter().map(|shape| shape.to_element()).collect();


        let mut start =
            format!("<svg
  viewBox=\"0 0 {length} {length}\"
  version=\"1.1\"
  xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\"> <g id=\"all_gall\">");

        for el in &els {
            start.push_str(&el);
        }
        start.push_str("</g></svg>");

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