use std::convert::From;
use std::boxed::Box;
use wasm_bindgen::link_to;
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
pub enum Thickness {
    Thin,
    Normal,
    Thick,
}
impl Thickness {
    pub fn val(&self) -> f64 {
        match self {
            Thin => WORD_RADIUS/100.0,
            Normal => WORD_RADIUS/50.0,
            Thick => WORD_RADIUS/25.0
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Polar {
    radius:f64,
    theta:f64,//anticlockwise from x-axis, as usual
}
impl Polar {
    pub fn new(radius:f64, theta:f64) -> Self {
        Self { radius, theta }
    }
    pub fn rotate(&self, dt:f64) -> Self {
        Self::new(self.radius, self.theta + dt)
    }

    pub fn extend(&self, dr:f64) -> Self {
        Self::new(self.radius+dr, self.theta)
    }
}
#[derive(Debug,Clone,Copy)]
pub struct Cart {
    x:f64,
    y:f64,
}
impl Cart {
    pub fn shove(&mut self,dx:f64,dy:f64) {
        self.x+=dx;
        self.y+=dy;
    }
    pub fn distance(&self, &other:&Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = dx*dx + dy*dy;
        return distance.sqrt();
    }
}
impl From<Polar> for Cart {
    fn from(polar: Polar) -> Self {
        let x = polar.radius * polar.theta.cos();
        let y = -polar.radius * polar.theta.sin();
        return Self {x, y,};
    }
}
pub struct Circle {
    centre: Cart,
    radius:f64,
    thickness:Option<Thickness>, // no thickness indicates fill
}

impl Circle {
    pub fn new(centre: Cart, radius:f64, thickness:Option<Thickness>) -> Self {
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
        return format!("<circle fill=\"{}\" stroke = \"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\" stroke-width=\"{}\" fill-opacity=\"{}\" />",
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
pub struct Arc {
    start: Cart,
    end: Cart,
    radius:f64,
    large:bool,//do we take the long way round
    clockwise:bool,
    thickness:Thickness,
}

impl Arc {
    pub fn new(start:Cart,end:Cart, radius:f64, large:bool,clockwise:bool, thickness:Thickness) -> Self {
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
pub
struct Line {
    start: Cart,
    end: Cart,
    thickness:Thickness,
}
impl Line {
    pub fn new(start:Cart, end:Cart, thickness:Thickness) -> Self {
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