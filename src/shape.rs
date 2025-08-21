use std::convert::From;
use std::boxed::Box;

use crate::tree::Word;
use crate::tree::Letter;

pub const WORD_RADIUS:f64 = 10.0;

const FILL:&str="#000000";
const STROKE:&str="#000000";
pub trait Shape {
    fn shove(&mut self, dx:f64,dy:f64);
    fn to_element(&self) -> String;
}
pub type Shapes = Vec<Box<dyn Shape>>;

use crate::shape::Thickness::{Thin,Normal,Thick};
enum Thickness {
    Thin,
    Normal,
    Thick,
}
impl Thickness {
    fn val(&self) -> f64 {
        match self {
            Thin => 0.1,
            Normal => 0.2,
            Thick => 0.4,
        }
    }
}

#[derive(Debug,Clone,Copy)]
struct Polar {
    radius:f64,
    theta:f64,//anticlockwise from x-axis, as usual
}
impl Polar {
    fn new(radius:f64, theta:f64) -> Self {
        Self { radius, theta }
    }
    fn rotate(&self, dt:f64) -> Self {
        Self::new(self.radius, self.theta + dt)
    }

    fn extend(&self, dr:f64) -> Self {
        Self::new(self.radius+dr, self.theta)
    }
}
#[derive(Debug,Clone,Copy)]
struct Cart {
    x:f64,
    y:f64,
}
impl Cart {
    fn shove(&mut self,dx:f64,dy:f64) {
        self.x+=dx;
        self.y+=dy;
    }
}
impl From<Polar> for Cart {
    fn from(polar: Polar) -> Self {
        let x = polar.radius * polar.theta.cos();
        let y = -polar.radius * polar.theta.sin();
        return Self {x, y,};
    }
}
struct Circle {
    centre: Cart,
    radius:f64,
    thickness:Option<Thickness>, // no thickness indicates fill
}

impl Circle {
    fn new(centre: Cart, radius:f64, thickness:Option<Thickness>) -> Self {
        Self { centre, radius, thickness }
    }
}

impl Shape for Circle {
    fn shove(&mut self, dx:f64,dy:f64) {
        self.centre.shove(dx,dy);
    }
    fn to_element(&self) -> String {
        let (opacity,width) = match &self.thickness {
            Some(t) => (0.0,t.val()),
            None => (1.0,0.0)
        };
        return format!("<circle fill=\"{}\" stroke = \"{}\" cx=\"{}\" cy\"{}\" r=\"{}\" stroke-width=\"{}\" fill-opacity=\"{}\" />",
                       FILL,
                       STROKE,
                       self.centre.x,
                       self.centre.y,
                       self.radius,
                       width,
                       opacity
        )
    }
}
struct Arc {
    start: Cart,
    end: Cart,
    radius:f64,
    large:bool,//do we take the long way round
    clockwise:bool,
    thickness:Thickness,
}

impl Arc {
    fn new(start:Cart,end:Cart, radius:f64, large:bool,clockwise:bool, thickness:Thickness) -> Self {
        Self {start, end, radius, large,clockwise, thickness}
    }
}

impl Shape for Arc {
    fn shove(&mut self, dx:f64,dy:f64) {
        self.start.shove(dx,dy);
        self.end.shove(dx,dy);
    }
    fn to_element(&self) -> String {
        let width = self.thickness.val();
        let large = match self.large {
            true => 1,
            false => 0
        };
        let clockwise = match self.clockwise {
            true => 1,
            false => 0
        };
        return format!("<path fill-opacity=\"0\" stroke=\"{}\" stroke-width=\"{}\" d=\"M {} {} A {} {} 0 {} {} {} {}\" />",
            STROKE,
            self.thickness.val(),
            self.start.x,
            self.start.y,
            self.radius,
            self.radius,
            large,
            clockwise,
            self.end.x,
            self.end.y,
        )
    }
}

struct Line {
    start: Cart,
    end: Cart,
    thickness:Thickness,
}
impl Line {
    fn new(start:Cart, end:Cart, thickness:Thickness) -> Self {
        Self {start, end, thickness}
    }
}
impl Shape for Line {
    fn shove(&mut self, dx:f64,dy:f64) {
        self.start.shove(dx,dy);
        self.end.shove(dx,dy);
    }
    fn to_element(&self) -> String {
        let width = self.thickness.val();
        return format!("<path stroke=\"{}\" stroke-width=\"{}\" M {} {} L {} {} />",
                       STROKE,
                       self.thickness.val(),
                       self.start.x,
                       self.start.y,
                       self.end.x,
                       self.end.y,
        )
    }
}
fn one_letter_word(letter:&Letter) -> Shapes {
    let pi = std::f64::consts::PI;
    let diff = pi/2.0;
    let start = Polar::new(WORD_RADIUS,-(pi+diff)/2.0);
    let mid = start.rotate(diff/2.0);
    let end = mid.rotate(diff/2.0);

    let mut shapes = Shapes::new();
    //get letter shapes
    let connector = Arc::new(end.into(),start.into(),WORD_RADIUS,true,false,Normal);
    shapes.push(Box::new(connector));
    return shapes;

}

impl From<Word> for Shapes {
    fn from(word:Word) -> Self {
        let num_parts = word.get_num_things();

        if num_parts == 0 {
            return Self::new();
        } else if num_parts == 1 {
            return one_letter_word(&word.word()[0]);
        }

        let pi = std::f64::consts::PI;
        let each = pi/num_parts as f64;
        let mut start = Polar::new(WORD_RADIUS,-(pi+each)/2.0);

        let mut result = Self::new();

        for _ in 0..num_parts {
            let mid = start.rotate(each/2.0);
            let end = mid.rotate(each/2.0);

            // get the shapes for the consonant or vowel

            let next = end.rotate(each);
            let connecting_arc = Box::new(Arc::new(end.into(),next.into(),WORD_RADIUS,false,false,Normal));
            result.push(connecting_arc);
            start = next;
        }
        return result;
        }
}
