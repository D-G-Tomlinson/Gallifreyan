
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

use crate::shape::Thickness::*;

#[derive(Debug,Clone)]
enum InProgress {
    PlainWord(Vec<char>),
    Punctuation(Vec<char>),
    Number(Vec<char>),
    NumberDec(Vec<char>),
    RawLetter(char),
    RawDigit(char),
    Space,
    Dot,
    QMark,
    ExMark,
    DQuote,
    SQuote,
    Dash,
    Comma,
    SColon,
    Colon
}

use crate::conversion::InProgress::*;
impl TryFrom<char> for InProgress {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='z' => Ok(RawLetter(c)),
            '0'..='9' => Ok(RawDigit(c)),
            ' ' => Ok(Space),
            '.' => Ok(Dot),
            '?' => Ok(QMark),
            '!' => Ok(ExMark),
            '"' => Ok(DQuote),
            '\'' => Ok(SQuote),
            '-' => Ok(Dash),
            ',' => Ok(Comma),
            ';' => Ok(SColon),
            ':' => Ok(Colon),
            _ => Err(format!("Invalid character '{}'", c)),
        }
    }
}

fn get_words(input:Vec<char>) -> Result<Vec<WordTypes>,String> {
    let mut in_prog:Vec<InProgress> = Vec::new();
    for c in input {
        in_prog.push(InProgress::try_from(c)?);
    }
    let in_prog = bunch_numbers(&bunch_chars(&in_prog));

    return Err(format!("not implemented"));
}
fn clone_inner(input:Option<&InProgress>) -> Option<InProgress> {
    match input {
        None => None,
        Some(p) => Some(p.clone())
    }
}
fn bunch_chars(input:&Vec<InProgress>) -> Vec<InProgress>{
    let mut result:Vec<InProgress> = Vec::new();
    for i in 0..input.len(){
        match &input[i] {
            RawLetter(c) => {
                if let Some(PlainWord(w)) = result.last_mut() {
                    w.push(*c);
                } else {
                    result.push(PlainWord(vec![*c]));
                }
            }
            n => result.push(n.clone()),
        }
    }
    // now need to handle single quotes
    for i in (1..result.len()-1).rev() {
        if let Dash = result.get_mut(i).unwrap() {
            let next = clone_inner(result.get(i+1));
            if let Some(PlainWord(w2)) = next {
                if let Some(PlainWord(prev)) = result.get_mut(i-1) {
                    prev.push('\'');
                    let mut next_val = w2.clone();
                    prev.append(&mut next_val);
                    result.remove(i);
                    result.remove(i);
                }
            }
        }
    }
    return result;
}

fn bunch_numbers(input:&Vec<InProgress>) -> Vec<InProgress>{
    let mut result:Vec<InProgress> = Vec::new();
    for i in 0..input.len(){
        match &input[i] {
            RawDigit(d) => {
                if let Some(Number(w)) = result.last_mut() {
                    w.push(*d);
                } else {
                    result.push(Number(vec![*d]));
                }
            },
            n => result.push(n.clone()),
        }
    }

    // now need to handle decimal place dot
    for i in (1..result.len()-1).rev() {
        if let Dot = result.get_mut(i).unwrap() {
            let next = clone_inner(result.get(i+1));
            if let Some(Number(w2)) = next {
                if let Some(prev) = result.get_mut(i-1) {
                    if let Number(mut prev_vec) = prev.clone() {
                    prev_vec.push('.');
                        let mut next_val = w2.clone();
                        prev_vec.append(&mut next_val);
                        result.remove(i);
                        result.remove(i);
                        result[i-1] = NumberDec(prev_vec.clone());
                    }
                }
            }
        }
    }

    for i in 0..input.len(){
        match &input[i] {
            Dash => {
                if let Some(Number(w)) = input.get(i+1) {
                    let mut num = vec!['-'];
                    num.append(&mut w.clone());
                    result.push(Number(num));
                } else if let Some(NumberDec(w)) = input.get(i-1) {
                    let mut num = vec!['-'];
                    num.append(&mut w.clone());
                    result.push(Number(num));
                } else {
                    result.push(Dash);
                }
            },
            n => result.push(n.clone()),
        }
    }

    let mut final_vals :Vec<InProgress> = Vec::new();
    for i in result {
        final_vals.push(match i {
            NumberDec(d) => Number(d),
            n => n
        });
    }
    return final_vals;
}

enum WordTypes {
    PlainWord(Vec<char>),
    Punctuation(Vec<char>),
    Number(Vec<char>),
}
fn get_num_words(sentence:&Vec<WordTypes>) -> u32 {
    let mut num_words = 0;
    for w in sentence {
        if let WordTypes::Punctuation(_) = w {
            ()
        } else {
            num_words += 1;
        }
    }
    return num_words;
}

impl TryFrom<String> for Svg {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.trim().to_lowercase().chars().collect::<Vec<char>>();
        if input.is_empty() {
            return Ok(Svg("<svg viewBox=\"0 0 10 10\" version=\"1.1\" xmlns=\"https://github.com/D-G-Tomlinson/Gallifreyan\"></svg>".to_string()));
        }

        let input:Vec<char> = input.into_iter().collect();

        let words = match get_words(input) {
            Ok(w) => w,
            Err(e) => return Err(e)
        };
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
            if let WordTypes::PlainWord(word) = word {
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
  viewBox=\"0 0 {length} {length}\"
  version=\"1.1\"
  xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\"> ");

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