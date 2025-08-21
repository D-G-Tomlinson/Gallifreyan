use std::convert::TryFrom;
use std::str::Chars;
use std::collections::HashMap;



use crate::conversion::Svg;
use crate::tree::Arc::{Above, Big, On};
use crate::tree::Letter::COpt;

#[derive(Debug,Copy, Clone)]
enum Arc{
    Big,
    Above,
    Small,
    On
}
#[derive(Debug,Copy, Clone)]
enum Marks {
    Blank,
    Dot(u8),
    Line(u8),
}
#[derive(Debug,Copy, Clone)]
enum Vowels {
    A,
    E,
    I,
    O,
    U
}
#[derive(Debug,Clone)]
struct Vowel {
    v:Vowels,
    double:bool,
}
fn get_v(letter:Vowels,double:bool) -> Letter {
    return Letter::VOpt(Vowel{v:letter, double});
}
#[derive(Debug,Clone)]
struct Consonant {
    arc: Arc,
    marks: Marks,
    diacritic: Option<Vowel>,
}
fn get_c(arc: Arc,marks: Marks) -> Letter {
    let diacritic = None;
    return COpt(Consonant {arc,marks,diacritic});
}
#[derive(Debug,Clone)]
enum Letter {
    COpt(Consonant),
    VOpt(Vowel),
}

fn chars_to_letters(chars:Vec<char>) -> Result<Vec<Letter>,&'static str> {
    let singles:HashMap<char,Letter> = HashMap::from([
        ('a',get_v(Vowels::A,false)),
        ('e',get_v(Vowels::E,false)),
        ('i',get_v(Vowels::I,false)),
        ('o',get_v(Vowels::O,false)),
        ('u',get_v(Vowels::U,false)),
        ('b',get_c(Big,Marks::Blank)),
        ('d',get_c(Big,Marks::Dot(3))),
        ('g',get_c(Big,Marks::Line(1))),
        ('h',get_c(Big,Marks::Line(2))),
        ('f',get_c(Big,Marks::Line(3))),
        ('j',get_c(Above,Marks::Blank)),
        ('k',get_c(Above,Marks::Dot(2))),
        ('l',get_c(Above,Marks::Dot(3))),
        ('c',get_c(Above,Marks::Dot(4))),
        ('n',get_c(Above,Marks::Line(1))),
        ('p',get_c(Above,Marks::Line(2))),
        ('m',get_c(Above,Marks::Line(3))),
        ('t',get_c(Arc::Small,Marks::Blank)),
        ('r',get_c(Arc::Small,Marks::Dot(3))),
        ('v',get_c(Arc::Small,Marks::Line(1))),
        ('w',get_c(Arc::Small,Marks::Line(2))),
        ('s',get_c(Arc::Small,Marks::Line(3))),
        ('y',get_c(On,Marks::Dot(2))),
        ('z',get_c(On,Marks::Dot(3))),
        ('q',get_c(On,Marks::Dot(4))),
        ('x',get_c(On,Marks::Line(2))),
    ]);
    let doubles:HashMap<(char,char),Letter> = HashMap::from([
        (('c','h'),get_c(Big,Marks::Dot(2))),
        (('n','d'),get_c(Big,Marks::Dot(4))),
        (('p','h'),get_c(Above,Marks::Dot(1))),
        (('w','h'),get_c(Arc::Small,Marks::Dot(1))),
        (('s','h'),get_c(Arc::Small,Marks::Dot(2))),
        (('n','t'),get_c(Arc::Small,Marks::Dot(4))),
        (('t','h'),get_c(Arc::On,Marks::Blank)),
        (('g','h'),get_c(Arc::On,Marks::Dot(1))),
        (('q','u'),get_c(Arc::On,Marks::Line(1))),
        (('n','g'),get_c(Arc::On,Marks::Line(3))),
        (('a','a'),get_v(Vowels::A,true)),
        (('e','e'),get_v(Vowels::E,true)),
        (('i','i'),get_v(Vowels::I,true)),
        (('o','o'),get_v(Vowels::O,true)),
        (('u','u'),get_v(Vowels::U,true)),
    ]);
    let mut result:Vec<Letter> = Vec::new();
    let mut i = 0;
    while i < chars.len()-1 {
        let next = chars[i];
        let next_two :(char,char) = (chars[i],chars[i+1]);
        if doubles.contains_key(&next_two) {
            result.push(doubles[&next_two].clone());
            i += 2;
        } else if singles.contains_key(&next) {
            result.push(singles[&next].clone());
            i += 1;
        } else {
            return Err("invalid letter found");
        }
    }
    if singles.contains_key(&chars[i]) {
        result.push(singles[&chars[i]].clone());
    }
    return Ok(result);
}

fn join_cv(letters:Vec<Letter>) -> Vec<Letter> {
    let mut result:Vec<Letter> = Vec::new();

    let mut i = 0;

    while i < letters.len()-1 {
        match &letters[i] {
            COpt(c) => {
                match c.diacritic {
                    Some(_) => result.push(Letter::COpt(c.clone())),
                    None => {
                        match &letters[i+1] {
                            Letter::VOpt(v) => {
                                let new_letter = Letter::COpt(Consonant{arc:c.arc,marks:c.marks,diacritic:Some(v.clone())});
                                result.push(new_letter);
                                i += 1;
                            },
                            _ => result.push(Letter::COpt(c.clone())),
                        }
                    }
                }
            },
            n => result.push(n.clone())
        }
        i += 1;
    }
    result.push(letters[letters.len()-1].clone());
    return result;
}

impl TryFrom<Vec<char>> for Word {
    type Error = &'static str;
    fn try_from(chars:Vec<char>) -> Result<Self, Self::Error> {
        let result = match chars_to_letters(chars) {
            Ok(l) => l,
            Err(e) => return Err(e)
        };
        let result=join_cv(result);
        return Ok(Word(result));
    }
}

#[derive(Debug,Clone)]
pub struct Word(Vec<Letter>);
//can work on sentances later

