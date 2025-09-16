use std::f64::consts::TAU;
use crate::shape::{Polar, Cart, Circle, Line, Thickness, Shapes, Shape, ShapeSet, BShape};
use crate::shape::Thickness::*;
use crate::tree::{Number, Word};
use crate::tree::Digit;

impl From<(&Digit, f64,f64,bool,f64)> for BShape {
    fn from((digit,outer,inner,is_clockwise,word_radius): (&Digit, f64,f64,bool,f64)) -> BShape {
        let outer_ring_thickness = if digit.follows_dot {ExtraThick} else {Normal};
        let mut shapes:Shapes =vec![Box::new(Circle::new(Cart::origin(),outer,Some(outer_ring_thickness.val(word_radius))))];

        let mut current_pos = Polar::new((outer+inner)/2.0,-TAU/4f64);
        let half_dist = (outer-inner)/2.0;
        let diff:f64;
        if digit.has_circle {
            let circle = Circle::new(current_pos.into(),half_dist,None);
            shapes.push(Box::new(circle));
            diff = TAU/(digit.num_lines+1) as f64;
            current_pos = current_pos.rotate(diff);
        } else {
            diff = TAU/digit.num_lines as f64;
        }
        for _ in 0..digit.num_lines {
            let start:Cart = current_pos.extend(-half_dist).into();
            let end:Cart = current_pos.extend(half_dist).into();
            let line:Line = Line::new(start,end,Normal.val(word_radius),false);
            shapes.push(Box::new(line));
            current_pos = current_pos.rotate(diff);
        }
        return Box::new(ShapeSet::new_rotating_class(shapes, is_clockwise, "digit"));
    }
}

fn get_centre(is_whole:&bool,is_positive:&bool,current_inner:f64,is_clockwise:bool,word_radius:f64) -> BShape {
    match (is_whole,is_positive) {
        (true,true) => Box::new(Circle::new(Cart::origin(),current_inner,None)),
        (true,false) => {
            let circle = Circle::new(Cart::origin(),current_inner,Some(ExtraThick.val(word_radius)));
            let line = Line::new(Cart::new(0.0,current_inner),Cart::new(0.0,-current_inner),ExtraThick.val(word_radius),false);
            Box::new(ShapeSet::new_rotating(vec![Box::new(circle), Box::new(line)], is_clockwise))
        },
        (false,true) => Box::new(Circle::new(Cart::origin(),current_inner,Some(Normal.val(word_radius)))),
        (false,false) => {
            let circle = Circle::new(Cart::origin(),current_inner,Some(Normal.val(word_radius)));
            let line = Line::new(Cart::new(0.0,current_inner),Cart::new(0.0,-current_inner),Normal.val(word_radius),false);
            Box::new(ShapeSet::new_rotating(vec![Box::new(circle), Box::new(line)], is_clockwise))
        },
    }
}

impl From<(&Number,f64)> for BShape {
    fn from((number,word_radius):(&Number,f64)) -> Self {
        println!("Number: {:?}", number);
        let mut shapes:Shapes = Shapes::new();
        let mut is_clockwise = false;
        let delta_rad = word_radius/(number.digits.len() as f64 + 1f64);
        let mut current_inner = word_radius;
        let mut current_outer = word_radius;
        for d in &number.digits {
            current_outer = current_inner;
            current_inner = current_inner - delta_rad;
            let digit_shapes = (d,current_outer, current_inner, is_clockwise,word_radius).into();
            shapes.push(digit_shapes);
            is_clockwise = !is_clockwise;
        }

        shapes.push(get_centre(&number.is_whole,&number.is_positive,current_inner,is_clockwise,word_radius));
        return Box::new(ShapeSet::new(shapes,"word number"));
    }
}