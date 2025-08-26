use std::convert::TryFrom;
use std::str::Chars;
use std::collections::HashMap;



use crate::conversion::Svg;
use crate::tree::Arc::{Above, Big, Small, On};
use crate::tree::Letter::{COpt, VOpt};
use crate::tree::Marks::{Blank,Dot,Line};
use crate::tree::Vowels::{A,E,I,O,U};

#[derive(Debug,Copy, Clone)]
pub enum Arc{
    Big,
    Above,
    Small,
    On
}
#[derive(Debug,Copy, Clone)]
pub enum Marks {
    Blank,
    Dot(u8),
    Line(u8),
}
#[derive(Debug,Copy, Clone)]
pub enum Vowels {
    A,
    E,
    I,
    O,
    U
}
#[derive(Debug,Clone)]
pub struct Vowel {
    pub v:Vowels,
    pub double:bool,
}
fn get_v(letter:Vowels,double:bool) -> Letter {
    return Letter::VOpt(Vowel{v:letter, double});
}
#[derive(Debug,Clone)]
pub struct Consonant {
    pub arc: Arc,
    pub marks: Marks,
    pub diacritic: Option<Vowel>,
}
fn get_c(arc: Arc,marks: Marks) -> Letter {
    let diacritic = None;
    return COpt(Consonant {arc,marks,diacritic});
}
#[derive(Debug,Clone)]
pub enum Letter {
    COpt(Consonant),
    VOpt(Vowel),
}

fn chars_to_letters(chars:Vec<char>) -> Result<Vec<Letter>,&'static str> {
    let singles:HashMap<char,Letter> = HashMap::from([
        ('a',get_v(A,false)),
        ('e',get_v(E,false)),
        ('i',get_v(I,false)),
        ('o',get_v(O,false)),
        ('u',get_v(U,false)),
        ('b',get_c(Big,Blank)),
        ('d',get_c(Big,Dot(3))),
        ('g',get_c(Big,Line(1))),
        ('h',get_c(Big,Line(2))),
        ('f',get_c(Big,Line(3))),
        ('j',get_c(Above,Blank)),
        ('k',get_c(Above,Dot(2))),
        ('l',get_c(Above,Dot(3))),
        ('c',get_c(Above,Dot(4))),
        ('n',get_c(Above,Line(1))),
        ('p',get_c(Above,Line(2))),
        ('m',get_c(Above,Line(3))),
        ('t',get_c(Small,Blank)),
        ('r',get_c(Small,Dot(3))),
        ('v',get_c(Small,Line(1))),
        ('w',get_c(Small,Line(2))),
        ('s',get_c(Small,Line(3))),
        ('y',get_c(On,Dot(2))),
        ('z',get_c(On,Dot(3))),
        ('q',get_c(On,Dot(4))),
        ('x',get_c(On,Line(2))),
    ]);
    let doubles:HashMap<(char,char),Letter> = HashMap::from([
        (('c','h'),get_c(Big,Dot(2))),
        (('n','d'),get_c(Big,Dot(4))),
        (('p','h'),get_c(Above,Dot(1))),
        (('w','h'),get_c(Small,Dot(1))),
        (('s','h'),get_c(Small,Dot(2))),
        (('n','t'),get_c(Small,Dot(4))),
        (('t','h'),get_c(On,Blank)),
        (('g','h'),get_c(On,Dot(1))),
        (('q','u'),get_c(On,Line(1))),
        (('n','g'),get_c(On,Line(3))),
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
    if i<chars.len() && singles.contains_key(&chars[i]) {
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

impl Word {
    pub fn word(&self) -> &Vec<Letter> {
        &self.0
    }
}

impl Word {
    pub fn get_num_things(&self) -> u32 {
        let mut i = 0;
        for thing in &self.0 {
            match thing {
                COpt(_) => i += 1,
                VOpt(_) => i += 1,
            }//this will be important for apostrophes
        }
        return i;
    }
}
//can work on sentances later

