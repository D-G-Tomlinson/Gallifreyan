use std::convert::TryFrom;
use std::str::Chars;
use std::collections::HashMap;



use crate::conversion::Svg;
use crate::tree::Arc::{Above, Big};

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
enum Vowel {
    A,
    E,
    I,
    O,
    U
}
#[derive(Debug,Copy, Clone)]
enum Diacritic {
    None,
    One(Vowel),
    Double(Vowel)
}
#[derive(Debug,Copy, Clone)]
struct Consonant {
    arc: Arc,
    marks: Marks,
    diacritic: Diacritic,
}
fn get_c(arc: Arc,marks: Marks) -> Letter {
    let diacritic = Diacritic::None;
    return Letter::Consonant(Consonant{arc,marks,diacritic});
}
#[derive(Debug,Copy, Clone)]
enum Letter {
    Consonant(Consonant),
    Vowel(Vowel),
    Apostrophe,
}

impl TryFrom<Vec<char>> for Word {
    type Error = &'static str;
    fn try_from(chars:Vec<char>) -> Result<Self, Self::Error> {
        let singles:HashMap<char,Letter> = HashMap::from([
            ('a',Letter::Vowel(Vowel::A)),
            ('e',Letter::Vowel(Vowel::E)),
            ('i',Letter::Vowel(Vowel::I)),
            ('o',Letter::Vowel(Vowel::O)),
            ('u',Letter::Vowel(Vowel::U)),
        ]);
        let doubles:HashMap<(char,char),Letter> = HashMap::from([
            (('c','h'),get_c(Big,Marks::Dot(2))),
            (('n','d'),get_c(Big,Marks::Dot(4))),
            (('p','h'),get_c(Above,Marks::Dot(1))),
            (('w','h'),get_c(Arc::Small,Marks::Dot(1))),
            (('s','h'),get_c(Arc::Small,Marks::Dot(2))),
            (('n','t'),get_c(Arc::Small,Marks::Dot(4))),

        ]);
        let mut result:Vec<Letter> = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            let next = chars[i];
            let next_two :(char,char) = (chars[i],chars[i+1]);
            if singles.contains_key(&chars[i]) {
                result.push(Letter::Vowel(singles[&chars[i]]));
                i += 1;
            } else
        }
        return Ok(Word(result));
    }
}

struct Word(Vec<Letter>);
//can work on sentances later

