use std::f64::consts::{PI, TAU};
use crate::shape::*;
use crate::shape::Thickness::{Normal, Thick, Thin};
use crate::tree::{Letter, Word, Consonant, Vowel, Marks, Vowels};
use crate::tree::Vowels::{A,E,I,O,U};
use crate::tree::Arc::{Big,Small,Above,On};

const VOWEL_MODIFIER:f64 = 0.1;
const CONSONANT_MODIFIER:f64 = 0.6;
pub const WORD_RADIUS:f64 = 10.0;

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

pub fn draw_word(word: Word,pos:Cart) -> Shapes {
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
        Above => get_above_arc(middle,std_dist,&consonant.marks,&consonant.diacritic),
        Small =>  get_small_arc((start,middle,end),std_dist,&consonant.marks,&consonant.diacritic),
        On => get_on_arc((start,middle,end),std_dist,&consonant.marks,&consonant.diacritic)
    };
    shapes.append(&mut new_shapes);
    return shapes;
}


fn get_big_arc((start,middle,end):(Polar,Polar,Polar),marks: &Option<Marks>,diacritic:&Option<Vowel>) -> Shapes {
    let diff = end.divide(&start).theta/4.0;
    let in_start:Cart = start.rotate(diff).into();
    let in_end:Cart = end.rotate(-diff).into();
    let std_dist = in_start.distance(&in_end);
    let radius = std_dist * CONSONANT_MODIFIER;

    let mut shapes = Shapes::new();
    shapes.push(Box::new(Arc::new(in_start, in_end, radius, true, true, Normal)));
    shapes.push(Box::new(Circle::new(in_start,Normal.val()*0.5,None)));
    shapes.push(Box::new(Circle::new(in_end,Normal.val()*0.5,None)));
    let start_arc = Arc::new(start.into(),in_start.into(),middle.radius,false,false,Normal);
    let end_arc = Arc::new(in_end.into(),end.into(),middle.radius,false,false,Normal);
    shapes.push(Box::new(start_arc));
    shapes.push(Box::new(end_arc));

    let rotation = Polar::new(end.radius,2.0*diff);
    let centre_radius = get_centre_radius(WORD_RADIUS,radius,end.divide(&start).divide(&rotation).theta,true);
    let centre = Polar::new(centre_radius,middle.theta);

    let avoid_centre:bool;
    if let Some(v) = diacritic {
        let c_start:Cart = start.into();
        let v_std_dist = c_start.distance(&end.into());
        let v_pos = get_big_arc_v_pos(middle,radius,v_std_dist,centre);
        shapes.append(&mut draw_vowel(&v,v_pos,v_std_dist));
        avoid_centre = v.v.centre()
    }else {
        avoid_centre = false;
    }
    if let Some(m) = marks {
        let relative_pos = recenter_mark_pos(centre, in_start, in_end);
        shapes.append(&mut add_marks(m,centre.into(),relative_pos,avoid_centre,std_dist))
    }
    return shapes;
}

fn recenter_mark_pos(centre:Polar, start:Cart, end:Cart) -> (Polar, Polar) {
    let centre:Cart = centre.into();
    let start = centre.to(&start);
    let end = centre.to(&end);
    return (start.into(),end.into());
}

fn get_big_arc_v_pos(middle:Polar, inner_radius:f64,v_std_dist:f64,centre:Polar) -> (Polar,Polar,Polar) {
    let outer = middle.extend(v_std_dist*VOWEL_MODIFIER*1.1);
    let middle = centre;
    let inner = centre.extend(-inner_radius);
    return (inner,middle,outer);
}

