use std::f64::consts::TAU;
use crate::shape::{Polar, Cart, Circle, Line, Thickness, Shapes, WORD_RADIUS, Shape, ShapeSet, BShape};
use crate::shape::Thickness::*;
use crate::tree::{Number, Word};
use crate::tree::Digit;

impl Digit {
    fn to_shapes(&self, outer:f64,inner:f64,is_clockwise:bool) -> BShape {
        let outer_ring_thickness = if self.follows_dot {ExtraThick} else {Normal};
        let mut shapes:Shapes =vec![Box::new(Circle::new(Cart::origin(),outer,Some(outer_ring_thickness)))];

        let mut current_pos = Polar::new((outer+inner)/2.0,-TAU/4f64);
        let half_dist = (outer-inner)/2.0;
        let diff:f64;
        if self.has_circle {
            let circle = Circle::new(current_pos.into(),half_dist,None);
            shapes.push(Box::new(circle));
            diff = TAU/(self.num_lines+1) as f64;
            current_pos = current_pos.rotate(diff);
        } else {
            diff = TAU/self.num_lines as f64;
        }
        for _ in 0..self.num_lines {
            let start:Cart = current_pos.extend(-half_dist).into();
            let end:Cart = current_pos.extend(half_dist).into();
            let line:Line = Line::new(start,end,Normal);
            shapes.push(Box::new(line));
            current_pos = current_pos.rotate(diff);
        }
        return Box::new(ShapeSet::new_rotating_class(shapes, is_clockwise, "digit"));
    }
}

fn get_centre(is_whole:&bool,is_positive:&bool,current_inner:f64,is_clockwise:bool) -> BShape {
    match (is_whole,is_positive) {
        (true,true) => Box::new(Circle::new(Cart::origin(),current_inner,None)),
        (true,false) => {
            let circle = Circle::new(Cart::origin(),current_inner,Some(ExtraThick));
            let line = Line::new(Cart::new(0.0,current_inner),Cart::new(0.0,-current_inner),ExtraThick);
            Box::new(ShapeSet::new_rotating(vec![Box::new(circle), Box::new(line)], is_clockwise))
        },
        (false,true) => Box::new(Circle::new(Cart::origin(),current_inner,Some(Normal))),
        (false,false) => {
            let circle = Circle::new(Cart::origin(),current_inner,Some(Normal));
            let line = Line::new(Cart::new(0.0,current_inner),Cart::new(0.0,-current_inner),Normal);
            Box::new(ShapeSet::new_rotating(vec![Box::new(circle), Box::new(line)], is_clockwise))
        },
    }
}

impl From<&Number> for BShape {
    fn from(number:&Number) -> Self {
        println!("Number: {:?}", number);
        let mut shapes:Shapes = Shapes::new();
        let mut is_clockwise = false;
        let delta_rad = WORD_RADIUS/(number.digits.len() as f64 + 1f64);
        let mut current_inner = WORD_RADIUS;
        let mut current_outer = WORD_RADIUS;
        for d in &number.digits {
            current_outer = current_inner;
            current_inner = current_inner - delta_rad;
            let digit_shapes = d.to_shapes(current_outer, current_inner, is_clockwise);
            shapes.push(digit_shapes);
            is_clockwise = !is_clockwise;
        }

        shapes.push(get_centre(&number.is_whole,&number.is_positive,current_inner,is_clockwise));
        return Box::new(ShapeSet::new(shapes,"word number"));
    }
}