use std::convert::From;
use std::boxed::Box;
use std::f64::consts::PI;
use std::f64::consts::TAU;

pub const WORD_RADIUS:f64 = 10.0;

const FILL:&str="#000000";
const STROKE:&str="#000000";
pub trait Shape {
    fn shove(&mut self, diff:Cart);
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
    pub radius:f64,
    pub theta:f64,//anticlockwise from x-axis, as usual
}
impl Polar {
    pub fn new(radius:f64, theta:f64) -> Self {
        let theta = theta.rem_euclid(TAU);
        Self { radius, theta }
    }
    pub fn rotate(&self, dt:f64) -> Self {
        Self::new(self.radius, self.theta + dt)
    }

    pub fn extend(&self, dr:f64) -> Self {
        Self::new(self.radius+dr, self.theta)
    }
    pub fn mult(&self, other:&Self) -> Self {
        let radius = self.radius * other.radius;
        let theta = self.theta + other.theta;
        Self::new(radius, theta)
    }
    pub fn divide(&self, other:&Self) -> Self {
        let radius = self.radius / other.radius;
        let theta = self.theta - other.theta;
        Self::new(radius, theta)
    }
}
#[derive(Debug,Clone,Copy)]
pub struct Cart {
    pub x:f64,
    pub y:f64,
}
impl Cart {
    pub fn new(x:f64, y:f64) -> Self {
        Self { x, y }
    }
    pub fn shove(&mut self,diff:Cart) {
        self.x+=diff.x;
        self.y+=diff.y;
    }
    pub fn distance(&self, &other:&Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = dx*dx + dy*dy;
        return distance.sqrt();
    }
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn to(&self, other:&Self) -> Self {
        let x = other.x-self.x;
        let y = other.y-self.y;
        return Self {x,y}
    }
}
impl From<Polar> for Cart {
    fn from(polar: Polar) -> Self {
        let x = polar.radius * polar.theta.cos();
        let y = -polar.radius * polar.theta.sin();
        return Self { x, y }
    }
}

impl From<Cart> for Polar {
    fn from(cart: Cart) -> Self {
        let x = cart.x;
        let y = -cart.y;
        let radius = (x*x + y*y).sqrt();
        let theta:f64;
            theta = (y/x).atan();
        let new_theta:f64;
        if x < 0.0 {
            if theta <= 0.0 {
                new_theta = theta + PI;
            } else {
                new_theta = theta - PI;
            }
        } else {
            new_theta = theta;
        }
        return Polar::new(radius, new_theta);
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
    fn shove(&mut self, diff:Cart) {
        self.centre.shove(diff);
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
    fn shove(&mut self, diff:Cart) {
        self.start.shove(diff);
        self.end.shove(diff);
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
    fn shove(&mut self, diff:Cart) {
        self.start.shove(diff);
        self.end.shove(diff);
    }
    fn to_element(&self) -> String {
        let width = self.thickness.val();
        return format!("<path stroke=\"{}\" stroke-width=\"{}\" d=\"M {} {} L {} {}\" />",
                       STROKE,
                       self.thickness.val(),
                       self.start.x,
                       self.start.y,
                       self.end.x,
                       self.end.y,
        )
    }
}