fn get_centre_radius(r1:f64,r2:f64,alpha:f64,inner:bool) -> f64 {
    let hal = alpha/2.0;
    let shal = hal.sin();
    let sind = r1*shal/r2;
    let mut d = sind.asin();
    if inner {
        d = PI-d;
    }
    let ep = PI - (hal+d);
    let result =  r2 * ep.sin()/shal;
    return result;
}
fn get_above_arc(middle:Polar,std_dist:f64,marks: &Option<Marks>,diacritic:&Option<Vowel>) -> Shapes {
    let radius = std_dist * CONSONANT_MODIFIER*0.5;
    let centre = middle.extend(-radius*1.1).into();
    let mut shapes:Shapes =  vec![Box::new(Circle::new(centre,radius, Some(Normal)))];

    let avoid_centre:bool;
    if let Some(v) = diacritic {
        let outer = middle.extend(std_dist*VOWEL_MODIFIER*1.1);
        let middle = middle.extend(-radius*1.1);
        let inner = middle.extend(-radius);
        let v_pos = (inner,middle,outer);
        shapes.append(&mut draw_vowel(&v,v_pos,std_dist));
        avoid_centre = v.v.centre()
    }else {
        avoid_centre = false;
    }
    if let Some(m) = marks {
        let start = Polar::new(radius,middle.theta);
        let relative_pos = (start.rotate(7f64*TAU/8f64),start.rotate(TAU/8f64));
        shapes.append(&mut add_marks(m,centre,relative_pos,avoid_centre,std_dist*0.75));
    }
    return shapes;
}
fn get_small_arc((start,middle,end):(Polar,Polar,Polar),std_dist:f64,marks: &Option<Marks>,diacritic:&Option<Vowel>) -> Shapes {
    let radius = std_dist * CONSONANT_MODIFIER;
    let mut shapes:Shapes = vec![Box::new(Arc::new(start.into(),end.into(),radius,false,true,Normal))];
    shapes.push(Box::new(Circle::new(start.into(),Normal.val()*0.5,None)));
    shapes.push(Box::new(Circle::new(end.into(),Normal.val()*0.5,None)));

    let outer = middle.extend(std_dist*VOWEL_MODIFIER*1.1);
    let centre_radius = get_centre_radius(WORD_RADIUS,radius,end.divide(&start).theta,false);
    let centre = Polar::new(centre_radius,middle.theta);
    let avoid_centre:bool;
    if let Some(v) = diacritic {
        let inner = centre.extend(-radius);
        let middle = Polar::new((inner.radius+WORD_RADIUS)/2.0,middle.theta);
        shapes.append(&mut draw_vowel(v,(inner,middle,outer),std_dist));
        avoid_centre = v.v.centre();
    } else {
        avoid_centre = false;
    }
    if let Some(m) = marks {
        let relative_pos = recenter_mark_pos(centre, start.into(), end.into());
        shapes.append(&mut add_marks(m,centre.into(),relative_pos,avoid_centre,std_dist))
    }
    return shapes;
}

fn get_on_arc((start,middle,end):(Polar,Polar,Polar),std_dist:f64,marks: &Option<Marks>,diacritic:&Option<Vowel>) -> Shapes {
    let radius = std_dist * CONSONANT_MODIFIER*0.5;
    let mut shapes:Shapes = vec![Box::new(Circle::new(middle.into(),radius, Some(Normal)))];

    let avoid_centre:bool;
    if let Some(v) = diacritic {
        let middle = middle;
        let inner = middle.extend(-radius);
        let outer = middle.extend(radius/2.0);
        shapes.append(&mut draw_vowel(v,(inner,middle,outer),std_dist));
        avoid_centre = v.v.centre();
    } else {
        avoid_centre = false;
    }
    if let Some(m) = marks {
        let r1 = middle.radius;
        let r2 = radius;
        let diff:f64 = (1f64- r2*r2/(2.0*r1*r1)).acos();
        let relative_pos = recenter_mark_pos(middle, middle.rotate(-diff).into(), middle.rotate(diff).into());
        shapes.append(&mut add_marks(m,middle.into(),relative_pos,avoid_centre,std_dist))
    }
    return shapes;
}
const SHOW_ENDS:bool = false;
fn add_marks(marks:&Marks,centre:Cart,(start,end):(Polar,Polar),avoid_centre:bool,std_dist:f64) -> Shapes {//centre co-ord is wrt the word's centre, start and end are wrt centre
    let (num,is_line):(i32,bool) = match marks {
        Marks::Line(n) => (*n,true),
        Marks::Dot(n) => (*n,false),
    };
    let diff_num:f64 = if avoid_centre && (num%2)!=0 {
        num as f64+ 2f64
    } else {
        num as f64+ 1f64
    };
    let diff = end.divide(&start).theta;
    let diff = TAU-diff;
    let diff = diff/diff_num;


    let mut ppos = start;
    let mut shapes:Shapes = Vec::new();
    let mut cpos:Cart;
    if SHOW_ENDS {
        cpos = ppos.into();
        cpos.shove(centre);
        shapes.push(if is_line {
            let mut end :Cart= ppos.extend(std_dist*CONSONANT_MODIFIER*0.3).into();
            end.shove(centre);
            Box::new(Line::new(cpos,end,Thick))
        } else {
            Box::new(Circle::new(cpos,std_dist*CONSONANT_MODIFIER*0.1,Some(Thin)))
        });
    }
    for _ in 0..num as i32 {
        ppos=ppos.rotate(-diff);
        cpos = ppos.into();
        cpos.shove(centre);
        shapes.push(if is_line {
            let mut end :Cart= ppos.extend(std_dist*CONSONANT_MODIFIER*0.3).into();
            end.shove(centre);
            Box::new(Line::new(cpos,end,Thick))
        } else {
            Box::new(Circle::new(cpos,std_dist*CONSONANT_MODIFIER*0.1,None))
        });
    }
    if SHOW_ENDS {
        ppos = ppos.rotate(-diff);
        cpos = ppos.into();
        cpos.shove(centre);
        shapes.push(if is_line {
            let mut end: Cart = ppos.extend(std_dist * CONSONANT_MODIFIER * 0.3).into();
            end.shove(centre);
            Box::new(Line::new(cpos, end, Thick))
        } else {
            Box::new(Circle::new(cpos, std_dist * CONSONANT_MODIFIER*0.1,Some(Thick)))
        });
    }
    return shapes;
}