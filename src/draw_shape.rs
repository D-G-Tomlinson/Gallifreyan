use crate::shape::*;
use crate::shape::Thickness::{Normal,Thin};
use crate::tree::{Letter, Word, Consonant, Vowel, Marks, Vowels};
use crate::tree::Vowels::{A,E,I,O,U};
use crate::tree::Arc::{Big,Small,Above,On};

const VOWEL_MODIFIER:f64 = 0.2;
const CONSONANT_MODIFIER:f64 = 0.6;

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

        result.append(&mut draw_letter(l,(start, middle, end)));

        let next = end.rotate(each);
        let connecting_arc = Box::new(crate::shape::Arc::new(end.into(), next.into(), WORD_RADIUS, false, false, crate::shape::Thickness::Normal));
        result.push(connecting_arc);
        start = next;
    }
    return result;
}


fn draw_letter(letter:&Letter, (start,middle,end):(Polar,Polar,Polar)) -> Shapes {
    let std_dist = Cart::from(start).distance(&Cart::from(end));
    return match letter {
        Letter::VOpt(v) => draw_loose_vowel(v,(start,middle,end),std_dist),
        Letter::COpt(c) => draw_consonant(c,(start,middle,end),std_dist)
    }
}

fn draw_loose_vowel(vowel:&Vowel, (start,middle,end):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let mut shapes = Shapes::new();
    let connecting_arc = Box::new(crate::shape::Arc::new(start.into(), end.into(), WORD_RADIUS, false, false, crate::shape::Thickness::Normal));
    shapes.push(connecting_arc);

    let inner = middle.extend(-std_dist*VOWEL_MODIFIER*1.01);
    let outer = middle.extend(std_dist*VOWEL_MODIFIER*1.01);
    shapes.append(&mut draw_vowel(vowel,(inner,middle,outer),std_dist));
    return shapes;
}

fn draw_vowel(vowel:&Vowel, (inner,middle,outer):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let mut shapes = Shapes::new();

    let polar_centre = match vowel.v {
        A => outer,
        E|I|U => middle,
        O => inner,
    };
    let cart_centre:Cart = polar_centre.into();
    let radius = std_dist * VOWEL_MODIFIER;
    let circle = Circle::new(cart_centre, radius, Some(Normal));
    shapes.push(Box::new(circle));
    if vowel.double {
        let other_circle = Circle::new(cart_centre, radius/2.0, Some(Thin));
        shapes.push(Box::new(other_circle));
    }
    match vowel.v {
        I => {
            let start:Cart = polar_centre.extend(-radius).into();
            let end = Cart::origin();
            let line = Line::new(start,end,Normal);
            shapes.push(Box::new(line));
        },
        U => {
            let start:Cart = polar_centre.extend(radius).into();
            let end:Cart = Polar::new(WORD_RADIUS*1.3,polar_centre.theta).into();
            let line = Line::new(start,end,Normal);
            shapes.push(Box::new(line));
        },
        _ => ()
    }
    return shapes;
}
fn draw_consonant(consonant: &Consonant, (start,middle,end):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let mut shapes = Shapes::new();
    match consonant.arc {
        Above|On => shapes.push(Box::new(crate::shape::Arc::new(start.into(), end.into(), WORD_RADIUS, false, false, crate::shape::Thickness::Normal))),
        _ => ()
    }

    let mut new_shapes:Shapes= match consonant.arc {
        Big => get_big_arc((start,middle,end),&consonant.marks,&consonant.diacritic),
        Above => get_above_arc((start,middle,end),std_dist),
        Small =>  get_small_arc((start,middle,end),std_dist),
        On => get_on_arc((start,middle,end),std_dist)
    };
    shapes.append(&mut new_shapes);
    return shapes;
}


fn get_big_arc((start,middle,end):(Polar,Polar,Polar),marks: &Marks,diacritic:&Option<Vowel>) -> Shapes {
    let diff = (end.theta - start.theta)/4.0;
    let in_start:Cart = start.rotate(diff).into();
    let in_end:Cart = end.rotate(-diff).into();
    let std_dist = in_start.distance(&in_end);
    let radius = std_dist * CONSONANT_MODIFIER;

    let mut shapes = Shapes::new();
    shapes.push(Box::new(Arc::new(in_start.into(), in_end.into(), radius, true, true, Normal)));
    let start_arc = Arc::new(start.into(),in_start.into(),middle.radius,false,false,Normal);
    let end_arc = Arc::new(in_end.into(),end.into(),middle.radius,false,false,Normal);
    shapes.push(Box::new(start_arc));
    shapes.push(Box::new(end_arc));

    if let Some(v) = diacritic {
        let c_start:Cart = start.into();
        let v_std_dist = c_start.distance(&end.into());
        let v_pos = get_big_arc_v_pos((start,middle,end),radius,end.theta-start.theta-2.0*diff,v_std_dist);
        shapes.append(&mut draw_vowel(&v,v_pos,v_std_dist));
    }
    return shapes;
}

fn get_big_arc_v_pos((start,middle,end):(Polar,Polar,Polar), inner_radius:f64, alpha:f64,v_std_dist:f64) -> (Polar,Polar,Polar) {
    let outer = middle.extend(v_std_dist*VOWEL_MODIFIER*0.99);

    let beta = middle.theta;
    let centre_radius = inner_radius/(alpha/2.0).tan();
    let centre = Polar::new(centre_radius,beta);

    let middle = centre;
    let inner = centre.extend(-inner_radius);
    return (inner,middle,outer);
}
fn get_above_arc((start,middle,end):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let radius = std_dist * CONSONANT_MODIFIER*0.5;
    return vec![Box::new(Circle::new(middle.extend(-radius*1.1).into(),radius, Some(Normal)))];
}
fn get_small_arc((start,middle,end):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let radius = std_dist * CONSONANT_MODIFIER;
    return vec![Box::new(Arc::new(start.into(),end.into(),radius,false,true,Normal))];
}

fn get_on_arc((start,middle,end):(Polar,Polar,Polar),std_dist:f64) -> Shapes {
    let radius = std_dist * CONSONANT_MODIFIER*0.5;
    return vec![Box::new(Circle::new(middle.into(),radius, Some(Normal)))];
}