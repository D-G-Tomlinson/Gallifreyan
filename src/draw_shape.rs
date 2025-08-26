use crate::shape::*;
use crate::shape::Thickness::Normal;
use crate::tree::{Letter, Word, Consonant, Vowel};
use crate::tree::Vowels::{A,E,I,O,U};
fn one_letter_word(letter:&Letter) -> Shapes {
    let pi = std::f64::consts::PI;
    let diff = pi/2.0;
    let start = crate::shape::Polar::new(WORD_RADIUS, -(pi+diff)/2.0);
    let middle = start.rotate(diff/2.0);
    let end = middle.rotate(diff/2.0);

    let mut shapes = Shapes::new();


    let mut letter = draw_letter(letter, (start,middle,end));
    shapes.append(&mut letter);

    let connector = crate::shape::Arc::new(end.into(), start.into(), WORD_RADIUS, true, false, crate::shape::Thickness::Normal);
    shapes.push(Box::new(connector));
    return shapes;

}

pub fn draw_word(word: Word) -> Shapes {
    println!("Drawing word: {:?}", word);
    let num_parts = word.get_num_things();

    if num_parts == 0 {
        return Shapes::new();
    } else if num_parts == 1 {
        return one_letter_word(&word.word()[0]);
    }

    let pi = std::f64::consts::PI;
    let each = pi/num_parts as f64;
    let mut start = crate::shape::Polar::new(WORD_RADIUS, -(pi+each)/2.0);

    let mut result = Shapes::new();

    for l in word.word() {
        let middle = start.rotate(each/2.0);
        let end = middle.rotate(each/2.0);

        draw_letter(l,(start, middle, end));

        let next = end.rotate(each);
        let connecting_arc = Box::new(crate::shape::Arc::new(end.into(), next.into(), WORD_RADIUS, false, false, crate::shape::Thickness::Normal));
        result.push(connecting_arc);
        start = next;
    }
    return result;
}


fn draw_letter(letter:&Letter, (start,middle,end):(Polar,Polar,Polar)) -> Shapes {
    println!("Drawing letter: {:#?}",letter);
    let std_dist = Cart::from(start).distance(&Cart::from(end));
    if let Letter::VOpt(v) = letter {
        return draw_loose_vowel(v,(start,middle,end),std_dist);
    }
    let mut shapes = Shapes::new();


    return shapes;
}

fn draw_loose_vowel(vowel:&Vowel, (start,middle,end):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let mut shapes = Shapes::new();
    let connecting_arc = Box::new(crate::shape::Arc::new(start.into(), end.into(), WORD_RADIUS, false, false, crate::shape::Thickness::Normal));
    shapes.push(connecting_arc);

    let inner = middle.extend(-std_dist*0.06);
    let outer = middle.extend(std_dist*0.06);
    shapes.append(&mut draw_vowel(vowel,(inner,middle,outer),std_dist));
    return shapes;
}

fn draw_vowel(vowel:&Vowel, (inner,middle,outer):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let mut shapes = Shapes::new();


    println!("Inner: {:#?}",inner);
    println!("Middle {:?}", middle);
    println!("Outer {:?}", outer);

    let centre = match vowel.v {
        A => outer,
        E|I|U => middle,
        O => inner,
    };
    println!("Centre {:?}", centre);
    let centre:Cart = centre.into();
    println!("Centre {:?}", centre);
    let radius = std_dist * 0.05;
    let circle = Circle::new(centre,radius,Some(Normal));
    shapes.push(Box::new(circle));
    if vowel.double {
        let other_circle = Circle::new(centre,radius/2.0,None);
        shapes.push(Box::new(other_circle));
    }
    return shapes;

}